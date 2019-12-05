use std::convert::TryFrom as _;
use std::io::BufRead as _;
use std::io::Write as _;

#[path = "../tests/helpers/mod.rs"]
mod helpers;

fn main() {
    let name = std::env::args().nth(1).unwrap();
    let _ = std::fs::remove_dir_all(format!("tests/data/fixtures/{}", name));
    std::fs::create_dir_all(format!("tests/data/fixtures/{}", name)).unwrap();

    let inputs =
        std::fs::File::open(format!("tests/data/fixtures/{}.in", name))
            .unwrap();
    let inputs = std::io::BufReader::new(inputs);

    let mut i = 1;
    let mut prev_input = vec![];
    for line in inputs.lines() {
        let line = line.unwrap();

        let input = unhex(line.as_bytes());
        let mut input_file = std::fs::File::create(format!(
            "tests/data/fixtures/{}/{}.typescript",
            name, i
        ))
        .unwrap();
        input_file.write_all(&input).unwrap();

        prev_input.extend(input);
        let mut term = vt100::Parser::default();
        term.process(&prev_input);
        let screen = helpers::FixtureScreen::from_screen(term.screen());

        let output_file = std::fs::File::create(format!(
            "tests/data/fixtures/{}/{}.json",
            name, i
        ))
        .unwrap();
        serde_json::to_writer_pretty(output_file, &screen).unwrap();

        i += 1;
    }
}

fn unhex(s: &[u8]) -> Vec<u8> {
    let mut ret = vec![];
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'\\' {
            match s[i + 1] {
                b'x' => {
                    let upper = s[i + 2];
                    let lower = s[i + 3];
                    ret.push(helpers::hex(upper, lower).unwrap());
                    i += 4;
                }
                b'u' => {
                    assert_eq!(s[i + 2], b'{');
                    let mut digits = vec![];
                    let mut j = i + 3;
                    while s[j] != b'}' {
                        digits.push(s[j]);
                        j += 1;
                    }
                    let digits: Vec<_> = digits
                        .iter()
                        .copied()
                        .skip_while(|x| x == &b'0')
                        .collect();
                    let digits = String::from_utf8(digits).unwrap();
                    let codepoint = u32::from_str_radix(&digits, 16).unwrap();
                    let c = char::try_from(codepoint).unwrap();
                    let mut bytes = [0; 4];
                    ret.extend(c.encode_utf8(&mut bytes).bytes());
                    i = j + 1;
                }
                b'r' => {
                    ret.push(0x0d);
                    i += 2;
                }
                b'n' => {
                    ret.push(0x0a);
                    i += 2;
                }
                b't' => {
                    ret.push(0x09);
                    i += 2;
                }
                _ => panic!("invalid escape"),
            }
        } else {
            ret.push(s[i]);
            i += 1;
        }
    }
    ret
}
