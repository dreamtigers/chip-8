use std::time::Duration;
use std::thread;

use chip_8::Chip8;

// Chip-8 Ran at 60 Hz, or 60 frames/second.
// That means each frame lasted 16.7 =~ 17 ms.
const TIME = Duration::from_millis(17);

fn main() {
    let chip8 = Chip8::new();

    loop {
        thread::sleep(TIME);

        chip8.cycle();

        chip8.draw();

        // chip8.set_keys();
    }
}
