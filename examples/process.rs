use std::io::Read as _;

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

fn process_frames(frames: &[Vec<u8>]) {
    let mut parser = vt100::Parser::new(24, 80);
    for frame in frames {
        parser.process(&frame);
    }
}

fn main() {
    let frames: Vec<Vec<u8>> = read_frames().collect();
    for _ in 1..100 {
        process_frames(&frames);
    }
}
