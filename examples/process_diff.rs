use std::io::{Read as _, Write as _};

fn read_frames() -> impl Iterator<Item = Vec<u8>> {
    (1..=7625).map(|i| {
        let mut file =
            std::fs::File::open(format!("tests/data/crawl/crawl{}", i))
                .unwrap();
        let mut frame = vec![];
        file.read_to_end(&mut frame).unwrap();
        frame
    })
}

fn draw_frames(frames: &[Vec<u8>]) {
    let mut stdout = std::io::stdout();
    let mut parser = vt100::Parser::new(24, 80);
    let mut screen = parser.screen().clone();
    for frame in frames {
        parser.process(&frame);
        let new_screen = parser.screen().clone();
        let diff = new_screen.contents_diff(&screen);
        stdout.write_all(&diff).unwrap();
        screen = new_screen;
    }
}

fn main() {
    let frames: Vec<Vec<u8>> = read_frames().collect();
    for _ in 1..10 {
        draw_frames(&frames);
    }
}
