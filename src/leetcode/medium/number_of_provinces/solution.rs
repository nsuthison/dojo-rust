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
                    if is_city_exist_in(&provinces, &row) {
                        if is_city_exist_in(&provinces, &column) {
                            let mut row_city_idx: usize = 0;
                            let mut column_city_idx: usize = 0;

                            for (idx, province) in provinces.iter().enumerate() {
                                if province.contains(&row) {
                                    row_city_idx = idx;
                                }
                            }

                            for (idx, province) in provinces.iter().enumerate() {
                                if province.contains(&column) {
                                    column_city_idx = idx;
                                }
                            }

                            if row_city_idx == column_city_idx {
                                continue;
                            }

                            let mut to_add = provinces[column_city_idx].clone();

                            provinces[row_city_idx].append(&mut to_add);
                            provinces.remove(column_city_idx);
                        } else {
                            for province in &mut provinces {
                                if province.contains(&row) {
                                    province.push(column);
                                }
                            }
                        }
                    } else if is_city_exist_in(&provinces, &column) {
                        for province in &mut provinces {
                            if province.contains(&column) {
                                province.push(row);
                            }
                        }
                    } else {
                        provinces.push(vec![row, column]);
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
