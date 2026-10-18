#[test]
fn setting_same_size_preserves_pending_wrap() {
    let mut parser = vt100::Parser::new(2, 8, 0);
    parser.process(b"12345678");
    assert_eq!(parser.screen().cursor_position(), (0, 8));

    parser.screen_mut().set_size(2, 8);
    assert_eq!(parser.screen().cursor_position(), (0, 8));
    parser.process(b"9");

    assert_eq!(parser.screen().contents(), "123456789");
    assert_eq!(parser.screen().cursor_position(), (1, 1));
}

#[test]
fn shrinking_height_keeps_the_cursor_line() {
    let mut parser = vt100::Parser::new(2, 8, 0);
    parser.process(b"top\r\nbottom");

    parser.screen_mut().set_size(1, 8);

    assert_eq!(parser.screen().contents(), "bottom");
    assert_eq!(parser.screen().cursor_position(), (0, 6));
}

#[test]
fn shrinking_alternate_screen_keeps_the_cursor_line() {
    let mut parser = vt100::Parser::new(2, 8, 0);
    parser.process(b"\x1b[?1049htop\r\nbottom");

    parser.screen_mut().set_size(1, 8);

    assert_eq!(parser.screen().contents(), "bottom");
    assert_eq!(parser.screen().cursor_position(), (0, 6));
}

#[test]
fn shrinking_then_growing_height_restores_scrollback() {
    let mut parser = vt100::Parser::new(2, 8, 10);
    parser.process(b"top\r\nbottom");

    parser.screen_mut().set_size(1, 8);
    parser.screen_mut().set_size(2, 8);

    assert_eq!(parser.screen().contents(), "top\nbottom");
    assert_eq!(parser.screen().cursor_position(), (1, 6));
}

#[test]
fn growing_replaces_discarded_history_with_blank_rows() {
    for (capacity, expected) in [(0, "\n\nbottom"), (1, "\nmid\nbottom")] {
        let mut parser = vt100::Parser::new(3, 8, capacity);
        parser.process(b"top\r\nmid\r\nbottom");
        parser.screen_mut().set_size(1, 8);
        parser.screen_mut().set_size(3, 8);

        assert_eq!(parser.screen().contents(), expected);
        assert_eq!(parser.screen().cursor_position(), (2, 6));
        parser.screen_mut().set_size(4, 8);
        assert_eq!(parser.screen().contents(), expected);
        assert_eq!(parser.screen().cursor_position(), (2, 6));
    }
}

#[test]
fn growing_alternate_screen_does_not_restore_history() {
    let mut parser = vt100::Parser::new(2, 8, 10);
    parser.process(b"\x1b[?1049htop\r\nbottom");
    parser.screen_mut().set_size(1, 8);
    parser.screen_mut().set_size(2, 8);

    assert_eq!(parser.screen().contents(), "bottom");
    assert_eq!(parser.screen().cursor_position(), (0, 6));
}

#[test]
fn shrinking_discards_bottom_rows_before_scrolling() {
    let mut parser = vt100::Parser::new(4, 8, 10);
    parser.process(b"top\r\nkeep\r\ndiscard\x1b[2;1H");
    parser.screen_mut().set_size(2, 8);
    parser.screen_mut().set_size(4, 8);

    assert_eq!(parser.screen().contents(), "top\nkeep");
    assert_eq!(parser.screen().cursor_position(), (1, 0));
}

#[test]
fn resizing_moves_saved_cursor_with_its_line() {
    let mut parser = vt100::Parser::new(3, 8, 10);
    parser.process(b"top\r\nmid\x1b7\r\nbottom");
    parser.screen_mut().set_size(2, 8);
    parser.process(b"\x1b8");
    assert_eq!(parser.screen().cursor_position(), (0, 3));
    parser.screen_mut().set_size(3, 8);
    parser.process(b"\x1b8");
    assert_eq!(parser.screen().cursor_position(), (1, 3));
}

#[test]
fn resizing_preserves_both_screens_and_pending_parser_state() {
    let mut parser = vt100::Parser::new(2, 8, 10);
    parser.process(b"top\r\nmain\x1b[?1049htop\r\nalt\x1b[3");
    parser.screen_mut().set_size(1, 8);
    parser.process(b"1m");
    assert_eq!(parser.screen().fgcolor(), vt100::Color::Idx(1));
    assert_eq!(parser.screen().contents(), "alt");
    parser.process(b"\x1b[?1049l");
    assert_eq!(parser.screen().contents(), "main");
    assert_eq!(parser.screen().cursor_position(), (0, 4));
}

#[test]
fn growing_after_clearing_history_does_not_restore_blank_rows() {
    let mut parser = vt100::Parser::new(2, 8, 0);
    parser.process(b"top\r\nbottom");
    parser.screen_mut().set_size(1, 8);
    parser.screen_mut().clear_scrollback();
    parser.screen_mut().set_size(2, 8);

    assert_eq!(parser.screen().contents(), "bottom");
    assert_eq!(parser.screen().cursor_position(), (0, 6));
}

#[test]
fn growing_and_narrowing_repairs_restored_wide_cells() {
    let mut parser = vt100::Parser::new(2, 8, 10);
    parser.process("aあ\r\nbottom".as_bytes());
    parser.screen_mut().set_size(1, 8);
    parser.screen_mut().set_size(2, 2);
    parser.process(b"\x1b[1;2H\x1b[K");

    assert_eq!(parser.screen().contents(), "a\nbo");
}

#[test]
fn resizing_resets_scroll_region_and_scrollback_position() {
    let mut parser = vt100::Parser::new(3, 8, 10);
    parser.process(b"old\r\ntop\r\nmid\r\nbottom\x1b[2;3r\x1b[3;1H");
    parser.screen_mut().set_scrollback(1);
    parser.screen_mut().set_size(2, 8);
    assert_eq!(parser.screen().scrollback(), 0);
    assert_eq!(parser.screen().contents(), "mid\nbottom");
    parser.process(b"\x1b[2;1H\n");
    assert_eq!(parser.screen().contents(), "bottom");
}
