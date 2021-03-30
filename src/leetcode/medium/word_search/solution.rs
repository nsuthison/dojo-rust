struct Solution;

use super::coordinate::Coordinate;
use crate::leetcode::medium::word_search::coordinate::Direction;

/// Question: https://leetcode.com/problems/word-search/
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for row in 0..(board.len() as i32) {
            for column in 0..(board[row as usize].len() as i32) {
                let is_match = find_next(
                    &Coordinate { row, column },
                    0,
                    &word,
                    &board,
                    &mut Vec::new(),
                );

                if is_match {
                    return true;
                }
            }
        }

        false
    }
}

fn find_next(
    cell: &Coordinate,
    char_position: i32,
    word: &str,
    board: &[Vec<char>],
    use_cell: &mut Vec<Coordinate>,
) -> bool {
    if is_char_match(
        word.chars().nth(char_position as usize).unwrap(),
        cell,
        board,
    ) {
        if char_position == word.len() as i32 - 1 {
            return true;
        }

        use_cell.push(cell.clone());

        if can_go_to(&cell.next(Direction::Up), &use_cell, board)
            && find_next(
                &cell.next(Direction::Up),
                char_position + 1,
                word,
                board,
                &mut use_cell.clone(),
            )
        {
            return true;
        }

        if can_go_to(&cell.next(Direction::Down), &use_cell, board)
            && find_next(
                &cell.next(Direction::Down),
                char_position + 1,
                word,
                board,
                &mut use_cell.clone(),
            )
        {
            return true;
        }

        if can_go_to(&cell.next(Direction::Left), &use_cell, board)
            && find_next(
                &cell.next(Direction::Left),
                char_position + 1,
                word,
                board,
                &mut use_cell.clone(),
            )
        {
            return true;
        }

        if can_go_to(&cell.next(Direction::Right), &use_cell, board)
            && find_next(
                &cell.next(Direction::Right),
                char_position + 1,
                word,
                board,
                &mut use_cell.clone(),
            )
        {
            return true;
        }
    }

    false
}

fn is_char_match(to_check: char, position_to_check: &Coordinate, board: &[Vec<char>]) -> bool {
    to_check == board[position_to_check.row as usize][position_to_check.column as usize]
}

fn can_go_to(cell: &Coordinate, use_cells: &[Coordinate], board: &[Vec<char>]) -> bool {
    is_in_boundary(cell, board) && !use_cells.contains(cell)
}

fn is_in_boundary(cell: &Coordinate, board: &[Vec<char>]) -> bool {
    let is_row_inbound = || cell.row >= 0 && cell.row < board.len() as i32;
    let is_column_inbound =
        || cell.column >= 0 && cell.column < board[cell.row as usize].len() as i32;

    is_row_inbound() && is_column_inbound()
}

#[cfg(test)]
pub mod solution_test {
    use super::*;
    use rstest::rstest;

    #[rstest(
    board,
    word,
    case(vec ! [vec ! ['A']], "A"),
    case(vec ! [vec ! ['A', 'B', 'C', 'E'], vec ! ['S', 'F', 'C', 'S'], vec ! ['A', 'D', 'E', 'E']], "ABCCED"),
    case(vec ! [vec ! ['A', 'B', 'C', 'E'], vec ! ['S', 'F', 'C', 'S'], vec ! ['A', 'D', 'E', 'E']], "SEE"),
    case(vec ! [vec ! ['A', 'B', 'C', 'E'], vec ! ['S', 'F', 'E', 'S'], vec ! ['A', 'D', 'E', 'E']], "ABCESEEEFS")
    )]
    fn exist_should_return_true_when_can_find_word_in_board(board: Vec<Vec<char>>, word: &str) {
        assert_eq!(Solution::exist(board, String::from(word)), true);
    }

    #[rstest(
    board,
    word,
    case(vec ! [vec ! ['A', 'B', 'C', 'E'], vec ! ['S', 'F', 'C', 'S'], vec ! ['A', 'D', 'E', 'E']], "ABCB"),
    )]
    fn exist_should_return_false_when_can_find_word_in_board(board: Vec<Vec<char>>, word: &str) {
        assert_eq!(Solution::exist(board, String::from(word)), false);
    }
}
