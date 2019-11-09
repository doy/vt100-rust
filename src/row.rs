use std::convert::TryInto as _;
use std::io::Write as _;

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

    pub fn clear(&mut self, bgcolor: crate::attrs::Color) {
        for cell in &mut self.cells {
            cell.clear(bgcolor);
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
                // using write! here is significantly slower, for some reason
                // write!(contents, "{}", cell.contents()).unwrap();
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
        attrs: crate::attrs::Attrs,
    ) -> (crate::attrs::Attrs, Option<u16>) {
        let mut prev_was_wide = false;
        let mut prev_attrs = attrs;

        let mut cols = None;
        for cell in self
            .cells()
            .skip(start as usize)
            .take(width.min(self.content_width(start, true)) as usize)
        {
            if prev_was_wide {
                prev_was_wide = false;
                continue;
            }

            let attrs = cell.attrs();
            if &prev_attrs != attrs {
                attrs.write_escape_code_diff(contents, &prev_attrs);
                prev_attrs = *attrs;
            }

            if cell.has_contents() {
                // using write! here is significantly slower, for some reason
                // write!(contents, "{}", cell.contents()).unwrap();
                contents.extend(cell.contents().as_bytes());
            } else if cell.bgcolor() == crate::attrs::Color::Default {
                write!(contents, "{}", crate::term::MoveRight::default())
                    .unwrap();
            } else {
                write!(
                    contents,
                    "{}{}",
                    crate::term::EraseChar::default(),
                    crate::term::MoveRight::default()
                )
                .unwrap();
            }

            prev_was_wide = cell.is_wide();
            cols =
                Some(cols.unwrap_or(0) + if prev_was_wide { 2 } else { 1 });
        }

        (prev_attrs, cols)
    }

    pub fn write_contents_diff<F: for<'a> Fn(&'a mut Vec<u8>)>(
        &self,
        contents: &mut Vec<u8>,
        prev: &Self,
        initial_pos: F,
        start: u16,
        width: u16,
        attrs: crate::attrs::Attrs,
    ) -> (crate::attrs::Attrs, Option<u16>) {
        let mut prev_was_wide = false;
        let mut prev_attrs = attrs;
        let mut skip = 0;
        let mut cols = None;
        let mut initial_pos = Some(initial_pos);

        for (cell, prev_cell) in self
            .cells()
            .zip(prev.cells())
            .skip(start as usize)
            .take(width as usize)
        {
            if prev_was_wide {
                prev_was_wide = false;
                continue;
            }

            if cell == prev_cell {
                prev_was_wide = cell.is_wide();
                skip += if prev_was_wide { 2 } else { 1 };
            } else {
                if let Some(f) = initial_pos.take() {
                    f(contents)
                }
                if skip > 0 {
                    write!(contents, "{}", crate::term::MoveRight::new(skip))
                        .unwrap();
                    cols = Some(cols.unwrap_or(0) + skip);
                    skip = 0;
                }

                let attrs = cell.attrs();
                if &prev_attrs != attrs {
                    attrs.write_escape_code_diff(contents, &prev_attrs);
                    prev_attrs = *attrs;
                }

                if cell.has_contents() {
                    write!(contents, "{}", cell.contents()).unwrap();
                } else {
                    write!(
                        contents,
                        "{}{}",
                        crate::term::EraseChar::default(),
                        crate::term::MoveRight::default()
                    )
                    .unwrap();
                }

                prev_was_wide = cell.is_wide();
                cols = Some(
                    cols.unwrap_or(0) + if prev_was_wide { 2 } else { 1 },
                );
            }
        }

        (prev_attrs, cols)
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
