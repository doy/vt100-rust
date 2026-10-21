use crate::term::BufWrite as _;
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug)]
pub struct Grid {
    size: Size,
    pos: Pos,
    saved_pos: Pos,
    rows: Vec<crate::row::Row>,
    scroll_top: u16,
    scroll_bottom: u16,
    origin_mode: bool,
    saved_origin_mode: bool,
    scrollback: Option<ScrollbackState>,
}

#[derive(Clone, Debug)]
struct ScrollbackState {
    rows: alloc::collections::VecDeque<crate::row::Row>,
    capacity: usize,
    offset: usize,
    /// The number of rows of scrollback there *would* be if the scrollback
    /// capacity were unlimited.
    unbounded_len: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Scrollback {
    Disabled,
    Enabled { capacity: usize },
}

impl Grid {
    pub fn new(size: Size, scrollback: Scrollback) -> Self {
        let scrollback = match scrollback {
            Scrollback::Disabled => None,
            Scrollback::Enabled { capacity } => Some(ScrollbackState {
                rows: alloc::collections::VecDeque::new(),
                capacity,
                offset: 0,
                unbounded_len: 0,
            }),
        };
        Self {
            size,
            pos: Pos::default(),
            saved_pos: Pos::default(),
            rows: Vec::new(),
            scroll_top: 0,
            scroll_bottom: size.rows() - 1,
            origin_mode: false,
            saved_origin_mode: false,
            scrollback,
        }
    }

    pub fn allocate_rows(&mut self) {
        if self.rows.is_empty() {
            self.rows.resize_with(usize::from(self.size.rows()), || {
                crate::row::Row::new(self.size.cols())
            });
        }
    }

