use sdl2::EventPump;
use sdl2::keyboard::Scancode;

pub fn poll(event_pump: &EventPump, keypad: &mut [bool; 16]) {
    let keyboard: Vec<Scancode> = event_pump
        .keyboard_state()
        .pressed_scancodes()
        // Scancodes represent the physical position of the key, modeled
        // after the standard QWERTY keyboard.
        // On an AZERTY keyboard, pressing the QWERTY letter 'Q' will emit
        // a 'q' scancode and an 'a' keycode.
        .collect();

    // | 1 | 2 | 3 | 4 |    | 1 | 2 | 3 | C |
    // | Q | W | E | R | -> | 4 | 5 | 6 | D |
    // | A | S | D | F |    | 7 | 8 | 9 | E |
    // | Z | X | C | V |    | A | 0 | B | F |
    for pressed_key in keyboard {
        let index = match pressed_key {
            Scancode::Num1 => Some(0x1),
            Scancode::Num2 => Some(0x2),
            Scancode::Num3 => Some(0x3),
            Scancode::Num4 => Some(0xC),
            Scancode::Q    => Some(0x4),
            Scancode::W    => Some(0x5),
            Scancode::E    => Some(0x6),
            Scancode::R    => Some(0xD),
            Scancode::A    => Some(0x7),
            Scancode::S    => Some(0x8),
            Scancode::D    => Some(0x9),
            Scancode::F    => Some(0xE),
            Scancode::Z    => Some(0xA),
            Scancode::X    => Some(0x0),
            Scancode::C    => Some(0xB),
            Scancode::V    => Some(0xF),
            _ => None,
        };

        if let Some(i) = index {
            keypad[i] = true;
        }
    }
}
