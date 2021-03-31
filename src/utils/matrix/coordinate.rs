use crate::utils::matrix::direction::Direction;

#[derive(Clone)]
pub struct Coordinate {
    pub row: i32,
    pub column: i32,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Coordinate {
    pub fn next(&self, direction: &Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate {
                row: self.row - 1,
                column: self.column,
            },
            Direction::Down => Coordinate {
                row: self.row + 1,
                column: self.column,
            },
            Direction::Left => Coordinate {
                row: self.row,
                column: self.column - 1,
            },
            Direction::Right => Coordinate {
                row: self.row,
                column: self.column + 1,
            },
        }
    }
}
