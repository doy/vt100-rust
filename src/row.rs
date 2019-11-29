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
        self.clear_wide(i.try_into().unwrap());
        self.cells.remove(i);
    }

    pub fn erase(&mut self, i: usize, attrs: crate::attrs::Attrs) {
        self.clear_wide(i.try_into().unwrap());
        self.cells.get_mut(i).unwrap().clear(attrs);
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

    pub fn clear_wide(&mut self, col: u16) {
        let cell = self.get(col).unwrap();
        let other = if cell.is_wide() {
            self.get_mut(col + 1).unwrap()
        } else if cell.is_wide_continuation() {
            self.get_mut(col - 1).unwrap()
        } else {
            return;
        };
        other.clear(*other.attrs());
    }

    pub fn write_contents(
        &self,
        contents: &mut String,
        start: u16,
        width: u16,
    ) {
        let mut prev_was_wide = false;

        let mut prev_col = start;
        for (col, cell) in self
            .cells()
            .enumerate()
            .skip(start as usize)
            .take(width as usize)
        {
            if prev_was_wide {
                prev_was_wide = false;
                continue;
            }
            prev_was_wide = cell.is_wide();

            let col: u16 = col.try_into().unwrap();
            if cell.has_contents() {
                for _ in 0..(col - prev_col) {
                    contents.push(' ');
                }
                prev_col += col - prev_col;

                contents.push_str(&cell.contents());
                prev_col += if cell.is_wide() { 2 } else { 1 };
            }
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

        let mut erase: Option<(u16, &crate::attrs::Attrs)> = None;
        for (col, cell) in self
            .cells()
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

            if let Some((prev_col, attrs)) = erase {
                if cell.has_contents() || cell.attrs() != attrs {
                    let new_pos = crate::grid::Pos { row, col: prev_col };
                    crate::term::MoveFromTo::new(prev_pos, new_pos)
                        .write_buf(contents);
                    prev_pos = new_pos;
                    if &prev_attrs != attrs {
                        attrs.write_escape_code_diff(contents, &prev_attrs);
                        prev_attrs = *attrs;
                    }
                    crate::term::EraseChar::new(pos.col - prev_col)
                        .write_buf(contents);
                    erase = None;
                }
            }

            if cell != &default_cell {
                let attrs = cell.attrs();
                if cell.has_contents() {
                    if pos != prev_pos {
                        if !wrapping
                            || prev_pos.row + 1 != pos.row
                            || prev_pos.col
                                < self.cols()
                                    - if cell.is_wide() { 1 } else { 0 }
                            || pos.col != 0
                        {
                            crate::term::MoveFromTo::new(prev_pos, pos)
                                .write_buf(contents);
                        }
                        prev_pos = pos;
                    }

                    if &prev_attrs != attrs {
                        attrs.write_escape_code_diff(contents, &prev_attrs);
                        prev_attrs = *attrs;
                    }

                    contents.extend(cell.contents().as_bytes());
                    prev_pos.col += if cell.is_wide() { 2 } else { 1 };
                } else if erase.is_none() {
                    erase = Some((pos.col, attrs));
                }
            }
        }
        if let Some((prev_col, attrs)) = erase {
            let new_pos = crate::grid::Pos { row, col: prev_col };
            crate::term::MoveFromTo::new(prev_pos, new_pos)
                .write_buf(contents);
            prev_pos = new_pos;
            if &prev_attrs != attrs {
                attrs.write_escape_code_diff(contents, &prev_attrs);
                prev_attrs = *attrs;
            }
            crate::term::ClearRowForward::default().write_buf(contents);
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

        let mut erase: Option<(u16, &crate::attrs::Attrs)> = None;
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

            if let Some((prev_col, attrs)) = erase {
                if cell.has_contents() || cell.attrs() != attrs {
                    let new_pos = crate::grid::Pos { row, col: prev_col };
                    crate::term::MoveFromTo::new(prev_pos, new_pos)
                        .write_buf(contents);
                    prev_pos = new_pos;
                    if &prev_attrs != attrs {
                        attrs.write_escape_code_diff(contents, &prev_attrs);
                        prev_attrs = *attrs;
                    }
                    crate::term::EraseChar::new(pos.col - prev_col)
                        .write_buf(contents);
                    erase = None;
                }
            }

            if cell != prev_cell {
                let attrs = cell.attrs();
                if cell.has_contents() {
                    if pos != prev_pos {
                        if !wrapping
                            || prev_pos.row + 1 != pos.row
                            || prev_pos.col
                                < self.cols()
                                    - if cell.is_wide() { 1 } else { 0 }
                            || pos.col != 0
                        {
                            crate::term::MoveFromTo::new(prev_pos, pos)
                                .write_buf(contents);
                        }
                        prev_pos = pos;
                    }

                    if &prev_attrs != attrs {
                        attrs.write_escape_code_diff(contents, &prev_attrs);
                        prev_attrs = *attrs;
                    }

                    contents.extend(cell.contents().as_bytes());
                    prev_pos.col += if cell.is_wide() { 2 } else { 1 };
                } else if erase.is_none() {
                    erase = Some((pos.col, attrs));
                }
            }
        }
        if let Some((prev_col, attrs)) = erase {
            let new_pos = crate::grid::Pos { row, col: prev_col };
            crate::term::MoveFromTo::new(prev_pos, new_pos)
                .write_buf(contents);
            prev_pos = new_pos;
            if &prev_attrs != attrs {
                attrs.write_escape_code_diff(contents, &prev_attrs);
                prev_attrs = *attrs;
            }
            crate::term::ClearRowForward::default().write_buf(contents);
        }

        (prev_pos, prev_attrs)
    }
}
