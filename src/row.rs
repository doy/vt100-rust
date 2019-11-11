use crate::term::BufWrite as _;
use std::convert::TryInto as _;

#[derive(Clone, Debug)]
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

    fn cols(&self) -> u16 {
        self.cells.len().try_into().unwrap()
    }

    pub fn clear(&mut self, attrs: crate::attrs::Attrs) {
        for cell in &mut self.cells {
            cell.clear(attrs);
        }
        self.wrapped = false;
    }

    fn cells(&self) -> impl Iterator<Item = &crate::cell::Cell> {
        self.cells.iter()
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

    pub fn write_contents(
        &self,
        contents: &mut String,
        start: u16,
        width: u16,
    ) {
        let mut prev_was_wide = false;

        for cell in self
            .cells()
            .skip(start as usize)
            .take(width.min(self.content_width(start, false)) as usize)
        {
            if prev_was_wide {
                prev_was_wide = false;
                continue;
            }

            if cell.has_contents() {
                contents.push_str(&cell.contents());
            } else {
                contents.push(' ');
            }

            prev_was_wide = cell.is_wide();
        }
    }

    pub fn write_contents_formatted(
        &self,
        contents: &mut Vec<u8>,
        start: u16,
        width: u16,
        row: u16,
        wrapping: bool,
        mut prev_pos: crate::grid::Pos,
        mut prev_attrs: crate::attrs::Attrs,
    ) -> (crate::grid::Pos, crate::attrs::Attrs) {
        let mut prev_was_wide = false;
        let default_cell = crate::cell::Cell::default();

        for (col, cell) in self
            .cells()
            .enumerate()
            .skip(start as usize)
            .take(width.min(self.content_width(start, true)) as usize)
        {
            if prev_was_wide {
                prev_was_wide = false;
                continue;
            }
            prev_was_wide = cell.is_wide();

            let pos = crate::grid::Pos {
                row,
                col: col.try_into().unwrap(),
            };
            if cell != &default_cell {
                if pos != prev_pos {
                    if pos.row == prev_pos.row + 1 {
                        if !wrapping
                            || prev_pos.col != self.cols()
                            || pos.col != 0
                        {
                            crate::term::CRLF::default().write_buf(contents);
                            crate::term::MoveRight::new(pos.col)
                                .write_buf(contents);
                        }
                    } else if prev_pos.row == pos.row {
                        crate::term::MoveRight::new(pos.col - prev_pos.col)
                            .write_buf(contents);
                    } else {
                        crate::term::MoveTo::new(pos).write_buf(contents);
                    }
                    prev_pos = pos;
                }

                let attrs = cell.attrs();
                if &prev_attrs != attrs {
                    attrs.write_escape_code_diff(contents, &prev_attrs);
                    prev_attrs = *attrs;
                }

                if cell.has_contents() {
                    contents.extend(cell.contents().as_bytes());
                    prev_pos.col += if cell.is_wide() { 2 } else { 1 };
                } else {
                    crate::term::EraseChar::default().write_buf(contents);
                }
            }
        }

        (prev_pos, prev_attrs)
    }

    // while it's true that most of the logic in this is identical to
    // write_contents_formatted, i can't figure out how to break out the
    // common parts without making things noticeably slower.
    pub fn write_contents_diff(
        &self,
        contents: &mut Vec<u8>,
        prev: &Self,
        start: u16,
        width: u16,
        row: u16,
        wrapping: bool,
        mut prev_pos: crate::grid::Pos,
        mut prev_attrs: crate::attrs::Attrs,
    ) -> (crate::grid::Pos, crate::attrs::Attrs) {
        let mut prev_was_wide = false;

        for (col, (cell, prev_cell)) in self
            .cells()
            .zip(prev.cells())
            .enumerate()
            .skip(start as usize)
            .take(width as usize)
        {
            if prev_was_wide {
                prev_was_wide = false;
                continue;
            }
            prev_was_wide = cell.is_wide();

            let pos = crate::grid::Pos {
                row,
                col: col.try_into().unwrap(),
            };
            if cell != prev_cell {
                if pos != prev_pos {
                    if pos.row == prev_pos.row + 1 {
                        if !wrapping
                            || prev_pos.col != self.cols()
                            || pos.col != 0
                        {
                            crate::term::CRLF::default().write_buf(contents);
                            crate::term::MoveRight::new(pos.col)
                                .write_buf(contents);
                        }
                    } else if prev_pos.row == pos.row
                        && prev_pos.col < pos.col
                    {
                        crate::term::MoveRight::new(pos.col - prev_pos.col)
                            .write_buf(contents);
                    } else {
                        crate::term::MoveTo::new(pos).write_buf(contents);
                    }
                    prev_pos = pos;
                }

                let attrs = cell.attrs();
                if &prev_attrs != attrs {
                    attrs.write_escape_code_diff(contents, &prev_attrs);
                    prev_attrs = *attrs;
                }

                if cell.has_contents() {
                    contents.extend(cell.contents().as_bytes());
                    prev_pos.col += if cell.is_wide() { 2 } else { 1 };
                } else {
                    crate::term::EraseChar::default().write_buf(contents);
                }
            }
        }

        (prev_pos, prev_attrs)
    }

    fn content_width(&self, start: u16, formatting: bool) -> u16 {
        for (col, cell) in
            self.cells.iter().skip(start as usize).enumerate().rev()
        {
            if cell.has_contents()
                || (formatting
                    && cell.bgcolor() != crate::attrs::Color::Default)
            {
                let width: u16 = col.try_into().unwrap();
                return width + 1;
            }
        }
        0
    }
}
