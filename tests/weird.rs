mod helpers;

#[test]
fn intermediate_control() {
    helpers::fixture("intermediate_control");
}

#[test]
fn many_empty_params() {
    let mut parser = vt100::Parser::default();
    parser.process(b"\x1b[::::::::::::::::::::::::::::::::@");
    parser.process(b"\x1b[::::::::::::::::::::::::::::::::H");
    parser.process(b"\x1b[::::::::::::::::::::::::::::::::r");
}
