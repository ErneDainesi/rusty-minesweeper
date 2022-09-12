const MINE: u8 = b'*';

pub struct Field {
    neighbours: Vec<u8>
}

impl Field {
    pub fn new(matrix: &crate::minefield::MineField, row: usize, column: usize) -> Self {
        Self {
            neighbours: Self::get_neighbours(matrix, row, column)
        }
    }

    fn get_neighbours(mine_field: &crate::minefield::MineField, row: usize, column: usize) -> Vec<u8> {
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

    pub fn sweep_mines(&self, count: &mut i32) {
        for field in self.neighbours.iter() {
            if is_mine(*field) {
                *count += 1
            }
        }
    }

    fn safe_decrement(n: usize, default_value: usize) -> usize {
        n.checked_sub(1).unwrap_or(default_value)
    }
}

pub fn is_mine(value: u8) -> bool {
    value == MINE
}
