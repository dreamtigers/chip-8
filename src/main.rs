mod display;

use chip_8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    let mut display = display::Display::new();

    loop {
        chip8.cycle();

        display.draw(&chip8.screen);

        // chip8.set_keys();
    }
}
