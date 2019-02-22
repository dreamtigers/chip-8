use sdl2;

use std::time::Duration;
use std::thread;

mod display;
mod keypad;

use chip_8::Chip8;

// Chip-8 Ran at 60 Hz, or 60 frames/second.
// That means each frame lasted 16.7 =~ 17 ms.
const TIME : Duration = Duration::from_millis(17);

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut chip8 = Chip8::new();
    let mut display = display::Display::new(&sdl_context);
    let mut keypad = keypad::Keypad::new(&sdl_context);

    'running : loop {
        let keys = match keypad.poll() {
            Ok(k) => k,
            Err(e) => {
                println!("{}", e);
                break 'running;
            },
        };

        chip8.cycle(keys);

        display.draw(&chip8.screen);

        thread::sleep(TIME);
    }
}
