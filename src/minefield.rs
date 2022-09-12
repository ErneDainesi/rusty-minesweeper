pub struct MineField<'a> {
    matrix: Vec<&'a [u8]>
}

impl<'a> MineField<'a> {
    pub fn new() -> Self {
        let matrix: Vec<&[u8]> = vec![];
        Self {
           matrix
        }
    }

    pub fn push(&mut self, row: &'a [u8]) {
        self.matrix.push(row);
    }

    pub fn len(&self) -> usize {
        self.matrix.len()
    }

    pub fn get_matrix(&self) -> &Vec<&[u8]> {
        &self.matrix
    }
}
