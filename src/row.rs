use std::convert::TryInto as _;

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

    pub fn clear(&mut self) {
        *self = Self::new(self.cells.len().try_into().unwrap());
    }

    pub fn cells_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut crate::cell::Cell> {
        self.cells.iter_mut()
    }

    pub fn get(&self, col: u16) -> Option<&crate::cell::Cell> {
        self.cells.get(col as usize)
    }

    pub fn get_mut(&mut self, col: u16) -> Option<&mut crate::cell::Cell> {
        self.cells.get_mut(col as usize)
    }

    pub fn insert(&mut self, i: usize, cell: crate::cell::Cell) {
        self.cells.insert(i, cell);
    }

    pub fn remove(&mut self, i: usize) {
        self.cells.remove(i);
    }

    pub fn truncate(&mut self, len: usize) {
        self.cells.truncate(len);
    }

    pub fn resize(&mut self, len: usize, cell: crate::cell::Cell) {
        self.cells.resize(len, cell);
    }

    pub fn wrap(&mut self, wrap: bool) {
        self.wrapped = wrap;
    }

    pub fn wrapped(&self) -> bool {
        self.wrapped
    }

    pub fn contents(&self, col_start: u16, col_end: u16) -> String {
        let mut prev_was_wide = false;
        let mut contents = String::new();
        if let Some(max_col) = self.max_col() {
            for col in col_start..=(col_end.min(max_col)) {
                if prev_was_wide {
                    prev_was_wide = false;
                    continue;
                }

                let cell = &self.cells[col as usize];
                let cell_contents = cell.contents();
                let cell_contents = if cell_contents == "" {
                    " "
                } else {
                    cell_contents
                };
                contents += cell_contents;
                prev_was_wide = cell.is_wide();
            }
        }
        if !self.wrapped {
            contents += "\n";
        }
        contents
    }

    pub fn contents_formatted(
        &self,
        col_start: u16,
        col_end: u16,
        attrs: crate::attrs::Attrs,
    ) -> (String, crate::attrs::Attrs) {
        let mut prev_was_wide = false;
        let mut contents = String::new();
        let mut prev_attrs = attrs;
        if let Some(max_col) = self.max_col() {
            for col in col_start..=(col_end.min(max_col)) {
                if prev_was_wide {
                    prev_was_wide = false;
                    continue;
                }

                let cell = &self.cells[col as usize];

                let attrs = cell.attrs();
                if &prev_attrs != attrs {
                    contents += &attrs.escape_code_diff(&prev_attrs);
                    prev_attrs = *attrs;
                }

                let cell_contents = cell.contents();
                let cell_contents = if cell_contents == "" {
                    " "
                } else {
                    cell_contents
                };
                contents += cell_contents;

                prev_was_wide = cell.is_wide();
            }
        }
        if !self.wrapped {
            contents += "\r\n";
        }
        (contents, prev_attrs)
    }

    fn max_col(&self) -> Option<u16> {
        let mut prev_was_wide = false;
        // XXX very inefficient
        let mut max_col = None;
        for (col, cell) in self.cells.iter().enumerate() {
            if cell.has_contents() || prev_was_wide {
                max_col = Some(col.try_into().unwrap());
                prev_was_wide = cell.is_wide();
            }
        }
        max_col
    }
}
