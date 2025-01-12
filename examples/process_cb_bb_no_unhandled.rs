use std::io::Read as _;

struct Callbacks;
impl vt100::Callbacks for Callbacks {
    fn audible_bell(&mut self, screen: &mut vt100::Screen) {
        std::hint::black_box(screen);
    }

    fn visual_bell(&mut self, screen: &mut vt100::Screen) {
        std::hint::black_box(screen);
    }

    fn resize(&mut self, screen: &mut vt100::Screen, request: (u16, u16)) {
        std::hint::black_box((screen, request));
    }

    fn set_window_icon_name(
        &mut self,
        screen: &mut vt100::Screen,
        icon_name: &[u8],
    ) {
        std::hint::black_box((screen, icon_name));
    }

    fn set_window_title(&mut self, screen: &mut vt100::Screen, title: &[u8]) {
        std::hint::black_box((screen, title));
    }
}

fn read_frames() -> impl Iterator<Item = Vec<u8>> {
    (1..=7625).map(|i| {
        let mut file =
            std::fs::File::open(format!("tests/data/crawl/crawl{i}"))
                .unwrap();
        let mut frame = vec![];
        file.read_to_end(&mut frame).unwrap();
        frame
    })
}

fn process_frames(frames: &[Vec<u8>]) {
    let mut parser = vt100::Parser::new_with_callbacks(24, 80, 0, Callbacks);
    for frame in frames {
        parser.process(frame);
    }
}

fn main() {
    let frames: Vec<Vec<u8>> = read_frames().collect();
    let start = std::time::Instant::now();
    let mut i = 0;
    loop {
        i += 1;
        process_frames(&frames);
        if (std::time::Instant::now() - start).as_secs() >= 30 {
            break;
        }
    }
    eprintln!("{i} iterations");
}
