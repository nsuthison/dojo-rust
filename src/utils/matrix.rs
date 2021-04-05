use crate::utils::matrix::coordinate::Coordinate;

pub mod coordinate;
pub mod direction;

pub struct Matrix<'a, T> {
    data: &'a [Vec<T>],
}

impl<T> Matrix<'_, T> {
    pub fn new(matrix: &[Vec<T>]) -> Matrix<T> {
        Matrix { data: matrix }
    }

    pub fn is_in_boundary(&self, cell: &Coordinate) -> bool {
        let is_row_inbound = || cell.row >= 0 && cell.row < self.data.len() as i32;
        let is_column_inbound =
            || cell.column >= 0 && cell.column < self.data[cell.row as usize].len() as i32;

        is_row_inbound() && is_column_inbound()
    }
}

#[cfg(test)]
pub mod matrix_test {
    use super::*;
    use rstest::rstest;

    #[rstest(
    input,
    coordinate,
    case(vec ! [vec ! [1, 1, 1], vec ! [1, 1, 1]], Coordinate { row: 0, column: 1}),
    case(vec ! [vec ! [1, 1, 1], vec ! [1, 1, 1]], Coordinate { row: 1, column: 2}),
    case(vec ! [vec ! [1, 1, 1], vec ! [1, 1, 1]], Coordinate { row: 1, column: 2}),
    )]
    fn is_in_boundary_should_return_true_when_given_coordinate_is_in_the_boundary(
        input: Vec<Vec<i32>>,
        coordinate: Coordinate,
    ) {
        let matrix = Matrix::new(&input);

        assert_eq!(matrix.is_in_boundary(&coordinate), true);
    }

    #[rstest(
    input,
    coordinate,
    case(vec ! [vec ! [1, 1, 1], vec ! [1, 1, 1]], Coordinate { row: 5, column: 5}),
    case(vec ! [vec ! [1, 1, 1], vec ! [1, 1, 1]], Coordinate { row: 3, column: 3}),
    case(vec ! [vec ! [1, 1, 1], vec ! [1, 1, 1]], Coordinate { row: 2, column: 3}),
    )]
    fn is_in_boundary_should_return_false_when_given_coordinate_is_outside_the_boundary(
        input: Vec<Vec<i32>>,
        coordinate: Coordinate,
    ) {
        let matrix = Matrix::new(&input);

        assert_eq!(matrix.is_in_boundary(&coordinate), false);
    }
}
