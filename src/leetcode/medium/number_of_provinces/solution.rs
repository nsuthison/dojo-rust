use crate::utils::matrix::coordinate::Coordinate;

/// Question: https://leetcode.com/problems/number-of-provinces/
// impl Solution {
//     pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
//         0
//     }
// }
//
// fn find_number_of_provinces(is_connected: &mut Vec<&mut Vec<i32>>) -> i32 {
//     0
// }

fn update(is_connected: &mut Vec<Vec<i32>>, at: Coordinate, by: i32) -> &mut Vec<Vec<i32>> {
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
    by,
    expected,
    case(vec ! [vec ! [1, 1, 1], vec ! [1, 1, 1]], Coordinate { row: 0, column: 1}, 0, vec ! [vec ! [1, 0, 1], vec ! [1, 1, 1]]),
    )]
    fn update_is_connected_should_return_same_is_connected_with_data_change_in_specific_coordinate_by_specific_value(
        input: Vec<Vec<i32>>,
        at: Coordinate,
        by: i32,
        expected: Vec<Vec<i32>>,
    ) {
        let input = &mut input.to_owned();
        let expected = &mut expected.to_owned();

        assert_eq!(update(input, at, by), expected);
    }
}
