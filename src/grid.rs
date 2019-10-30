pub struct Grid {
    size: crate::pos::Pos,
    rows: Vec<crate::row::Row>,
}

impl Grid {
    pub fn new(size: crate::pos::Pos) -> Self {
        Self {
            size,
            rows: vec![crate::row::Row::new(size.col); size.row as usize],
        }
    }

    pub fn cell(&self, pos: crate::pos::Pos) -> Option<&crate::cell::Cell> {
        self.rows.get(pos.row as usize).and_then(|r| r.get(pos.col))
    }

    pub fn cell_mut(
        &mut self,
        pos: crate::pos::Pos,
    ) -> Option<&mut crate::cell::Cell> {
        self.rows
            .get_mut(pos.row as usize)
            .and_then(|v| v.get_mut(pos.col))
    }

    pub fn window_contents(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        let mut contents = String::new();
        for row in row_start..=(row_end.min(self.size.row)) {
            contents += &self.rows[row as usize].contents(col_start, col_end);
        }
        contents
    }

    pub fn window_contents_formatted(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        unimplemented!()
    }
}
