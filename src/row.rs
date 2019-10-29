#[derive(Clone)]
pub struct Row {
    cells: Vec<crate::cell::Cell>,
    wrapped: bool,
}

impl Row {
    pub fn new(cols: u16) -> Self {
        Self {
            cells: vec![crate::cell::Cell::default(); cols as usize],
            wrapped: false,
        }
    }

    pub fn get(&self, col: u16) -> Option<&crate::cell::Cell> {
        self.cells.get(col as usize)
    }

    pub fn get_mut(&mut self, col: u16) -> Option<&mut crate::cell::Cell> {
        self.cells.get_mut(col as usize)
    }

    pub fn contents(&self, col_start: u16, col_end: u16) -> String {
        // XXX very inefficient
        let mut max_col = 0;
        for (col, cell) in self.cells.iter().enumerate() {
            if cell.has_contents() {
                max_col = col;
            }
        }
        let mut contents = String::new();
        for col in col_start..=(col_end.min(max_col as u16)) {
            contents += self.cells[col as usize].contents();
        }
        if !self.wrapped {
            contents += "\n";
        }
        contents
    }
}
