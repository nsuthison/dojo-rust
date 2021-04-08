use crate::utils::matrix::coordinate::Coordinate;
use crate::utils::matrix::direction::Direction;
use crate::utils::matrix::Matrix;

/// Question: https://leetcode.com/problems/number-of-islands/
#[allow(clippy::redundant_clone)]
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut number_of_island = 0;
        let grid = &mut grid.to_owned();

        for row in 0..(grid.len() as i32) {
            for column in 0..(grid[row as usize].len() as i32) {
                if grid[row as usize][column as usize] == '1' {
                    number_of_island += 1;

                    traverse_cities(&Coordinate { row, column }, grid);
                }
            }
        }

        number_of_island
    }
}

fn traverse_cities(coordinate: &Coordinate, is_connected: &mut Vec<Vec<char>>) {
    if Matrix::is_in_boundary(&Matrix::new(is_connected), &coordinate)
        && is_connected[coordinate.row as usize][coordinate.column as usize] == '1'
    {
        update(is_connected, coordinate, '0');
    } else {
        return;
    }

    let directions: Vec<Direction> = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    for direction in directions {
        let next = coordinate.next(&direction);

        if Matrix::is_in_boundary(&Matrix::new(is_connected), &next) {
            traverse_cities(&next, is_connected);
        }
    }
}

fn update<'a>(
    is_connected: &'a mut Vec<Vec<char>>,
    at: &'a Coordinate,
    by: char,
) -> &'a mut Vec<Vec<char>> {
    is_connected[at.row as usize][at.column as usize] = by;
    is_connected
}

struct Solution;

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(
    input,
    at,
    expected,
    case(vec ! [vec ! ['1', '1', '1'], vec ! ['1', '1', '1']], Coordinate { row: 0, column: 0}, vec ! [vec ! ['0', '0', '0'], vec ! ['0', '0', '0']]),
    )]
    fn traverse_cities_should_update_all_cell_to_zero_when_given_coordinate_which_contain_group_of_one(
        input: Vec<Vec<char>>,
        at: Coordinate,
        expected: Vec<Vec<char>>,
    ) {
        let input = &mut input.to_owned();
        let expected = &mut expected.to_owned();

        traverse_cities(&at, input);

        assert_eq!(input, expected);
    }

    #[rstest(
    input,
    at,
    by,
    expected,
    case(vec ! [vec ! ['1', '1', '1'], vec ! ['1', '1', '1']], Coordinate { row: 0, column: 1}, '0', vec ! [vec ! ['1', '0', '1'], vec ! ['1', '1', '1']]),
    )]
    fn update_is_connected_should_return_is_connected_with_data_change_in_specific_coordinate_by_specific_value(
        input: Vec<Vec<char>>,
        at: Coordinate,
        by: char,
        expected: Vec<Vec<char>>,
    ) {
        let input = &mut input.to_owned();
        let expected = &mut expected.to_owned();

        assert_eq!(update(input, &at, by), expected);
    }

    #[rstest(
    is_connected,
    expected,
    case(vec ! [vec ! ['1', '1', '0'], vec ! ['1', '1', '0'], vec ! ['0', '0', '1']], 2),
    )]
    fn number_of_island_should_return_number_of_provinces_when_given_matrix_one_and_zero_which_group_of_one_represent_a_province(
        is_connected: Vec<Vec<char>>,
        expected: i32,
    ) {
        assert_eq!(Solution::num_islands(is_connected), expected);
    }
}
