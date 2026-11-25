mod helpers;

// Read the logical text across history and the live screen without changing
// the caller's viewport. Soft wraps join; explicit line breaks stay intact.
fn snapshot(mut screen: vt100::Screen) -> String {
    let rows = screen.size().0.get();
    let mut offset = screen.scrollback_len();
    let mut text = String::new();
    while offset > 0 {
        screen.set_scrollback(offset);
        let count = rows.min(u16::try_from(offset).unwrap_or(u16::MAX));
        text.push_str(&screen.contents_between(0, 0, count, 0));
        offset = offset.saturating_sub(usize::from(count));
    }
    screen.set_scrollback(0);
    text.push_str(&screen.contents());
    text
}

#[test]
fn resizing_preserves_output_scrollback_and_soft_wrapped_lines() {
    let mut parser = vt100::Parser::new(
        6.try_into().unwrap(),
        40.try_into().unwrap(),
        5000,
    );
    for number in 0..10 {
        parser.process(
            format!("line {number:02}: abcdefghijklmnopqrstuvwxyz\r\n")
                .as_bytes(),
        );
    }
    parser.process("\x1b[36mproject 界 e\u{301}\x1b[0m $ ".as_bytes());
    let before = snapshot(parser.screen().clone());
    for (rows, cols) in [(4, 10), (8, 60), (2, 3), (6, 40)] {
        parser.set_size(rows.try_into().unwrap(), cols.try_into().unwrap());
        assert_eq!(snapshot(parser.screen().clone()), before);
    }
    parser.process(b"echo preserved");
    assert!(parser.screen().contents().ends_with("$ echo preserved"));
    let (row, _) = parser.screen().cursor_position();
    assert_eq!(
        parser.screen().cell(row, 0).unwrap().fgcolor(),
        vt100::Color::Idx(6)
    );
}

#[test]
fn resizing_rejoins_wide_glyphs_and_preserves_pending_wraps() {
    let mut parser =
        vt100::Parser::new(4.try_into().unwrap(), 8.try_into().unwrap(), 100);
    parser.process("abc界e\u{301}fg".as_bytes());
    for cols in [4, 1, 3, 8] {
        parser.set_size(4.try_into().unwrap(), cols.try_into().unwrap());
        assert_eq!(snapshot(parser.screen().clone()), "abc界e\u{301}fg");
    }
    parser.process(b"h");
    assert_eq!(parser.screen().contents(), "abc界e\u{301}fgh");
}

#[test]
fn repeated_resizing_preserves_blank_lines_spaces_and_unicode() {
    for output in [
        "\r\n\r\nblank lines\r\n\r\nprompt $ ",
        "ab       界 界\r\nprompt $ ",
        "123456789界界e\u{301}😀 tail\r\nprompt $ ",
        "\x1b[41mcolored background   \x1b[0m\r\nprompt $ ",
    ] {
        let mut parser = vt100::Parser::new(
            6.try_into().unwrap(),
            40.try_into().unwrap(),
            500,
        );
        parser.process(output.as_bytes());
        let before = snapshot(parser.screen().clone());
        for cols in (1..=12).chain([40]) {
            parser.set_size(6.try_into().unwrap(), cols.try_into().unwrap());
            assert_eq!(
                snapshot(parser.screen().clone()),
                before,
                "width {cols}, output {output:?}"
            );
        }
        parser.process(b"ok");
        assert!(parser.screen().contents().ends_with("prompt $ ok"));
    }
}

#[test]
fn resizing_keeps_the_scrolled_view_on_its_text() {
    let mut parser = vt100::Parser::new(
        4.try_into().unwrap(),
        20.try_into().unwrap(),
        100,
    );
    for number in 0..20 {
        parser.process(format!("line {number:02}\r\n").as_bytes());
    }
    parser.screen_mut().set_scrollback(8);
    let before = parser.screen().contents();
    parser.set_size(4.try_into().unwrap(), 10.try_into().unwrap());
    assert_eq!(parser.screen().contents(), before);
    parser.set_size(4.try_into().unwrap(), 20.try_into().unwrap());
    assert_eq!(parser.screen().contents(), before);
}

#[test]
fn alternate_screen_resizes_without_replacing_the_primary_history_or_modes() {
    let mut parser = vt100::Parser::new(
        4.try_into().unwrap(),
        20.try_into().unwrap(),
        100,
    );
    parser.process(
        b"primary output\r\n$ \x1b[?1h\x1b[?2004h\x1b[?1049happlication",
    );
    parser.set_size(6.try_into().unwrap(), 8.try_into().unwrap());
    assert!(parser.screen().alternate_screen());
    parser.process(b"\x1b[?1049l");
    assert!(parser.screen().application_cursor());
    assert!(parser.screen().bracketed_paste());
    // Normal output owns its reflow again after the input mode ends.
    parser.process(b"\x1b[?2004l");
    parser.set_size(4.try_into().unwrap(), 20.try_into().unwrap());
    assert_eq!(parser.screen().contents(), "primary output\n$ ");
    parser.process(b"echo");
    assert!(parser.screen().contents().ends_with("$ echo"));
}

