pub struct Grid {
    size: Size,
    pos: Pos,
    saved_pos: Pos,
    rows: Vec<crate::row::Row>,
    scroll_top: u16,
    scroll_bottom: u16,
}

impl Grid {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            pos: Pos::default(),
            saved_pos: Pos::default(),
            rows: vec![crate::row::Row::new(size.cols); size.rows as usize],
            scroll_top: 0,
            scroll_bottom: size.rows - 1,
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
        self.row_clamp_top();
        self.row_clamp_bottom();
        self.col_clamp();
    }

    pub fn save_pos(&mut self) {
        self.saved_pos = self.pos;
    }

    pub fn restore_pos(&mut self) {
        self.pos = self.saved_pos;
    }

    pub fn row(&self, pos: Pos) -> Option<&crate::row::Row> {
        self.rows.get(pos.row as usize)
    }

    pub fn row_mut(&mut self, pos: Pos) -> Option<&mut crate::row::Row> {
        self.rows.get_mut(pos.row as usize)
    }

    pub fn current_row_mut(&mut self) -> Option<&mut crate::row::Row> {
        self.row_mut(self.pos)
    }

    pub fn cell(&self, pos: Pos) -> Option<&crate::cell::Cell> {
        self.row(pos).and_then(|r| r.get(pos.col))
    }

    pub fn cell_mut(&mut self, pos: Pos) -> Option<&mut crate::cell::Cell> {
        self.row_mut(pos).and_then(|r| r.get_mut(pos.col))
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
        for row in row_start..=(row_end.min(self.size.rows - 1)) {
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
        let mut contents = String::new();
        let mut prev_attrs = crate::attrs::Attrs::default();
        for row in row_start..=(row_end.min(self.size.rows - 1)) {
            let (new_contents, new_attrs) = &self.rows[row as usize]
                .contents_formatted(col_start, col_end, prev_attrs);
            contents += new_contents;
            prev_attrs = *new_attrs;
        }
        contents
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
        row.wrap(false);
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
        row.wrap(false);
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
            self.rows.remove(self.scroll_bottom as usize);
            self.rows.insert(
                pos.row as usize,
                crate::row::Row::new(self.size.cols),
            );
        }
    }

    pub fn delete_lines(&mut self, pos: Pos, count: u16) {
        for _ in 0..(count.min(self.size.rows - pos.row)) {
            self.rows.insert(
                self.scroll_bottom as usize + 1,
                crate::row::Row::new(self.size.cols),
            );
            self.rows.remove(pos.row as usize);
        }
    }

    pub fn scroll_up(&mut self, count: u16) {
        self.delete_lines(
            Pos {
                row: self.scroll_top,
                col: 0,
            },
            count,
        );
    }

    pub fn scroll_down(&mut self, count: u16) {
        self.insert_lines(
            Pos {
                row: self.scroll_top,
                col: 0,
            },
            count,
        );
    }

    // TODO: left/right
    pub fn set_scroll_region(
        &mut self,
        top: u16,
        bottom: u16,
        _left: u16,
        _right: u16,
    ) {
        let bottom = bottom.min(self.size().rows - 1);
        if top < bottom {
            self.scroll_top = top;
            self.scroll_bottom = bottom;
        } else {
            self.scroll_top = 0;
            self.scroll_bottom = self.size().rows - 1;
        }
        self.pos.row = self.scroll_top;
        self.pos.col = 0;
    }

    pub fn row_inc_clamp(&mut self, count: u16) {
        self.pos.row = self.pos.row.saturating_add(count);
        self.row_clamp_bottom();
    }

    pub fn row_inc_scroll(&mut self, count: u16) {
        self.pos.row = self.pos.row.saturating_add(count);
        let lines = self.row_clamp_bottom();
        self.scroll_up(lines);
    }

    pub fn row_dec_clamp(&mut self, count: u16) {
        self.pos.row = self.pos.row.saturating_sub(count);
        self.row_clamp_top();
    }

    pub fn row_dec_scroll(&mut self, count: u16) {
        self.pos.row = self.pos.row.saturating_sub(count);
        let lines = self.row_clamp_top();
        self.scroll_down(lines);
    }

    pub fn row_set(&mut self, i: u16) {
        self.pos.row = i;
        self.row_clamp_top();
        self.row_clamp_bottom();
    }

    pub fn col_inc(&mut self, count: u16) {
        self.pos.col = self.pos.col.saturating_add(count);
    }

    pub fn col_inc_clamp(&mut self, count: u16) {
        self.pos.col = self.pos.col.saturating_add(count);
        self.col_clamp();
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

    pub fn col_wrap(&mut self, width: u16) {
        if self.pos.col > self.size.cols - width {
            self.current_row_mut().unwrap().wrap(true);
            self.pos.col = 0;
            self.row_inc_scroll(1);
        }
    }

    fn row_clamp_top(&mut self) -> u16 {
        if self.pos.row < self.scroll_top {
            let rows = self.scroll_top - self.pos.row;
            self.pos.row = self.scroll_top;
            rows
        } else {
            0
        }
    }

    fn row_clamp_bottom(&mut self) -> u16 {
        if self.pos.row > self.scroll_bottom {
            let rows = self.pos.row - self.scroll_bottom;
            self.pos.row = self.scroll_bottom;
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
