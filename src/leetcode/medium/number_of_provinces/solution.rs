use std::io::{Error, ErrorKind};

/// Question: https://leetcode.com/problems/number-of-provinces/
#[allow(clippy::redundant_clone)]
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut provinces: Vec<Vec<i32>> = Vec::new();

        for row in 0..(is_connected.len() as i32) {
            for column in row..(is_connected[row as usize].len() as i32) {
                if row == column {
                    if !is_city_exist_in(&provinces, &row) {
                        provinces.push(vec![row]);
                    }

                    continue;
                }

                if is_connected[row as usize][column as usize] == 1 {
                    let city_exist_kind = get_city_exist_kind(&provinces, &row, &column);

                    match city_exist_kind {
                        CityExistKind::BothCitiesExist => {
                            let province_contain_row_city_idx = get_province_index_which_contain(row, &provinces)
                                .expect("city should exist in provinces");
                            let province_contain_column_city_idx =
                                get_province_index_which_contain(column, &provinces)
                                    .expect("city should exist in provinces");

                            if province_contain_row_city_idx == province_contain_column_city_idx {
                                continue;
                            }

                            let mut to_add = provinces[province_contain_column_city_idx].clone();

                            provinces[province_contain_row_city_idx].append(&mut to_add);
                            provinces.remove(province_contain_column_city_idx);
                        }
                        CityExistKind::RowCityExistOnly => {
                            for province in &mut provinces {
                                if province.contains(&row) {
                                    province.push(column);
                                }
                            }
                        }
                        CityExistKind::ColumnCityExistOnly => {
                            for province in &mut provinces {
                                if province.contains(&column) {
                                    province.push(row);
                                }
                            }
                        }
                        CityExistKind::BothCitiesNotExist => {
                            provinces.push(vec![row, column]);
                        }
                    }
                }
            }
        }

        provinces.len() as i32
    }
}

#[allow(clippy::ptr_arg)] // known problems: https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
fn is_city_exist_in(provinces: &Vec<Vec<i32>>, city: &i32) -> bool {
    for province in provinces {
        if province.contains(city) {
            return true;
        }
    }

    false
}

#[allow(clippy::collapsible_else_if)] // To not collapse if condition here looks better than collapse one.
fn get_city_exist_kind(provinces: &[Vec<i32>], city_row: &i32, city_column: &i32) -> CityExistKind {
    if is_city_exist_in(&provinces.to_vec(), city_row) {
        if is_city_exist_in(&provinces.to_vec(), city_column) {
            CityExistKind::BothCitiesExist
        } else {
            CityExistKind::RowCityExistOnly
        }
    } else {
        if is_city_exist_in(&provinces.to_vec(), city_column) {
            CityExistKind::ColumnCityExistOnly
        } else {
            CityExistKind::BothCitiesNotExist
        }
    }
}

fn get_province_index_which_contain(city: i32, provinces: &[Vec<i32>]) -> Result<usize, Error> {
    for (idx, province) in provinces.iter().enumerate() {
        if province.contains(&city) {
            return Ok(idx);
        }
    }

    Err(Error::new(ErrorKind::NotFound, ""))
}

enum CityExistKind {
    RowCityExistOnly,
    ColumnCityExistOnly,
    BothCitiesExist,
    BothCitiesNotExist,
}

struct Solution;

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(
    is_connected,
    expected,
    case(vec ! [vec ! [1, 1, 0], vec ! [1, 1, 0], vec ! [0, 0, 1]], 2),
    case(vec ! [vec ! [1, 0, 0], vec ! [0, 1, 0], vec ! [0, 0, 1]], 3),
    case(vec ! [vec ! [1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], vec ! [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec ! [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec ! [0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0], vec ! [0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0], vec ! [0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0], vec ! [0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0], vec ! [1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0], vec ! [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0], vec ! [0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1], vec ! [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0], vec ! [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0], vec ! [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0], vec ! [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0], vec ! [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]], 3),
    )]
    fn find_circle_num_should_return_number_of_provinces_when_given_matrix_one_and_zero_which_group_of_one_represent_a_province(
        is_connected: Vec<Vec<i32>>,
        expected: i32,
    ) {
        assert_eq!(Solution::find_circle_num(is_connected), expected);
    }
}