#[test]
fn resizing_preserves_a_line_editors_physical_prompt_rows() {
    for callbacks in [false, true] {
        let mut parser = vt100::Parser::new(
            24.try_into().unwrap(),
            60.try_into().unwrap(),
            100,
        );
        parser.process(b"saved output\r\nproject on feature/error-handling\r\n$ \x1b[46Cat 10:37:49\x1b[57D\x1b[?2004h");
        // Captured zsh SIGWINCH redraws: cursor-up counts describe the
        // old physical prompt, before wrapping at the new width.
        for (cols, up) in
            [(30, 1), (12, 2), (50, 3), (100, 1), (30, 1), (80, 2)]
        {
            if callbacks {
                parser.set_size(
                    24.try_into().unwrap(),
                    cols.try_into().unwrap(),
                );
            } else {
                parser.screen_mut().set_size(
                    24.try_into().unwrap(),
                    cols.try_into().unwrap(),
                );
            }
            parser.process(
                format!("\r\r\x1b[{up}A\x1b[Jproject on feature/error-handling\r\n$ ")
                    .as_bytes(),
            );
            if cols >= 30 {
                parser.process(
                    format!(
                        "\x1b[{}Cat 10:37:49\x1b[{}D",
                        cols - 14,
                        cols - 3
                    )
                    .as_bytes(),
                );
            }
            let text = snapshot(parser.screen().clone());
            assert_eq!(
                text.matches("project").count(),
                1,
                "width {cols}: {text:?}"
            );
            assert!(text.starts_with(
                "saved output\nproject on feature/error-handling\n$ "
            ));
            assert_eq!(parser.screen().cursor_position().1, 2);
        }
        // Once the line is accepted, command output reflows normally.
        parser.process(
            b"\x1b[?2004lecho done\r\nabcdefghijklmnopqrstuvwxyz\r\n",
        );
        let before = snapshot(parser.screen().clone());
        parser.set_size(24.try_into().unwrap(), 10.try_into().unwrap());
        parser.set_size(24.try_into().unwrap(), 80.try_into().unwrap());
        assert_eq!(snapshot(parser.screen().clone()), before);
    }
}

#[test]
fn wide_wrap_formatting_round_trips() {
    for prefix in 0..80 {
        for suffix in ["", "\r\nx", "\nx", "\r\n\r\nx"] {
            let input = format!("{}界{suffix}", "a".repeat(prefix));
            helpers::assert_reproduces_state(input.as_bytes());
        }
    }
}

#[test]
fn cursor_motion_before_a_wide_wrap_round_trips() {
    for prefix in 0..80 {
        let before = format!("{}\x1b[1;80H", "a".repeat(prefix));
        helpers::assert_reproduces_state_from(
            "界".as_bytes(),
            before.as_bytes(),
        );
    }
}

#[test]
fn cursor_padding_after_a_pending_wrap_round_trips() {
    let mut parser = helpers::new(2, 1, 5);
    parser.process(b"      ");
    helpers::set_parser_size(&mut parser, 7, 6);
    parser.process(b"\n");
    helpers::set_parser_size(&mut parser, 9, 5);
    assert!(helpers::contents_formatted_reproduces_sized_screen(
        parser.screen()
    ));
}

#[test]
fn a_clipped_continuation_does_not_leave_the_last_row_wrapped() {
    let mut parser = helpers::new(9, 7, 4);
    parser.process("   \u{3000}     ".as_bytes());
    helpers::set_size(parser.screen_mut(), 2, 4);
    parser.process("\u{3000}".as_bytes());
    helpers::set_size(parser.screen_mut(), 1, 10);
    assert!(!parser.screen().row_wrapped(0));
    helpers::set_parser_size(&mut parser, 5, 8);
    assert!(helpers::contents_formatted_reproduces_sized_screen(
        parser.screen()
    ));
}

#[test]
fn erasing_a_wide_continuation_preserves_its_soft_wrap_on_export() {
    for prefix in [0, 1, 20, 78, 79] {
        let before = format!("{}\x1b[1;80H界", "a".repeat(prefix));
        helpers::assert_reproduces_state_from(
            b"\r\x1b[2Kx",
            before.as_bytes(),
        );
    }
}

#[test]
fn a_retained_wide_glyph_can_be_overwritten_at_one_column() {
    let mut parser = helpers::new(11, 5, 1);
    parser.process("\u{3000}\r".as_bytes());
    helpers::set_size(parser.screen_mut(), 9, 1);
    parser.process(b" ");
    assert_eq!(parser.screen().cell(0, 0).unwrap().contents(), " ");
    assert!(!parser.screen().cell(0, 0).unwrap().is_wide());
}

#[test]
fn scrolling_before_reflow_round_trips() {
    let mut parser = helpers::new(2, 3, 5);
    parser.process(b"  \x1b[S ");
    helpers::set_parser_size(&mut parser, 8, 2);
    assert!(helpers::contents_formatted_reproduces_sized_screen(
        parser.screen()
    ));
}

#[test]
fn cursor_positioned_spacing_survives_reflow_at_one_column() {
    for input in [b"\t ".as_slice(), "\u{3000}\x1b[D ".as_bytes()] {
        let mut parser = helpers::new(9, 9, 20);
        parser.process(input);
        let before = snapshot(parser.screen().clone());
        helpers::set_parser_size(&mut parser, 12, 1);
        assert_eq!(snapshot(parser.screen().clone()), before);
        assert!(helpers::contents_formatted_reproduces_sized_screen(
            parser.screen()
        ));
    }
}
