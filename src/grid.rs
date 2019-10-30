pub struct Grid {
    size: Size,
    pos: Pos,
    saved_pos: Pos,
    rows: Vec<crate::row::Row>,
}

impl Grid {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            pos: Pos::default(),
            saved_pos: Pos::default(),
            rows: vec![crate::row::Row::new(size.cols); size.rows as usize],
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    pub fn pos(&self) -> &Pos {
        &self.pos
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
        self.row_clamp();
        self.col_clamp();
    }

    pub fn save_pos(&mut self) {
        self.saved_pos = self.pos;
    }

    pub fn restore_pos(&mut self) {
        self.pos = self.saved_pos;
    }

    pub fn cell(&self, pos: Pos) -> Option<&crate::cell::Cell> {
        self.rows.get(pos.row as usize).and_then(|r| r.get(pos.col))
    }

    pub fn cell_mut(&mut self, pos: Pos) -> Option<&mut crate::cell::Cell> {
        self.rows
            .get_mut(pos.row as usize)
            .and_then(|v| v.get_mut(pos.col))
    }

    pub fn current_cell_mut(&mut self) -> Option<&mut crate::cell::Cell> {
        self.cell_mut(self.pos)
    }

    pub fn window_contents(
        &self,
        row_start: u16,
        col_start: u16,
        row_end: u16,
        col_end: u16,
    ) -> String {
        let mut contents = String::new();
        for row in row_start..=(row_end.min(self.size.rows)) {
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

    pub fn erase_all(&mut self) {
        self.rows = vec![
            crate::row::Row::new(self.size.cols);
            self.size.rows as usize
        ];
    }

    pub fn erase_all_forward(&mut self, pos: Pos) {
        for i in (pos.row + 1)..self.size.rows {
            self.rows[i as usize] = crate::row::Row::new(self.size.cols);
        }
        let row = &mut self.rows[pos.row as usize];
        for i in pos.col..self.size.cols {
            *row.get_mut(i).unwrap() = crate::cell::Cell::default();
        }
    }

    pub fn erase_all_backward(&mut self, pos: Pos) {
        for i in 0..pos.row {
            self.rows[i as usize] = crate::row::Row::new(self.size.cols);
        }
        let row = &mut self.rows[pos.row as usize];
        for i in 0..pos.col {
            *row.get_mut(i).unwrap() = crate::cell::Cell::default();
        }
    }

    pub fn erase_row(&mut self, pos: Pos) {
        self.rows[pos.row as usize] = crate::row::Row::new(self.size.cols);
    }

    pub fn erase_row_forward(&mut self, pos: Pos) {
        let row = &mut self.rows[pos.row as usize];
        for i in pos.col..self.size.cols {
            *row.get_mut(i).unwrap() = crate::cell::Cell::default();
        }
    }

    pub fn erase_row_backward(&mut self, pos: Pos) {
        let row = &mut self.rows[pos.row as usize];
        for i in 0..pos.col {
            *row.get_mut(i).unwrap() = crate::cell::Cell::default();
        }
    }

    pub fn insert_cells(&mut self, pos: Pos, count: u16) {
        let row = &mut self.rows[pos.row as usize];
        for _ in 0..count {
            row.insert(pos.col as usize, crate::cell::Cell::default());
        }
        row.truncate(self.size.cols as usize);
    }

    pub fn delete_cells(&mut self, pos: Pos, count: u16) {
        let row = &mut self.rows[pos.row as usize];
        for _ in 0..(count.min(self.size.cols - pos.col)) {
            row.remove(pos.col as usize);
        }
        row.resize(self.size.cols as usize, crate::cell::Cell::default());
    }

    pub fn erase_cells(&mut self, pos: Pos, count: u16) {
        let row = &mut self.rows[pos.row as usize];
        for i in pos.col..(pos.col + count).min(self.size.cols - 1) {
            *row.get_mut(i).unwrap() = crate::cell::Cell::default();
        }
    }

    pub fn insert_lines(&mut self, pos: Pos, count: u16) {
        for _ in 0..count {
            self.rows.insert(
                pos.row as usize,
                crate::row::Row::new(self.size.cols),
            );
        }
        self.rows.truncate(self.size.rows as usize)
    }

    pub fn delete_lines(&mut self, pos: Pos, count: u16) {
        for _ in 0..(count.min(self.size.rows - pos.row)) {
            self.rows.remove(pos.row as usize);
        }
        self.rows.resize(
            self.size.rows as usize,
            crate::row::Row::new(self.size.cols),
        )
    }

    pub fn scroll_up(&mut self, count: u16) {
        self.delete_lines(Pos { row: 0, col: 0 }, count);
    }

    pub fn scroll_down(&mut self, count: u16) {
        self.insert_lines(Pos { row: 0, col: 0 }, count);
    }

    pub fn row_inc_clamp(&mut self, count: u16) {
        self.pos.row = self.pos.row.saturating_add(count);
        self.row_clamp();
    }

    pub fn row_inc_scroll(&mut self, count: u16) {
        self.pos.row = self.pos.row.saturating_add(count);
        let lines = self.row_clamp();
        self.scroll_up(lines);
    }

    pub fn row_dec(&mut self, count: u16) {
        self.pos.row = self.pos.row.saturating_sub(count);
    }

    pub fn row_set(&mut self, i: u16) {
        self.pos.row = i;
        self.row_clamp();
    }

    pub fn col_inc_clamp(&mut self, count: u16) {
        self.pos.col = self.pos.col.saturating_add(count);
        self.col_clamp();
    }

    pub fn col_inc_wrap(&mut self, count: u16) {
        self.pos.col = self.pos.col.saturating_add(count);
        self.col_wrap();
    }

    pub fn col_dec(&mut self, count: u16) {
        self.pos.col = self.pos.col.saturating_sub(count);
    }

    pub fn col_tab(&mut self) {
        self.pos.col -= self.pos.col % 8;
        self.pos.col += 8;
        self.col_clamp();
    }

    pub fn col_set(&mut self, i: u16) {
        self.pos.col = i;
        self.col_clamp();
    }

    fn row_clamp(&mut self) -> u16 {
        if self.pos.row > self.size.rows - 1 {
            let rows = self.pos.row - (self.size.rows - 1);
            self.pos.row = self.size.rows - 1;
            rows
        } else {
            0
        }
    }

    fn col_clamp(&mut self) {
        if self.pos.col > self.size.cols - 1 {
            self.pos.col = self.size.cols - 1;
        }
    }

    fn col_wrap(&mut self) {
        if self.pos.col > self.size.cols - 1 {
            self.pos.col = 0;
            self.row_inc_scroll(1);
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Size {
    pub rows: u16,
    pub cols: u16,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Pos {
    pub row: u16,
    pub col: u16,
}
