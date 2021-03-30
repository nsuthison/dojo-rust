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
