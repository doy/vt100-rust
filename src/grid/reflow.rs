//! Resize the primary grid without discarding the ends of its lines.

use super::{Grid, Pos, Size};
use crate::{Cell, row::Row};
use alloc::vec::Vec;

impl Grid {
    pub(super) fn reflow<F>(&mut self, size: Size, mut on_row: F)
    where
        F: FnMut(&Row),
    {
        let scrollback = self.scrollback.as_mut().unwrap();
        let history_len = scrollback.rows.len();
        // Map the live cursor, saved cursor and the top of a scrolled view
        // through the same cell stream as the text.
        let positions = [
            (
                history_len + usize::from(self.pos.row),
                usize::from(self.pos.col),
            ),
            (
                history_len + usize::from(self.saved_pos.row),
                usize::from(self.saved_pos.col),
            ),
            (history_len.saturating_sub(scrollback.offset), 0),
        ];
        let offset = scrollback.offset;
        let mut original: Vec<_> = scrollback.rows.drain(..).collect();
        original.append(&mut self.rows);
        // Unused space below the cursor is viewport padding, not output.
        let keep = original
            .iter()
            .rposition(|row| !row.is_blank())
            .map_or(0, |row| row + 1)
            .max(positions[0].0 + 1)
            .max(positions[1].0 + 1);
        original.truncate(keep);

        let width = usize::from(size.cols());
        let mut rows = Vec::new();
        let mut current = Row::new(size.cols());
        let mut col = 0;
        let mut mapped = [(0, 0); 3];
        let mut original = original.into_iter().peekable();
        let mut row_index = 0;
        while let Some(row) = original.next() {
            let wrapped = row.wrapped();
            // A wide glyph that doesn't fit leaves one padding cell before
            // wrapping. That padding must not become a space inside the line.
            let wide_next = wrapped
                && original
                    .peek()
                    .and_then(|row| row.get(0))
                    .is_some_and(Cell::is_wide);
            let cells = row.into_cells();
            let mut len = if wrapped {
                cells.len()
            } else {
                cells
                    .iter()
                    .rposition(|cell| *cell != Cell::DEFAULT)
                    .map_or(0, |col| col + 1)
            };
            if wide_next && cells.last() == Some(&Cell::DEFAULT) {
                len = len.saturating_sub(1);
            }
            let contents_len = len;
            for &(row, column) in &positions[..2] {
                if row == row_index {
                    len = len.max(column.min(cells.len()));
                }
            }
            let mut source_col = 0;
            while source_col < len {
                let cell = &cells[source_col];
                if cell.is_wide_continuation() {
                    source_col += 1;
                    continue;
                }
                let cell_width = if cell.is_wide() { 2 } else { 1 };
                if col == width
                    || (cell_width == 2 && col + 2 > width && col > 0)
                {
                    // Do not turn trailing cursor-only padding into a logical
                    // continuation; preserve spacing that leads to later text.
                    current.wrap(
                        wrapped
                            || !current.is_blank()
                            || source_col < contents_len,
                    );
                    rows.push(current);
                    current = Row::new(size.cols());
                    col = 0;
                }
                for (index, &(row, column)) in positions.iter().enumerate() {
                    if row == row_index
                        && (source_col..source_col + cell_width)
                            .contains(&column)
                    {
                        mapped[index] =
                            (rows.len(), col + column - source_col);
                    }
                }
                let mut moved = cell.clone();
                if moved == Cell::DEFAULT && source_col < contents_len {
                    // Internal cursor-positioned spacing remains part of the
                    // logical line, even when it fills a new physical row.
                    moved.set(' ', *cell.attrs());
                }
                *current.get_mut(col.try_into().unwrap()).unwrap() = moved;
                if cell_width == 2 && width > 1 {
                    let mut continuation = Cell::new();
                    continuation.set_wide_continuation(true);
                    *current
                        .get_mut((col + 1).try_into().unwrap())
                        .unwrap() = continuation;
                }
                col += cell_width.min(width);
                source_col += cell_width;
            }
            for (index, &(row, column)) in positions.iter().enumerate() {
                if row == row_index && column >= len {
                    mapped[index] = (rows.len(), col);
                }
            }
            if !wrapped {
                rows.push(current);
                current = Row::new(size.cols());
                col = 0;
            }
            row_index += 1;
        }
        if col > 0 {
            rows.push(current);
        }

        let height = usize::from(size.rows());
        // Keep the active cursor visible even when text or a saved cursor
        // extends below it. Rows below the resulting viewport are discarded,
        // matching the height-only resize path.
        let start = rows.len().saturating_sub(height).min(mapped[0].0);
        self.rows = rows.split_off(start);
        self.rows.resize_with(height, || Row::new(size.cols()));
        // A clipped continuation below the viewport no longer exists. Match
        // the height-only resize path's invariant for the final row.
        self.rows.last_mut().unwrap().wrap(false);
        let dropped = rows.len().saturating_sub(scrollback.capacity);
        for row in &rows[..dropped] {
            on_row(row);
        }
        scrollback.rows = rows.into_iter().skip(dropped).collect();
        scrollback.unbounded_len = start;
        scrollback.offset = if offset == 0 {
            0
        } else {
            start.saturating_sub(mapped[2].0).min(scrollback.rows.len())
        };
        let position = |(row, col): (usize, usize)| Pos {
            row: row
                .saturating_sub(start)
                .min(height - 1)
                .try_into()
                .unwrap(),
            col: col.min(width - 1).try_into().unwrap(),
        };
        self.pos = position(mapped[0]);
        self.saved_pos = position(mapped[1]);
        // A cursor immediately after the last column carries a pending wrap.
        if mapped[0].1 == width {
            self.pos.col = size.cols();
        }
        self.scroll_top = 0;
        self.scroll_bottom = size.rows() - 1;
        self.size = size;
    }
}
