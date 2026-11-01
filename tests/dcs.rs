use vt100::{Callbacks, Parser};

#[derive(Default)]
struct Dcs {
    params: Vec<Vec<u16>>,
    intermediates: Vec<u8>,
    action: char,
    payload: Vec<u8>,
}

impl Callbacks for Dcs {
    fn unhandled_dcs(
        &mut self,
        _: &mut vt100::Screen,
        params: &[&[u16]],
        intermediates: &[u8],
        action: char,
        payload: &[u8],
    ) {
        self.params = params.iter().map(|param| param.to_vec()).collect();
        self.intermediates = intermediates.to_vec();
        self.action = action;
        self.payload = payload.to_vec();
    }
}

#[test]
fn unhandled_dcs() {
    let mut parser = Parser::new_with_callbacks(24, 80, 0, Dcs::default());
    parser.process(b"\x1bP+q696e646e\x1b\\");

    let dcs = parser.callbacks();
    assert_eq!(dcs.params, vec![vec![0]]);
    assert_eq!(dcs.intermediates, b"+");
    assert_eq!(dcs.action, 'q');
    assert_eq!(dcs.payload, b"696e646e");
}

#[test]
fn unhandled_dcs_split_across_process_calls() {
    let mut parser = Parser::new_with_callbacks(24, 80, 0, Dcs::default());
    parser.process(b"\x1bP+q696e");
    assert!(parser.callbacks().payload.is_empty());
    parser.process(b"646e\x1b\\");

    assert_eq!(parser.callbacks().payload, b"696e646e");
}
