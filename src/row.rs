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
        let mut max_col = None;
        for (col, cell) in self.cells.iter().enumerate() {
            if cell.has_contents() {
                max_col = Some(col);
            }
        }

        let mut contents = String::new();
        if let Some(max_col) = max_col {
            for col in col_start..=(col_end.min(max_col as u16)) {
                let cell_contents = self.cells[col as usize].contents();
                let cell_contents = if cell_contents == "" {
                    " "
                } else {
                    cell_contents
                };
                contents += cell_contents;
            }
        }
        if !self.wrapped {
            contents += "\n";
        }
        contents
    }
}
