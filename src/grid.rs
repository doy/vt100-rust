pub struct Grid {
    size: Size,
    rows: Vec<crate::row::Row>,
}

impl Grid {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            rows: vec![
                crate::row::Row::new(size.cols());
                size.rows() as usize
            ],
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn cell(&self, pos: Pos) -> Option<&crate::cell::Cell> {
        self.rows
            .get(pos.row() as usize)
            .and_then(|r| r.get(pos.col()))
    }

    pub fn cell_mut(&mut self, pos: Pos) -> Option<&mut crate::cell::Cell> {
        self.rows
            .get_mut(pos.row() as usize)
            .and_then(|v| v.get_mut(pos.col()))
    }

    pub fn window_contents(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        let mut contents = String::new();
        for row in row_start..=(row_end.min(self.size.rows())) {
            contents += &self.rows[row as usize].contents(col_start, col_end);
        }
        contents
    }

    pub fn window_contents_formatted(
        &self,
        _row_start: u16,
        _col_start: u16,
        _row_end: u16,
        _col_end: u16,
    ) -> String {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Size {
    rows: u16,
    cols: u16,
}

impl Size {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self { rows, cols }
    }

    pub fn rows(self) -> u16 {
        self.rows
    }

    pub fn cols(self) -> u16 {
        self.cols
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos {
    row: u16,
    col: u16,
    size: Size,
}

impl Pos {
    pub fn new(row: u16, col: u16, size: Size) -> Self {
        Self { row, col, size }
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn row(self) -> u16 {
        self.row
    }

    pub fn row_inc(&mut self, count: u16) {
        // XXX this is not correct - scrolling past the bottom of the screen
        self.row += count;
    }

    pub fn row_dec(&mut self, count: u16) {
        // XXX this is not correct - scrolling past the top of the screen
        self.row -= count;
    }

    pub fn row_set(&mut self, i: u16) {
        self.row = i;
    }

    pub fn col(self) -> u16 {
        self.col
    }

    pub fn col_inc(&mut self, count: u16) {
        // XXX handle overflow
        self.col += count;
        if self.col > self.size.cols() {
            self.col = 0;
            self.row += 1;
        }
    }

    pub fn col_dec(&mut self, count: u16) {
        // XXX are there any cases where i need to wrap backwards?
        if count > self.col {
            self.col = 0;
        } else {
            self.col -= count;
        }
    }

    pub fn col_set(&mut self, i: u16) {
        self.col = i;
    }

    pub fn next_tabstop(&mut self) {
        self.col -= self.col % 8;
        self.col += 8;
    }
}
