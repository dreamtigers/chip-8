use sdl2;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;

pub struct Keypad {
    events: EventPump,
    keys: [bool; 16],
}

impl Keypad {
    pub fn new(sdl_context: &Sdl) -> Keypad {
        let event_subsystem = sdl_context.event_pump().unwrap();

        Keypad {
            events: event_subsystem,
            keys: [false; 16],
        }
    }

    pub fn poll(&mut self) -> Result<[bool; 16], &str> {
        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err("Quitted Chip-8");
            };
        }

        let keys: Vec<Scancode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            // Scancodes represent the physical position of the key, modeled
            // after the standard QWERTY keyboard.
            // On an AZERTY keyboard, pressing the QWERTY letter 'Q' will emit
            // a 'q' scancode and an 'a' keycode.
            .collect();

        /// | 1 | 2 | 3 | C |    | 1 | 2 | 3 | 4 |
        /// | 4 | 5 | 6 | D | -> | Q | W | E | R |
        /// | 7 | 8 | 9 | E |    | A | S | D | F |
        /// | A | 0 | B | F |    | Z | X | C | V |
        for key in keys {
            let index = match key {
                Scancode::Num1 => Some(0x1),
                Scancode::Num2 => Some(0x2),
                Scancode::Num3 => Some(0x3),
                Scancode::Num4 => Some(0xc),
                Scancode::Q => Some(0x4),
                Scancode::W => Some(0x5),
                Scancode::E => Some(0x6),
                Scancode::R => Some(0xd),
                Scancode::A => Some(0x7),
                Scancode::S => Some(0x8),
                Scancode::D => Some(0x9),
                Scancode::F => Some(0xe),
                Scancode::Z => Some(0xa),
                Scancode::X => Some(0x0),
                Scancode::C => Some(0xb),
                Scancode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                self.keys[i] = true;
            }
        }

        Ok(self.keys)
    }
}
