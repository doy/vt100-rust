extern crate vt100;

use std::io::prelude::*;

fn get_file_contents(name: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(name).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    buf
}

fn write_to_screen(chunks: &mut Vec<Vec<u8>>) -> String {
    let mut screen = vt100::Screen::new(37, 193);
    let mut full_chunk = vec![];
    for chunk in chunks.iter_mut() {
        full_chunk.append(chunk);
        let bytes = screen.process(&mut full_chunk);
        full_chunk = full_chunk.split_off(bytes as usize);
    }
    assert_eq!(full_chunk.len(), 0);
    screen.window_contents(0, 0, 36, 192)
}

fn test_splits(filename: &str) {
    let bytes = get_file_contents(filename);
    let len = bytes.len();
    let expected = write_to_screen(&mut vec![bytes.clone()]);
    for i in 0..(len - 1) {
        let bytes_copy = bytes.clone();
        let (start, end) = bytes_copy.split_at(i);
        let mut chunks = vec![start.to_vec(), end.to_vec()];
        let got = write_to_screen(&mut chunks);
        assert!(
            got == expected,
            "failed to render {} when split at byte {}",
            filename,
            i
        );
    }
}

#[test]
fn split_escapes_weechat() {
    test_splits("tests/data/weechat.typescript");
}
