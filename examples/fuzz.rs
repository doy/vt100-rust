use std::io::Read as _;

#[path = "../tests/helpers/mod.rs"]
mod helpers;

fn main() {
    let mut vt_base = vt100::Parser::default();
    let mut vt_diff = vt100::Parser::default();
    let mut prev_screen = vt_base.screen().clone();
    let empty_screen = vt100::Parser::default().screen().clone();
    let mut idx = 0;
    loop {
        let mut byte = [0];
        match std::io::stdin().read(&mut byte) {
            Ok(bytes) => {
                if bytes != 1 {
                    break;
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
        vt_base.process(&byte);

        let mut vt_full = vt100::Parser::default();
        vt_full.process(&vt_base.screen().state_formatted());
        vt_full.process(&vt_base.screen().bells_diff(&empty_screen));
        assert!(
            helpers::compare_screens(vt_full.screen(), vt_base.screen()),
            "{}: full",
            idx,
        );

        let mut vt_diff_empty = vt100::Parser::default();
        vt_diff_empty.process(&vt_base.screen().state_diff(&empty_screen));
        vt_diff_empty.process(&vt_base.screen().bells_diff(&empty_screen));
        assert!(
            helpers::compare_screens(
                vt_diff_empty.screen(),
                vt_base.screen()
            ),
            "{}: diff-empty",
            idx,
        );

        vt_diff.process(&vt_base.screen().state_diff(&prev_screen));
        vt_diff.process(&vt_base.screen().bells_diff(&empty_screen));
        prev_screen = vt_base.screen().clone();
        assert!(
            helpers::compare_screens(vt_diff.screen(), vt_base.screen()),
            "{}: diff",
            idx,
        );

        let mut vt_rows = vt100::Parser::default();
        let mut wrapped = false;
        for (idx, row) in vt_base.screen().rows_formatted(0, 80).enumerate() {
            vt_rows.process(b"\x1b[m");
            if !wrapped {
                vt_rows.process(format!("\x1b[{}H", idx + 1).as_bytes());
            }
            vt_rows.process(&row);
            wrapped = vt_base.screen().row_wrapped(idx.try_into().unwrap());
        }
        vt_rows.process(&vt_base.screen().cursor_state_formatted());
        vt_rows.process(&vt_base.screen().attributes_formatted());
        vt_rows.process(&vt_base.screen().input_mode_formatted());
        vt_rows.process(&vt_base.screen().title_formatted());
        vt_rows.process(&vt_base.screen().bells_diff(&empty_screen));
        assert!(
            helpers::compare_screens(vt_rows.screen(), vt_base.screen()),
            "{}: rows",
            idx,
        );

        idx += 1;
    }
}
