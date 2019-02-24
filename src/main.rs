use sdl2;
use sdl2::event::Event;

use std::time::Duration;
use std::thread;

mod display;
mod keyboard;

use chip_8::Chip8;

// Chip-8 Ran at 60 Hz, or 60 frames/second.
// That means each frame lasted 16.7 =~ 17 ms.
const TIME : Duration = Duration::from_millis(17);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    let mut display = display::Display::new(&sdl_context);

    'running : loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            };
        };

        keyboard::poll(&event_pump, &mut chip8.keypad);

        chip8.cycle();

        display.draw(&chip8.screen);

        thread::sleep(TIME);
    }
}
