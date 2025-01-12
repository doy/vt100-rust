mod helpers;

#[test]
fn title_icon_name() {
    #[derive(Default)]
    struct Window {
        title: String,
        icon_name: String,
    }
    impl vt100::Callbacks for Window {
        fn set_window_icon_name(
            &mut self,
            _: &mut vt100::Screen,
            icon_name: &[u8],
        ) {
            self.icon_name =
                std::str::from_utf8(icon_name).unwrap().to_string();
        }
        fn set_window_title(&mut self, _: &mut vt100::Screen, title: &[u8]) {
            self.title = std::str::from_utf8(title).unwrap().to_string();
        }
    }

    let mut parser =
        vt100::Parser::new_with_callbacks(24, 80, 0, Window::default());
    assert_eq!(parser.callbacks().icon_name, "");
    assert_eq!(parser.callbacks().title, "");
    parser.process(b"\x1b]1;icon_name\x07");
    assert_eq!(parser.callbacks().icon_name, "icon_name");
    assert_eq!(parser.callbacks().title, "");
    parser.process(b"\x1b]2;title\x07");
    assert_eq!(parser.callbacks().icon_name, "icon_name");
    assert_eq!(parser.callbacks().title, "title");
    parser.process(b"\x1b]0;both\x07");
    assert_eq!(parser.callbacks().icon_name, "both");
    assert_eq!(parser.callbacks().title, "both");
}

#[test]
fn unknown_osc() {
    helpers::fixture("unknown_osc");
}
