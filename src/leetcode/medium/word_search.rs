struct Solution;

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
    board: &Vec<Vec<char>>,
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

        if can_go_up(cell, &use_cell) {
            let next_cell = Coordinate {
                row: cell.row - 1,
                column: cell.column,
            };

            if find_next(
                &next_cell,
                char_position + 1,
                word,
                board,
                &mut use_cell.clone(),
            ) {
                return true;
            }
        }

        if can_go_down(cell, &use_cell, board) {
            let next_cell = Coordinate {
                column: cell.column,
                row: cell.row + 1,
            };

            if find_next(
                &next_cell,
                char_position + 1,
                word,
                board,
                &mut use_cell.clone(),
            ) {
                return true;
            }
        }

        if can_go_left(cell, &use_cell) {
            let next_cell = Coordinate {
                column: cell.column - 1,
                row: cell.row,
            };

            if find_next(
                &next_cell,
                char_position + 1,
                word,
                board,
                &mut use_cell.clone(),
            ) {
                return true;
            }
        }

        if can_go_right(cell, &use_cell, board) {
            let next_cell = Coordinate {
                column: cell.column + 1,
                row: cell.row,
            };

            if find_next(
                &next_cell,
                char_position + 1,
                word,
                board,
                &mut use_cell.clone(),
            ) {
                return true;
            }
        }
    }

    false
}

fn can_go_up(current_cell: &Coordinate, use_cell: &Vec<Coordinate>) -> bool {
    let next_cell = Coordinate {
        row: current_cell.row - 1,
        column: current_cell.column,
    };

    next_cell.row >= 0 && !use_cell.contains(&next_cell)
}

fn can_go_down(
    current_cell: &Coordinate,
    use_cell: &Vec<Coordinate>,
    board: &Vec<Vec<char>>,
) -> bool {
    let next_cell = Coordinate {
        row: current_cell.row + 1,
        column: current_cell.column,
    };

    next_cell.row < board.len() as i32 && !use_cell.contains(&next_cell)
}

fn can_go_left(current_cell: &Coordinate, use_cell: &Vec<Coordinate>) -> bool {
    let next_cell = Coordinate {
        row: current_cell.row,
        column: current_cell.column - 1,
    };

    next_cell.column >= 0 && !use_cell.contains(&next_cell)
}

fn can_go_right(
    current_cell: &Coordinate,
    use_cell: &Vec<Coordinate>,
    board: &Vec<Vec<char>>,
) -> bool {
    let next_cell = Coordinate {
        row: current_cell.row,
        column: current_cell.column + 1,
    };

    next_cell.column < board[0].len() as i32 && !use_cell.contains(&next_cell)
}

fn is_char_match(to_check: char, position_to_check: &Coordinate, board: &Vec<Vec<char>>) -> bool {
    to_check == board[position_to_check.row as usize][position_to_check.column as usize]
}

#[derive(Clone)]
struct Coordinate {
    row: i32,
    column: i32,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
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
