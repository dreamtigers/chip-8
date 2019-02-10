use chip_8::Chip8;

fn main() {
    let chip8 = Chip8::new();

    loop {
        chip8.cycle();

        chip8.draw();

        // chip8.set_keys();
    }
}
