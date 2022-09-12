/// This struc represents the minfield read
/// from the file. It contains a matrix that
/// is used to look for mines.
pub struct MineField<'a> {
    matrix: Vec<&'a [u8]>
}

impl<'a> MineField<'a> {
    /// Constructor used to create an empty matrix
    pub fn new() -> Self {
        let matrix: Vec<&[u8]> = vec![];
        Self {
           matrix
        }
    }

    /// Save a row into the matrix
    pub fn push(&mut self, row: &'a [u8]) {
        self.matrix.push(row);
    }

    /// Get the amount of rows the matrix has
    pub fn len(&self) -> usize {
        self.matrix.len()
    }

    /// Function used to get the matrix that represents
    /// the mine field
    pub fn get_matrix(&self) -> &Vec<&[u8]> {
        &self.matrix
    }
}
