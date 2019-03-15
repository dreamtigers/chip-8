use sdl2;
use sdl2::event::Event;

use std::time::Duration;
use std::thread;

use chip_8::Chip8;

mod audio;
mod display;
mod keyboard;

// Chip-8 Ran at 60 Hz, or 60 frames/second.
// That means each frame lasted 16.7 =~ 17 ms.
const TIME : Duration = Duration::from_millis(17);

fn main() {
    let     sdl_context = sdl2::init().unwrap();
    let mut event_pump  = sdl_context.event_pump().unwrap();

    let mut chip8   = Chip8::new();
    let     audio   = audio::Audio::new(&sdl_context);
    let mut display = display::Display::new(&sdl_context);

    'running : loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            };
        };

        let keyboard = keyboard::poll(&event_pump);
        chip8.set_keypad(keyboard);

        chip8.cycle();

        display.draw(&chip8.screen);

        audio.beep(&chip8.sound_timer);

        thread::sleep(TIME);
    }
}