    pub fn clear(&mut self) {
        self.pos = Pos::default();
        self.saved_pos = Pos::default();
        for row in self.drawing_rows_mut() {
            row.reset();
        }
        self.scroll_top = 0;
        self.scroll_bottom = self.size.rows() - 1;
        self.origin_mode = false;
        self.saved_origin_mode = false;
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn set_size<F>(&mut self, size: Size, on_row: F)
    where
        F: FnMut(&crate::row::Row),
    {
        if size == self.size {
            return;
        }

        // xterm always resets the scroll region and scrollback offset when the
        // screen is resized.
        self.scroll_top = 0;
        self.scroll_bottom = self.size.rows() - 1;
        if let Some(scrollback) = &mut self.scrollback {
            scrollback.offset = 0;
        }

        let new_row = || crate::row::Row::new(size.cols());
        let diff = self.size.rows().abs_diff(size.rows());
        // `rows` might be empty if this is the alternate screen and the rows
        // haven't been allocated (Self::allocate_rows) yet.
        let is_empty = self.rows.is_empty();
        let pos_diff = if is_empty {
            0
        } else if size.rows() < self.size.rows() {
            let num_excess_bottom = self.size.rows() - self.pos.row - 1;
            let num_pushed = diff.saturating_sub(num_excess_bottom);
            self.scroll_up(num_pushed, on_row);
            self.rows.truncate(size.rows().into());
            num_pushed
        } else if let Some(scrollback) = &mut self.scrollback {
            let num_pulled = u16::try_from(scrollback.unbounded_len)
                .unwrap_or(u16::MAX)
                .min(diff);
            // Number of rows actually pulled from scrollback.
            let num_pulled_scrollback =
                scrollback.rows.len().min(num_pulled.into());
            // Number of blank rows inserted at the top, because our scrollback
            // buffer isn't as large as the theoretical amount of scrollback
            // an unbounded terminal would have.
            let num_pulled_blank =
                usize::from(num_pulled) - num_pulled_scrollback;
            let pulled =
                core::iter::repeat_with(new_row)
                    .take(num_pulled_blank)
                    .chain(scrollback.rows.drain(
                        scrollback.rows.len() - num_pulled_scrollback..,
                    ));
            self.rows.splice(0..0, pulled);
            self.rows.resize_with(size.rows().into(), new_row);
            scrollback.unbounded_len -= usize::from(num_pulled);
            num_pulled
        } else {
            self.rows.resize_with(size.rows().into(), new_row);
            0
        };

        if let Some(row) = self.rows.last_mut() {
            // The last row is maintained to always be a valid row
            row.wrap(false);
        }
        for row in &mut self.rows {
            row.resize(size.cols(), crate::Cell::new());
        }

        // Maintain the cursor position relative to the bottom-left corner of
        // the screen.
        if size.rows() < self.size.rows() {
            self.pos.row = self.pos.row.saturating_sub(pos_diff);
            self.saved_pos.row = self
                .saved_pos
                .row
                .saturating_sub(pos_diff)
                .min(size.rows() - 1);
        } else {
            self.pos.row += pos_diff;
            self.saved_pos.row += pos_diff;
        }

        self.pos.col = self.pos.col.min(size.cols() - 1);
        self.saved_pos.col = self.saved_pos.col.min(size.cols() - 1);
        self.scroll_bottom = size.rows() - 1;
        self.size = size;

        if !is_empty {
            debug_assert_eq!(self.rows.len(), usize::from(self.size.rows()));
        }
        debug_assert!(self.pos.row < self.size.rows());
        debug_assert!(self.pos.col < self.size.cols());
        debug_assert!(self.saved_pos.row < self.size.rows());
        debug_assert!(self.saved_pos.col < self.size.cols());
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn set_pos(&mut self, mut pos: Pos) {
        if self.origin_mode {
            pos.row = pos.row.saturating_add(self.scroll_top);
        }
        self.pos = pos;
        self.row_clamp_top(self.origin_mode);
        self.row_clamp_bottom(self.origin_mode);
        self.col_clamp();
    }

    pub fn save_cursor(&mut self) {
        self.saved_pos = self.pos;
        self.saved_origin_mode = self.origin_mode;
    }

    pub fn restore_cursor(&mut self) {
        self.pos = self.saved_pos;
        self.origin_mode = self.saved_origin_mode;
    }

    pub fn visible_rows(&self) -> impl Iterator<Item = &crate::row::Row> {
        self.scrollback
            .as_ref()
            .map(|s| s.rows.iter().skip(s.rows.len() - s.offset))
            .into_iter()
            .flatten()
            .chain(&self.rows)
            .take(self.rows.len())
    }

    pub fn drawing_rows(&self) -> impl Iterator<Item = &crate::row::Row> {
        self.rows.iter()
    }

    pub fn drawing_rows_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut crate::row::Row> {
        self.rows.iter_mut()
    }

    pub fn visible_row(&self, row: u16) -> Option<&crate::row::Row> {
        self.visible_rows().nth(usize::from(row))
    }

    pub fn drawing_row(&self, row: u16) -> Option<&crate::row::Row> {
        self.drawing_rows().nth(usize::from(row))
    }

    pub fn drawing_row_mut(
        &mut self,
        row: u16,
    ) -> Option<&mut crate::row::Row> {
        self.drawing_rows_mut().nth(usize::from(row))
    }

    pub fn current_row_mut(&mut self) -> &mut crate::row::Row {
        self.drawing_row_mut(self.pos.row)
            // we assume self.pos.row is always valid
            .unwrap()
    }

    pub fn visible_cell(&self, pos: Pos) -> Option<&crate::Cell> {
        self.visible_row(pos.row).and_then(|r| r.get(pos.col))
    }

    pub fn drawing_cell(&self, pos: Pos) -> Option<&crate::Cell> {
        self.drawing_row(pos.row).and_then(|r| r.get(pos.col))
    }

    pub fn drawing_cell_mut(&mut self, pos: Pos) -> Option<&mut crate::Cell> {
        self.drawing_row_mut(pos.row)
            .and_then(|r| r.get_mut(pos.col))
    }

    pub fn scrollback_capacity(&self) -> usize {
        self.scrollback.as_ref().map_or(0, |s| s.capacity)
    }

    pub fn scrollback_len(&self) -> usize {
        self.scrollback.as_ref().map_or(0, |s| s.rows.len())
    }

    pub fn scrollback(&self) -> usize {
        self.scrollback.as_ref().map_or(0, |s| s.offset)
    }

    /// Ensures every visible row of scrollback is at least the width of the
    /// screen.
    fn grow_visible_scrollback(scrollback: &mut ScrollbackState, size: Size) {
        let scrollback_len = scrollback.rows.len();
        scrollback
            .rows
            .iter_mut()
            .skip(scrollback_len - scrollback.offset)
            .take(size.rows().into())
            .for_each(|row| {
                row.grow(size.cols(), crate::cell::Cell::new());
            });
    }

    pub fn set_scrollback(&mut self, rows: usize) {
        let Some(scrollback) = &mut self.scrollback else {
            return;
        };
        scrollback.offset = rows.min(scrollback.rows.len());
        Self::grow_visible_scrollback(scrollback, self.size);
    }

    pub fn write_contents(&self, contents: &mut String) {
        let mut wrapping = false;
        for row in self.visible_rows() {
            row.write_contents(contents, 0, self.size.cols(), wrapping);
            if !row.wrapped() {
                contents.push('\n');
            }
            wrapping = row.wrapped();
        }

        while contents.ends_with('\n') {
            contents.truncate(contents.len() - 1);
        }
    }

    pub fn write_contents_formatted(
        &self,
        contents: &mut String,
    ) -> crate::attrs::Attrs {
        crate::term::ClearAttrs.write_buf(contents);
        crate::term::ClearScreen.write_buf(contents);

        let mut prev_attrs = crate::attrs::Attrs::default();
        let mut prev_pos = Pos::default();
        let mut wrapping = false;

        for (i, row) in self.visible_rows().enumerate() {
            // we limit the number of cols to a u16 (see Size), so
            // visible_rows() can never return more rows than will fit
            let i = i.try_into().unwrap();
            let (new_pos, new_attrs) = row.write_contents_formatted(
                contents,
                0,
                self.size.cols(),
                i,
                wrapping,
                Some(prev_pos),
                Some(prev_attrs),
            );
            prev_pos = new_pos;
            prev_attrs = new_attrs;
            wrapping = row.wrapped();
        }

        self.write_cursor_position_formatted(
            contents,
            Some(prev_pos),
            Some(prev_attrs),
        );

        prev_attrs
    }

    pub fn write_contents_formatted_basic(
        &self,
        writer: &mut impl alloc::fmt::Write,
        range: crate::capture::BasicFormattedCaptureRange<'_>,
    ) -> alloc::fmt::Result {
        fn write<'a>(
            rows: impl Iterator<Item = &'a crate::row::Row>,
            writer: &mut impl alloc::fmt::Write,
            state: &mut crate::capture::BasicFormattedCaptureState,
        ) -> alloc::fmt::Result {
            rows.map(crate::capture::RowContents)
                .try_for_each(|row| row.write_formatted_basic(writer, state))
        }
        match range {
            crate::capture::BasicFormattedCaptureRange::Full(state) => {
                if let Some(scrollback) = &self.scrollback {
                    write(scrollback.rows.iter(), writer, state)?;
                }
                write(self.rows.iter(), writer, state)
            }
            crate::capture::BasicFormattedCaptureRange::Visible => write(
                self.visible_rows(),
                writer,
                &mut crate::capture::BasicFormattedCaptureState::new(),
            ),
        }
    }

    pub fn write_contents_diff(
        &self,
        contents: &mut String,
        prev: &Self,
        mut prev_attrs: crate::attrs::Attrs,
    ) -> crate::attrs::Attrs {
        let mut prev_pos = prev.pos;
        let mut wrapping = false;
        let mut prev_wrapping = false;
        for (i, (row, prev_row)) in
            self.visible_rows().zip(prev.visible_rows()).enumerate()
        {
            // we limit the number of cols to a u16 (see Size), so
            // visible_rows() can never return more rows than will fit
            let i = i.try_into().unwrap();
            let (new_pos, new_attrs) = row.write_contents_diff(
                contents,
                prev_row,
                0,
                self.size.cols(),
                i,
                wrapping,
                prev_wrapping,
                prev_pos,
                prev_attrs,
            );
            prev_pos = new_pos;
            prev_attrs = new_attrs;
            wrapping = row.wrapped();
            prev_wrapping = prev_row.wrapped();
        }

        self.write_cursor_position_formatted(
            contents,
            Some(prev_pos),
            Some(prev_attrs),
        );

        prev_attrs
    }

    pub fn write_cursor_position_formatted(
        &self,
        contents: &mut String,
        prev_pos: Option<Pos>,
        prev_attrs: Option<crate::attrs::Attrs>,
    ) {
        let prev_attrs = prev_attrs.unwrap_or_default();
        // writing a character to the last column of a row doesn't wrap the
        // cursor immediately - it waits until the next character is actually
        // drawn. it is only possible for the cursor to have this kind of
        // position after drawing a character though, so if we end in this
        // position, we need to redraw the character at the end of the row.
        if prev_pos != Some(self.pos) && self.pos.col >= self.size.cols() {
            let mut pos = Pos {
                row: self.pos.row,
                col: self.size.cols() - 1,
            };
            if self
                .drawing_cell(pos)
                // we assume self.pos.row is always valid, and self.size.cols
                // - 1 is always a valid column
                .unwrap()
                .is_wide_continuation()
            {
                pos.col = self.size.cols() - 2;
            }
            let cell =
                // we assume self.pos.row is always valid, and self.size.cols
                // - 2 must be a valid column because self.size.cols - 1 is
                // always valid and we just checked that the cell at
                // self.size.cols - 1 is a wide continuation character, which
                // means that the first half of the wide character must be
                // before it
                self.drawing_cell(pos).unwrap();
            if cell.has_contents() {
                if let Some(prev_pos) = prev_pos {
                    crate::term::MoveFromTo::new(prev_pos, pos)
                        .write_buf(contents);
                } else {
                    crate::term::MoveTo::new(pos).write_buf(contents);
                }
                cell.attrs().write_escape_code_diff(contents, &prev_attrs);
                contents.push_str(cell.contents());
                prev_attrs.write_escape_code_diff(contents, cell.attrs());
            } else {
                // if the cell doesn't have contents, we can't have gotten
                // here by drawing a character in the last column. this means
                // that as far as i'm aware, we have to have reached here from
                // a newline when we were already after the end of an earlier
                // row. in the case where we are already after the end of an
                // earlier row, we can just write a few newlines, otherwise we
                // also need to do the same as above to get ourselves to after
                // the end of a row.
                let mut found = false;
                for i in (0..self.pos.row).rev() {
                    pos.row = i;
                    pos.col = self.size.cols() - 1;
                    if self
                        .drawing_cell(pos)
                        // i is always less than self.pos.row, which we assume
                        // to be always valid, so it must also be valid.
                        // self.size.cols - 1 is always a valid col.
                        .unwrap()
                        .is_wide_continuation()
                    {
                        pos.col = self.size.cols() - 2;
                    }
                    let cell = self
                        .drawing_cell(pos)
                        // i is always less than self.pos.row, which we assume
                        // to be always valid, so it must also be valid.
                        // self.size.cols - 2 is valid because self.size.cols
                        // - 1 is always valid, and col gets set to
                        // self.size.cols - 2 when the cell at self.size.cols
                        // - 1 is a wide continuation character, meaning that
                        // the first half of the wide character must be before
                        // it
                        .unwrap();
                    if cell.has_contents() {
                        if let Some(prev_pos) = prev_pos {
                            if prev_pos.row != i
                                || prev_pos.col < self.size.cols()
                            {
                                crate::term::MoveFromTo::new(prev_pos, pos)
                                    .write_buf(contents);
                                cell.attrs().write_escape_code_diff(
                                    contents,
                                    &prev_attrs,
                                );
                                contents.push_str(cell.contents());
                                prev_attrs.write_escape_code_diff(
                                    contents,
                                    cell.attrs(),
                                );
                            }
                        } else {
                            crate::term::MoveTo::new(pos).write_buf(contents);
                            cell.attrs().write_escape_code_diff(
                                contents,
                                &prev_attrs,
                            );
                            contents.push_str(cell.contents());
                            prev_attrs.write_escape_code_diff(
                                contents,
                                cell.attrs(),
                            );
                        }
                        contents.push_str(
                            &"\n".repeat(usize::from(self.pos.row - i)),
                        );
                        found = true;
                        break;
                    }
                }

                // this can happen if you get the cursor off the end of a row,
                // and then do something to clear the end of the current row
                // without moving the cursor (IL, DL, ED, EL, etc). we know
                // there can't be something in the last column because we
                // would have caught that above, so it should be safe to
                // overwrite it.
                if !found {
                    pos = Pos {
                        row: self.pos.row,
                        col: self.size.cols() - 1,
                    };
                    if let Some(prev_pos) = prev_pos {
                        crate::term::MoveFromTo::new(prev_pos, pos)
                            .write_buf(contents);
                    } else {
                        crate::term::MoveTo::new(pos).write_buf(contents);
                    }
                    contents.push(' ');
                    // we know that the cell has no contents, but it still may
                    // have drawing attributes (background color, etc)
                    let end_cell = self
                        .drawing_cell(pos)
                        // we assume self.pos.row is always valid, and
                        // self.size.cols - 1 is always a valid column
                        .unwrap();
                    end_cell
                        .attrs()
                        .write_escape_code_diff(contents, &prev_attrs);
                    crate::term::SaveCursor.write_buf(contents);
                    crate::term::Backspace.write_buf(contents);
                    crate::term::EraseChar::new(1).write_buf(contents);
                    crate::term::RestoreCursor.write_buf(contents);
                    prev_attrs
                        .write_escape_code_diff(contents, end_cell.attrs());
                }
            }
        } else if let Some(prev_pos) = prev_pos {
            crate::term::MoveFromTo::new(prev_pos, self.pos)
                .write_buf(contents);
        } else {
            crate::term::MoveTo::new(self.pos).write_buf(contents);
        }
    }

    pub fn erase_all(&mut self, attrs: crate::attrs::Attrs) {
        for row in self.drawing_rows_mut() {
            row.clear(attrs);
        }
    }

    pub fn erase_all_forward(&mut self, attrs: crate::attrs::Attrs) {
        let pos = self.pos;
        for row in self.drawing_rows_mut().skip(usize::from(pos.row) + 1) {
            row.clear(attrs);
        }

        self.erase_row_forward(attrs);
    }

    pub fn erase_all_backward(&mut self, attrs: crate::attrs::Attrs) {
        let pos = self.pos;
        for row in self.drawing_rows_mut().take(usize::from(pos.row)) {
            row.clear(attrs);
        }

        self.erase_row_backward(attrs);
    }

    pub fn erase_all_scrollback<F>(&mut self, mut on_row: F)
    where
        F: FnMut(&crate::row::Row),
    {
        if let Some(scrollback) = &mut self.scrollback {
            for row in scrollback.rows.drain(..) {
                on_row(&row);
            }
            scrollback.unbounded_len = 0;
            scrollback.offset = 0;
        }
    }

    pub fn erase_row(&mut self, attrs: crate::attrs::Attrs) {
        self.current_row_mut().clear(attrs);
    }

    pub fn erase_row_forward(&mut self, attrs: crate::attrs::Attrs) {
        let size = self.size;
        let pos = self.pos;
        let row = self.current_row_mut();
        for col in pos.col..size.cols() {
            row.erase(col, attrs);
        }
    }

    pub fn erase_row_backward(&mut self, attrs: crate::attrs::Attrs) {
        let size = self.size;
        let pos = self.pos;
        let row = self.current_row_mut();
        for col in 0..=pos.col.min(size.cols() - 1) {
            row.erase(col, attrs);
        }
    }

    pub fn insert_cells(&mut self, count: u16) {
        let pos = self.pos;
        let row = self.current_row_mut();
        row.insert(pos.col, count);
    }

    pub fn delete_cells(&mut self, count: u16) {
        let size = self.size;
        let pos = self.pos;
        let row = self.current_row_mut();
        row.remove(pos.col..pos.col.saturating_add(count).min(size.cols()));
    }

    pub fn erase_cells(&mut self, count: u16, attrs: crate::attrs::Attrs) {
        let size = self.size;
        let pos = self.pos;
        let row = self.current_row_mut();
        for col in pos.col..((pos.col.saturating_add(count)).min(size.cols()))
        {
            row.erase(col, attrs);
        }
    }

    pub fn insert_lines(&mut self, count: u16) {
        if count == 0 {
            return;
        }
        if !(self.scroll_top..=self.scroll_bottom).contains(&self.pos.row) {
            return;
        }
        let between = &mut self.rows
            [usize::from(self.pos.row)..=usize::from(self.scroll_bottom)];
        let num_between = between.len();
        let num_removed = usize::from(count).min(num_between);
        between.rotate_right(num_removed);
        for row in &mut between[..num_removed] {
            row.reset();
        }
        // self.scroll_bottom is maintained to always be a valid row
        self.rows[usize::from(self.scroll_bottom)].wrap(false);
    }

    pub fn delete_lines(&mut self, count: u16) {
        if !(self.scroll_top..=self.scroll_bottom).contains(&self.pos.row) {
            return;
        }
        let between = &mut self.rows
            [usize::from(self.pos.row)..=usize::from(self.scroll_bottom)];
        let num_between = between.len();
        let num_removed = usize::from(count).min(num_between);
        for row in &mut between[..num_removed] {
            row.reset();
        }
        between.rotate_left(num_removed);
    }

    pub fn scroll_up<F>(&mut self, count: u16, mut on_row: F)
    where
        F: FnMut(&crate::row::Row),
    {
        // This can be true even when `self.scrollback` is `None`; it controls
        // whether we call the `on_row` callback.
        let scrollback_enabled = self.scroll_top == 0;
        let between = &mut self.rows
            [usize::from(self.scroll_top)..=usize::from(self.scroll_bottom)];
        let num_between = between.len();
        let count = usize::from(count).min(num_between);

        // Number of to-be-removed rows.
        let num_removed = count.min(num_between);

        if let Some(scrollback) =
            self.scrollback.as_mut().filter(|_| scrollback_enabled)
        {
            scrollback.unbounded_len =
                scrollback.unbounded_len.saturating_add(num_removed);

            // Number of rows that will be pushed to the scrollback.
            let num_pushed = count.min(scrollback.capacity);

            // Scrollback length is capped at `scrollback.capacity`. These rows
            // will be lost, so just remove them now to avoid allocating.
            let scrollback_overflow = (scrollback.rows.len() + num_pushed)
                .saturating_sub(scrollback.capacity);
            for row in scrollback.rows.drain(0..scrollback_overflow) {
                on_row(&row);
            }

            // Number of rows that will simply be reset rather than pushed to
            // the scrollback.
            let num_reset = num_removed - num_pushed;

            for row in &mut between[..num_reset] {
                on_row(row);
                row.reset();
            }

            scrollback.rows.extend(
                between[num_reset..num_removed]
                    .iter_mut()
                    .map(crate::row::Row::take_trimmed),
            );

            if scrollback.offset > 0 {
                scrollback.offset = scrollback
                    .offset
                    .saturating_add(count)
                    .min(scrollback.rows.len());
            }
            Self::grow_visible_scrollback(scrollback, self.size);
        } else {
            for row in &mut between[..num_removed] {
                if scrollback_enabled {
                    on_row(row);
                }
                row.reset();
            }
        }

        // Finally, rotate the rows so the "removed" ones (which have now been
        // reset to empty) are at the bottom.
        between.rotate_left(num_removed);
    }

    pub fn scroll_down(&mut self, count: u16) {
        if count == 0 {
            return;
        }
        let between = &mut self.rows
            [usize::from(self.scroll_top)..=usize::from(self.scroll_bottom)];
        let num_between = between.len();
        let num_removed = usize::from(count).min(num_between);
        between.rotate_right(num_removed);
        for row in &mut between[..num_removed] {
            row.reset();
        }
        // self.scroll_bottom is maintained to always be a valid row
        self.rows[usize::from(self.scroll_bottom)].wrap(false);
    }

    pub fn set_scroll_region(&mut self, top: u16, bottom: u16) {
        let bottom = bottom.min(self.size().rows() - 1);
        if top < bottom {
            self.scroll_top = top;
            self.scroll_bottom = bottom;
        } else {
            self.scroll_top = 0;
            self.scroll_bottom = self.size().rows() - 1;
        }
        self.pos.row = self.scroll_top;
        self.pos.col = 0;
    }

    fn in_scroll_region(&self) -> bool {
        self.pos.row >= self.scroll_top && self.pos.row <= self.scroll_bottom
    }

    pub fn set_origin_mode(&mut self, mode: bool) {
        self.origin_mode = mode;
        self.set_pos(Pos { row: 0, col: 0 });
    }

    pub fn row_inc_clamp(&mut self, count: u16) {
        let in_scroll_region = self.in_scroll_region();
        self.pos.row = self.pos.row.saturating_add(count);
        self.row_clamp_bottom(in_scroll_region);
    }

    pub fn row_inc_scroll<F>(&mut self, count: u16, on_row: F) -> u16
    where
        F: FnMut(&crate::row::Row),
    {
        let in_scroll_region = self.in_scroll_region();
        self.pos.row = self.pos.row.saturating_add(count);
        let lines = self.row_clamp_bottom(in_scroll_region);
        if in_scroll_region {
            self.scroll_up(lines, on_row);
            lines
        } else {
            0
        }
    }

    pub fn row_dec_clamp(&mut self, count: u16) {
        let in_scroll_region = self.in_scroll_region();
        self.pos.row = self.pos.row.saturating_sub(count);
        self.row_clamp_top(in_scroll_region);
    }

    pub fn row_dec_scroll(&mut self, count: u16) {
        let in_scroll_region = self.in_scroll_region();
        // need to account for clamping by both row_clamp_top and by
        // saturating_sub
        let extra_lines = count.saturating_sub(self.pos.row);
        self.pos.row = self.pos.row.saturating_sub(count);
        let lines = self.row_clamp_top(in_scroll_region);
        self.scroll_down(lines + extra_lines);
    }

    pub fn row_set(&mut self, i: u16) {
        self.pos.row = i;
        self.row_clamp();
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
        self.pos.col = self.pos.col.saturating_add(8);
        self.col_clamp();
    }

    pub fn col_set(&mut self, i: u16) {
        self.pos.col = i;
        self.col_clamp();
    }

    pub fn col_wrap<F>(&mut self, width: u16, wrap: bool, on_row: F)
    where
        F: FnMut(&crate::row::Row),
    {
        if self.pos.col <= self.size.cols() - width {
            // we haven't reached the end of the row yet
            return;
        }

        let mut prev_pos = self.pos;
        self.pos.col = 0;
        let scrolled = self.row_inc_scroll(1, on_row);
        if prev_pos.row == 0 && scrolled > 0 {
            // we're trying to scroll but we're already at the top of the
            // grid - this can happen when the grid height is set to 1. in
            // this case, we want to do the scrolling (via row_inc_scroll)
            // but don't want to set the wrap state (since it never makes
            // sense on a 1-height grid).
            return;
        }

        prev_pos.row -= scrolled;
        let new_pos = self.pos;
        self.drawing_row_mut(prev_pos.row)
            // we assume self.pos.row is always valid, and so prev_pos.row
            // must be valid because it is always less than or equal to
            // self.pos.row
            .unwrap()
            .wrap(wrap && prev_pos.row + 1 == new_pos.row);
    }

    fn row_clamp_top(&mut self, limit_to_scroll_region: bool) -> u16 {
        if limit_to_scroll_region && self.pos.row < self.scroll_top {
            let rows = self.scroll_top - self.pos.row;
            self.pos.row = self.scroll_top;
            rows
        } else {
            0
        }
    }

    fn row_clamp_bottom(&mut self, limit_to_scroll_region: bool) -> u16 {
        let bottom = if limit_to_scroll_region {
            self.scroll_bottom
        } else {
            self.size.rows() - 1
        };
        if self.pos.row > bottom {
            let rows = self.pos.row - bottom;
            self.pos.row = bottom;
            rows
        } else {
            0
        }
    }

    fn row_clamp(&mut self) {
        if self.pos.row > self.size.rows() - 1 {
            self.pos.row = self.size.rows() - 1;
        }
    }

    fn col_clamp(&mut self) {
        if self.pos.col > self.size.cols() - 1 {
            self.pos.col = self.size.cols() - 1;
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Size {
    pub rows: core::num::NonZeroU16,
    pub cols: core::num::NonZeroU16,
}

impl Size {
    pub fn rows(self) -> u16 {
        self.rows.get()
    }

    pub fn cols(self) -> u16 {
        self.cols.get()
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Pos {
    pub row: u16,
    pub col: u16,
}
