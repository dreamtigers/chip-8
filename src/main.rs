use sdl2;
use sdl2::event::Event;

use std::env;
use std::fs;
use std::time::Duration;
use std::thread;

use chip_8::Chip8;

mod audio;
mod display;
mod keyboard;

const TIME : Duration = Duration::from_millis(2);

fn main() {
    let filename = env::args().nth(1).expect("Filename not specified");
    let data = fs::read(filename).expect("There was a problem opening the file");

    let     sdl_context = sdl2::init().unwrap();
    let mut event_pump  = sdl_context.event_pump().unwrap();

    let mut chip8   = Chip8::new();
    let     audio   = audio::Audio::new(&sdl_context);
    let mut display = display::Display::new(&sdl_context);

    chip8.load(&data);

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
