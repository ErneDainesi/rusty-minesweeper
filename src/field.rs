/// The byte value of a mine, used later
/// to check if the field is a mine or not
const MINE: u8 = b'*';

/// The Field struct represents each value
/// of the mine field. It stores its neighbours
/// which are later used to check if any of them
/// are a mine
pub struct Field {
    neighbours: Vec<u8>,
}

impl Field {
    /// Basic constructor of the field, it receives a mine field,
    /// the current row and column and it uses the get_neighbours
    /// function to store the values of the neighbours to the
    /// current field
    pub fn new(matrix: &crate::minefield::MineField, row: usize, column: usize) -> Self {
        Self {
            neighbours: Self::get_neighbours(matrix, row, column),
        }
    }

    /// Receives the mine field and the current row and column.
    /// First it gets each necessary index of the neighbour fields
    /// Then, if the indexes are valid, the value of each neighbour
    /// field is stored into a vector.
    fn get_neighbours(
        mine_field: &crate::minefield::MineField,
        row: usize,
        column: usize,
    ) -> Vec<u8> {
        let left_idx = Self::safe_decrement(column, mine_field.get_matrix()[row].len());
        let top_idx = Self::safe_decrement(row, mine_field.get_matrix()[row].len());
        let bottom_idx = row + 1;
        let right_idx = column + 1;
        let mut neighbours: Vec<u8> = vec![];
        let row_length = mine_field.get_matrix()[row].len();
        if right_idx < row_length {
            neighbours.push(mine_field.get_matrix()[row][right_idx]);
        }
        if left_idx < row_length {
            neighbours.push(mine_field.get_matrix()[row][left_idx]);
        }
        if top_idx < row_length && right_idx < row_length {
            neighbours.push(mine_field.get_matrix()[top_idx][right_idx]);
        }
        if top_idx < row_length && left_idx < row_length {
            neighbours.push(mine_field.get_matrix()[top_idx][left_idx]);
        }
        if bottom_idx < mine_field.len() && left_idx < row_length {
            neighbours.push(mine_field.get_matrix()[bottom_idx][left_idx]);
        }
        if top_idx < row_length {
            neighbours.push(mine_field.get_matrix()[top_idx][column]);
        }
        if bottom_idx < mine_field.len() && right_idx < row_length {
            neighbours.push(mine_field.get_matrix()[bottom_idx][right_idx]);
        }
        if bottom_idx < mine_field.len() {
            neighbours.push(mine_field.get_matrix()[bottom_idx][column]);
        }
        neighbours
    }

    /// This function iterates through each neighbour
    /// and checks if the value of the neighbour field
    /// is a mine. If it is, then the mine_count is incremented
    pub fn sweep_mines(&self, mine_count: &mut i32) {
        for field in self.neighbours.iter() {
            if is_mine(*field) {
                *mine_count += 1
            }
        }
    }

    /// This is a utility function to avoid overflow when
    /// subtracting from a usize. If theres no overflow,
    /// then the subtraction is executed. Otherwise, a
    /// default value is returned, this dafault value is
    /// later checked to make sure we dont access an invalid
    /// position of a vector
    fn safe_decrement(n: usize, default_value: usize) -> usize {
        n.checked_sub(1).unwrap_or(default_value)
    }
}

/// This function receivs a value from a field
/// and checks whether or not the value is a mine
pub fn is_mine(value: u8) -> bool {
    value == MINE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test to check if value passed isn't a mine
    fn value_is_not_a_mine() {
        let value: u8 = ".".as_bytes()[0];
        assert!(!is_mine(value));
    }

    #[test]
    /// Test to check if passing a * is treated
    /// as a mine
    fn value_is_a_mine() {
        let value: u8 = "*".as_bytes()[0];
        assert!(is_mine(value));
    }

    #[test]
    /// Test if passing a row to the mine field
    /// and the sweeping for mines the same row
    /// returns the expected amount of mines
    fn correct_amount_of_mines_are_found() {
        let mut mine_count = 0;
        let mine_field_row = ".*.*.".as_bytes();
        let mut mine_field = crate::minefield::MineField::new();
        mine_field.push(mine_field_row);
        let row = 0;
        let column = 2;
        let field = Field::new(&mine_field, row, column);
        field.sweep_mines(&mut mine_count);
        assert_eq!(mine_count, 2);
    }
}
