// use crate::utils::matrix::coordinate::Coordinate;
// use crate::utils::matrix::direction::Direction;
// use crate::utils::matrix::Matrix;
//
// /// Question: https://leetcode.com/problems/number-of-provinces/
// #[allow(clippy::redundant_clone)]
// impl Solution {
//     pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
//         let mut provinces: Vec<Vec<i32>> = vec![vec![]];
//
//         for row in 0..(is_connected.len() as i32) {
//             for column in row..(is_connected[row as usize].len() as i32) {
//                 if row == column { continue; }
//
//                 if is_connected[row][column] == 1 {
//                     let mut is_city_exist_in_province = false;
//                     for province in provinces {
//                         if province.contains(&row) {
//                             is_city_exist_in_province = true;
//                             break;
//                         }
//                     }
//                 }
//             }
//         }
//
//         provinces.len() as i32
//     }
// }
//
//
// struct Solution;
//
// #[cfg(test)]
// pub mod solution_test {
//     use super::*;
//     use rstest::rstest;
//
// }
