use rand;

pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
pub const CHIP8_RAM: usize = 4096;

const OPCODE_SIZE: usize = 2;

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
}

pub struct Chip8 {
    // Index Register for memory addresses
    i: u16,
    // General purpose registers
    v: [u8; 16],
    // u16 Program counter, not accessable from Chip-8 programs, stores current executing address.
    pc: usize,
    // Stack pointer, points to the topmost level of the stack
    sp: usize,
    // Stack, used for recursion.
    stack: [u16; 16],
    // The RAM
    memory: [u8; CHIP8_RAM],
    pub screen: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub keypad: [bool; 16],
    delay_timer: u8,
    sound_timer: u8,

    keypad_waiting: bool,
    keypad_register: usize,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut memory = [0u8; CHIP8_RAM];

        for i in 0..FONT_SET.len() {
            memory[i] = FONT_SET[i];
        }

        Chip8 {
            i: 0,
            pc: 0x200,
            sp: 0,
            v: [0; 16],
            stack: [0; 16],
            memory: memory,
            screen: [[0u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
            keypad: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
            keypad_waiting: false,
            keypad_register: 0,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = self.pc + i;
            if addr < CHIP8_RAM {
                self.memory[self.pc + i] = byte;
            } else {
                break;
            }
        }

    }

    pub fn cycle(&mut self) {
        if self.keypad_waiting {
            for i in 0..self.keypad.len() {
                if self.keypad[i] {
                    self.keypad_waiting = false;
                    self.v[self.keypad_register] = i as u8;
                    break;
                }
            }
        } else {
            if self.delay_timer > 0 {
                self.delay_timer -= 1
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1
            }

            let opcode = self.read_opcode();

            self.eval_opcode(opcode);
        }
    }

    pub fn read_opcode(&self) -> u16 {
        (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16)
    }

    fn eval_opcode(&mut self, opcode: u16) {
        // These are the nibbles (groups of 4 bits)
        let op = ((opcode & 0xF000) >> 12) as usize;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as usize;

        let nnn = (opcode & 0x0FFF) as u16;
        let kk = (opcode & 0x00FF) as u8;

        let pc_change = match (op, x, y, n) {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01,    _,    _,    _) => self.op_1nnn(nnn),
            (0x02,    _,    _,    _) => self.op_2nnn(nnn),
            (0x03,    _,    _,    _) => self.op_3xkk(x, kk),
            (0x04,    _,    _,    _) => self.op_4xkk(x, kk),
            (0x05,    _,    _, 0x00) => self.op_5xy0(x, y),
            (0x06,    _,    _,    _) => self.op_6xkk(x, kk),
            (0x07,    _,    _,    _) => self.op_7xkk(x, kk),
            (0x08,    _,    _, 0x00) => self.op_8xy0(x, y),
            (0x08,    _,    _, 0x01) => self.op_8xy1(x, y),
            (0x08,    _,    _, 0x02) => self.op_8xy2(x, y),
            (0x08,    _,    _, 0x03) => self.op_8xy3(x, y),
            (0x08,    _,    _, 0x04) => self.op_8xy4(x, y),
            (0x08,    _,    _, 0x05) => self.op_8xy5(x, y),
            (0x08,    _,    _, 0x06) => self.op_8xy6(x, y),
            (0x08,    _,    _, 0x07) => self.op_8xy7(x, y),
            (0x08,    _,    _, 0x0e) => self.op_8xye(x, y),
            (0x09,    _,    _, 0x00) => self.op_9xy0(x, y),
            (0x0a,    _,    _,    _) => self.op_annn(nnn),
            (0x0b,    _,    _,    _) => self.op_bnnn(nnn),
            (0x0c,    _,    _,    _) => self.op_cxkk(x, kk),
            (0x0d,    _,    _,    _) => self.op_dxyn(x, y, n),
            (0x0e,    _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e,    _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f,    _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f,    _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f,    _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f,    _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f,    _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f,    _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f,    _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f,    _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f,    _, 0x06, 0x05) => self.op_fx65(x),
            _                        => self.no_impl(opcode),
        };

        match pc_change {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(addr) => self.pc = addr as usize,
        }
    }

    fn op_00e0(&mut self) -> ProgramCounter {
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                self.screen[y][x] = 0;
            }
        }

        ProgramCounter::Next
    }

    fn op_00ee(&mut self) -> ProgramCounter {
        self.pc = self.stack[self.sp as usize] as usize;
        self.sp -= 1;

        ProgramCounter::Jump(self.pc as u16)
    }

    fn op_1nnn(&mut self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }

    fn op_2nnn(&mut self, nnn: u16) -> ProgramCounter {
        self.stack[(self.sp as usize)] = self.pc as u16;
        self.sp += 1;
        ProgramCounter::Jump(nnn)
    }

    fn op_3xkk(&self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] == kk)
    }

    fn op_4xkk(&self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] != kk)
    }

    fn op_5xy0(&self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] == self.v[y])
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.v[x] = kk;
        ProgramCounter::Next
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.v[x] = self.v[x].wrapping_add(kk);
        ProgramCounter::Next
    }

    fn op_8xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] = self.v[y];
        ProgramCounter::Next
    }

    fn op_8xy1(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] |= self.v[y];
        ProgramCounter::Next
    }

    fn op_8xy2(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] &= self.v[y];
        ProgramCounter::Next
    }

    fn op_8xy3(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] ^= self.v[y];
        ProgramCounter::Next
    }

    fn op_8xy4(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] = match self.v[x].checked_add(self.v[y]) {
            Some(n) => n,
            None => {
                self.v[0xF] = 1;
                self.v[x].wrapping_add(self.v[y])
            },
        };

        ProgramCounter::Next
    }

    fn op_8xy5(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);

        ProgramCounter::Next
    }

    fn op_8xy6(&mut self, x: usize, _y: usize) -> ProgramCounter {
        self.v[0xF] = self.v[x] & 0x1;
        self.v[x] >>= 1;

        ProgramCounter::Next
    }

    fn op_8xy7(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);

        ProgramCounter::Next
    }

    fn op_8xye(&mut self, x: usize, _y: usize) -> ProgramCounter {
        self.v[0xF] = (self.v[x] & 0b10000000) >> 7;
        self.v[x] <<= 1;

        ProgramCounter::Next
    }

    fn op_9xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] != self.v[y])
    }

    fn op_annn(&mut self, nnn: u16) -> ProgramCounter {
        self.i = nnn;

        ProgramCounter::Next
    }

    fn op_bnnn(&mut self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(nnn + (self.v[0] as u16))
    }

    fn op_cxkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.v[x] = rand::random::<u8>() & kk;

        ProgramCounter::Next
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) -> ProgramCounter {
        for byte in 0..n {
            let y = (self.v[y] as usize + byte) % CHIP8_HEIGHT;
            for bit in 0..8 {
                let x = (self.v[x] as usize + bit) % CHIP8_WIDTH;
                let color = ((self.memory[(self.i as usize) + byte]) >> (7 - bit)) & 1;
                self.v[0xF] |= color & self.screen[y][x];
                self.screen[y][x] ^= color;
            }
        }

        ProgramCounter::Next
    }

    fn op_ex9e(&mut self, x: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.keypad[self.v[x] as usize])
    }

    fn op_exa1(&mut self, x: usize) -> ProgramCounter {
        ProgramCounter::skip_if(!self.keypad[self.v[x] as usize])
    }

    fn op_fx07(&mut self, x: usize) -> ProgramCounter {
        self.v[x] = self.delay_timer;

        ProgramCounter::Next
    }

    fn op_fx0a(&mut self, x: usize) -> ProgramCounter {
        self.keypad_waiting = true;
        self.keypad_register = x;

        ProgramCounter::Next
    }

    fn op_fx15(&mut self, x: usize) -> ProgramCounter {
        self.delay_timer = self.v[x];

        ProgramCounter::Next
    }

    fn op_fx18(&mut self, x: usize) -> ProgramCounter {
        self.sound_timer = self.v[x];

        ProgramCounter::Next
    }

    fn op_fx1e(&mut self, x: usize) -> ProgramCounter {
        self.i = match self.i.checked_add(self.v[x] as u16) {
            Some(n) => n,
            None => {
                self.v[0xF] = 1;
                self.i.wrapping_add(self.v[x] as u16)
            },
        };

        ProgramCounter::Next
    }

    fn op_fx29(&mut self, x: usize) -> ProgramCounter {
        self.i = (self.v[x] as u16) * 5;

        ProgramCounter::Next
    }

    fn op_fx33(&mut self, x: usize) -> ProgramCounter {
        self.memory[self.i as usize] = self.v[x] / 100;
        self.memory[(self.i as usize) + 1] = (self.v[x] % 100) / 10;
        self.memory[(self.i as usize) + 2] = self.v[x] % 10;

        ProgramCounter::Next
    }

    fn op_fx55(&mut self, x: usize) -> ProgramCounter {
        for i in 0..(x + 1) {
            self.memory[(self.i as usize) + i] = self.v[i];
        }

        ProgramCounter::Next
    }

    fn op_fx65(&mut self, x: usize) -> ProgramCounter {
        for i in 0..(x + 1) {
            self.v[i] = self.memory[(self.i as usize) + i];
        }

        ProgramCounter::Next
    }

    fn no_impl(&self, opcode: u16) -> ProgramCounter {
        println!("Not implemented: opcode {:x} in memory address {:x}",
                 opcode, self.pc);

        ProgramCounter::Next
    }
}

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let mut chip8 = Chip8::new();

        chip8.load(&vec![1, 2, 3]);

        assert_eq!(chip8.memory[0x200], 1);
        assert_eq!(chip8.memory[0x201], 2);
        assert_eq!(chip8.memory[0x202], 3);
    }

    #[test]
    fn test_op_00e0() {
        let mut chip8 = Chip8::new();

        // Change the default screen state
        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                chip8.screen[y][x] = 1;
            }
        }

        chip8.eval_opcode(0x00E0);

        for y in 0..CHIP8_HEIGHT {
            for x in 0..CHIP8_WIDTH {
                assert_eq!(chip8.screen[y][x], 0);
            }
        }
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_00ee() {
        let mut chip8 = Chip8::new();
        chip8.pc = 0x432;
        chip8.sp = 1;
        chip8.stack[chip8.sp as usize] = 0x200;

        chip8.eval_opcode(0x00EE);

        assert_eq!(chip8.pc, 0x200, "Program Counter");
        assert_eq!(chip8.stack[0], 0, "Stack");
        assert_eq!(chip8.sp, 0, "Stack Pointer");
    }

    #[test]
    fn test_op_1nnn() {
        let mut chip8 = Chip8::new();

        chip8.eval_opcode(0x1F4C);

        assert_eq!(chip8.pc, 0x0F4C);
    }

    #[test]
    fn test_op_2nnn() {
        let mut chip8 = Chip8::new();

        chip8.eval_opcode(0x2F4C);

        assert_eq!(chip8.pc, 0x0F4C, "Program Counter");
        assert_eq!(chip8.stack[0], 0x200 as u16, "Stack");
        assert_eq!(chip8.sp, 1, "Stack Pointer");
    }

    #[test]
    fn test_op_3xkk() {
        let mut chip8 = Chip8::new();
        chip8.v[2] = 0x02;
        chip8.v[5] = 0x05;

        chip8.eval_opcode(0x3202);

        assert_eq!(chip8.pc, 0x204, "Skip");

        chip8.eval_opcode(0x3506);

        assert_eq!(chip8.pc, 0x206, "Next");
    }

    #[test]
    fn test_op_4xkk() {
        let mut chip8 = Chip8::new();
        chip8.v[5] = 0x66;
        chip8.v[2] = 0x4C;

        chip8.eval_opcode(0x4577);

        assert_eq!(chip8.pc, 0x204, "Skip");

        chip8.eval_opcode(0x424C);

        assert_eq!(chip8.pc, 0x206, "Next");
    }

    #[test]
    fn test_op_5xy0() {
        let mut chip8 = Chip8::new();
        chip8.v[7] = 0x07;
        chip8.v[8] = 0x08;
        chip8.v[9] = 0x07;

        chip8.eval_opcode(0x5790);

        assert_eq!(chip8.pc, 0x204, "Skip");

        chip8.eval_opcode(0x5780);

        assert_eq!(chip8.pc, 0x206, "Next");
    }

    #[test]
    fn test_op_6xkk() {
        let mut chip8 = Chip8::new();

        chip8.eval_opcode(0x6789);

        assert_eq!(chip8.v[7], 0x89);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_7xkk() {
        let mut chip8 = Chip8::new();
        chip8.v[8] = 5;

        chip8.eval_opcode(0x7804);

        assert_eq!(chip8.v[8], 0x09);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_8xy0() {
        let mut chip8 = Chip8::new();
        chip8.v[3] = 4;
        chip8.v[5] = 6;

        chip8.eval_opcode(0x8350);

        assert_eq!(chip8.v[3], 0x06);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_8xy1() {
        let mut chip8 = Chip8::new();
        chip8.v[3] = 0xF0;
        chip8.v[5] = 0x0F;

        chip8.eval_opcode(0x8351);

        assert_eq!(chip8.v[3], 0xFF);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_8xy2() {
        let mut chip8 = Chip8::new();
        chip8.v[3] = 0xF0;
        chip8.v[5] = 0xFF;

        chip8.eval_opcode(0x8352);

        assert_eq!(chip8.v[3], 0xF0);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_8xy3() {
        let mut chip8 = Chip8::new();
        chip8.v[3] = 0x0F;
        chip8.v[5] = 0xFF;

        chip8.eval_opcode(0x8353);

        assert_eq!(chip8.v[3], 0xF0);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_8xy4() {
        let mut chip8 = Chip8::new();
        chip8.v[1] = 0x05;
        chip8.v[2] = 0x05;
        chip8.v[6] = 0xFF;
        chip8.v[7] = 0x0F;

        chip8.eval_opcode(0x8124);

        assert_eq!(chip8.v[1], 0x0A);
        assert_eq!(chip8.v[0xF], 0x00);
        assert_eq!(chip8.pc, 0x202);

        chip8.eval_opcode(0x8674);

        assert_eq!(chip8.v[6], 0x0E);
        assert_eq!(chip8.v[0xF], 0x01);
        assert_eq!(chip8.pc, 0x204);
    }

    #[test]
    fn test_op_8xy5() {
        let mut chip8 = Chip8::new();
        chip8.v[1] = 0x05;
        chip8.v[2] = 0x04;
        chip8.v[6] = 0x0F;
        chip8.v[7] = 0xFF;

        chip8.eval_opcode(0x8125);

        assert_eq!(chip8.v[1], 0x01);
        assert_eq!(chip8.v[0xF], 0x01);
        assert_eq!(chip8.pc, 0x202);

        chip8.eval_opcode(0x8675);

        assert_eq!(chip8.v[6], 0x10);
        assert_eq!(chip8.v[0xF], 0x00);
        assert_eq!(chip8.pc, 0x204);
    }

    #[test]
    fn test_op_8xy6() {
        let mut chip8 = Chip8::new();
        chip8.v[1] = 0x05;
        chip8.v[2] = 0x04;

        chip8.eval_opcode(0x8106);

        assert_eq!(chip8.v[1], 0x02);
        assert_eq!(chip8.v[0xF], 0x01);
        assert_eq!(chip8.pc, 0x202);

        chip8.eval_opcode(0x8206);

        assert_eq!(chip8.v[2], 0x02);
        assert_eq!(chip8.v[0xF], 0x00);
        assert_eq!(chip8.pc, 0x204);
    }

    #[test]
    fn test_op_8xy7() {
        let mut chip8 = Chip8::new();
        chip8.v[1] = 0x01;
        chip8.v[2] = 0x02;
        chip8.v[3] = 0x03;

        chip8.eval_opcode(0x8127);

        assert_eq!(chip8.v[1], 0x01);
        assert_eq!(chip8.v[0xF], 0x01);
        assert_eq!(chip8.pc, 0x202);

        chip8.eval_opcode(0x8317);

        assert_eq!(chip8.v[3], 0xFE);
        assert_eq!(chip8.v[0xF], 0x00);
        assert_eq!(chip8.pc, 0x204);
    }

    #[test]
    fn test_op_8xye() {
        let mut chip8 = Chip8::new();
        chip8.v[1] = 0xFF;
        chip8.v[2] = 0x0F;

        chip8.eval_opcode(0x810e);

        assert_eq!(chip8.v[1], 0xFE);
        assert_eq!(chip8.v[0xF], 0x01);
        assert_eq!(chip8.pc, 0x202);

        chip8.eval_opcode(0x820e);

        assert_eq!(chip8.v[2], 0x1E);
        assert_eq!(chip8.v[0xF], 0x00);
        assert_eq!(chip8.pc, 0x204);
    }

    #[test]
    fn test_op_9xy0() {
        let mut chip8 = Chip8::new();

        chip8.eval_opcode(0xa123);

        assert_eq!(chip8.i, 0x123);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_annn() {
        let mut chip8 = Chip8::new();

        chip8.eval_opcode(0xa123);

        assert_eq!(chip8.i, 0x123);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_bnnn() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 5;

        chip8.eval_opcode(0xb123);

        assert_eq!(chip8.pc, 0x128);
    }

    #[test]
    fn test_op_cxkk() {
        let mut chip8 = Chip8::new();
        chip8.v[0] = 0;

        chip8.eval_opcode(0xc000);

        assert_eq!(chip8.v[0], 0x0);

        chip8.eval_opcode(0xc00F);

        assert_eq!(chip8.v[0] & 0xF0, 0x0);
    }

    #[test]
    fn test_op_dxyn() {
        let mut chip8 = Chip8::new();

        chip8.i = 0;
        chip8.memory[0] = 0b11111111;
        chip8.memory[1] = 0b00000000;
        chip8.screen[0][0] = 1;
        chip8.screen[0][1] = 0;
        chip8.screen[1][0] = 1;
        chip8.screen[1][1] = 0;
        chip8.v[0] = 0;

        chip8.eval_opcode(0xd002);

        assert_eq!(chip8.screen[0][0], 0);
        assert_eq!(chip8.screen[0][1], 1);
        assert_eq!(chip8.screen[1][0], 1);
        assert_eq!(chip8.screen[1][1], 0);
        assert_eq!(chip8.v[0x0f], 1);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_ex9e() {
        let mut chip8 = Chip8::new();

        chip8.keypad[0xA] = true;
        chip8.v[0x4] = 0xA;

        chip8.eval_opcode(0xe49e);

        assert_eq!(chip8.pc, 0x204, "keypad[v[x]] == true");

        chip8.eval_opcode(0xe59e);

        assert_eq!(chip8.pc, 0x206, "keypad[v[x]] == false");
    }

    #[test]
    fn test_op_exa1() {
        let mut chip8 = Chip8::new();

        chip8.keypad[0xA] = true;
        chip8.v[0x4] = 0xA;

        chip8.eval_opcode(0xe4a1);

        assert_eq!(chip8.pc, 0x202, "keypad[v[x]] == false");

        chip8.eval_opcode(0xe59e);

        assert_eq!(chip8.pc, 0x204, "keypad[v[x]] == true");
    }

    #[test]
    fn test_op_fx07() {
        let mut chip8 = Chip8::new();

        chip8.delay_timer = 40;
        chip8.v[0x4] = 15;

        chip8.eval_opcode(0xf407);

        assert_eq!(chip8.v[4], 40);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_fx0a() {
        let mut chip8 = Chip8::new();

        chip8.eval_opcode(0xf50a);
        assert_eq!(chip8.keypad_waiting, true);
        assert_eq!(chip8.keypad_register, 5);
        assert_eq!(chip8.pc, 0x202);

        // Tick with no keypresses doesn't do anything
        chip8.keypad = [false; 16];
        chip8.cycle();
        assert_eq!(chip8.keypad_waiting, true);
        assert_eq!(chip8.keypad_register, 5);
        assert_eq!(chip8.pc, 0x202);

        // Tick with a keypress finishes wait and loads
        // first pressed key into vx
        chip8.keypad = [true; 16];
        chip8.cycle();
        assert_eq!(chip8.keypad_waiting, false);
        assert_eq!(chip8.v[5], 0);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_fx15() {
        let mut chip8 = Chip8::new();

        chip8.delay_timer = 40;
        chip8.v[0x4] = 15;

        chip8.eval_opcode(0xf415);

        assert_eq!(chip8.delay_timer, 15);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_fx18() {
        let mut chip8 = Chip8::new();

        chip8.sound_timer = 40;
        chip8.v[0x4] = 15;

        chip8.eval_opcode(0xf418);

        assert_eq!(chip8.sound_timer, 15);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_fx1e() {
        let mut chip8 = Chip8::new();

        chip8.i = 14;
        chip8.v[0x4] = 15;

        chip8.eval_opcode(0xf41e);

        assert_eq!(chip8.i, 29);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_fx29() {
        let mut chip8 = Chip8::new();

        chip8.v[0x4] = 4;

        chip8.eval_opcode(0xf429);

        assert_eq!(chip8.i, 20);
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_fx33() {
        let mut chip8 = Chip8::new();

        chip8.v[5] = 123;
        chip8.i = 1000;
        chip8.eval_opcode(0xf533);
        assert_eq!(chip8.memory[1000], 1);
        assert_eq!(chip8.memory[1001], 2);
        assert_eq!(chip8.memory[1002], 3);
        assert_eq!(chip8.pc, 0x202);

    }

    #[test]
    fn test_op_fx55() {
        let mut chip8 = Chip8::new();

        chip8.i = 1000;
        chip8.eval_opcode(0xff55);
        for i in 0..16 {
            assert_eq!(chip8.memory[1000 + i as usize], chip8.v[i]);
        }
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_op_fx65() {
        let mut chip8 = Chip8::new();

        for i in 0..16 as usize {
            chip8.memory[1000 + i] = i as u8;
        }
        chip8.i = 1000;
        chip8.eval_opcode(0xff65);

        for i in 0..16 as usize {
            assert_eq!(chip8.v[i], chip8.memory[1000 + i]);
        }
        assert_eq!(chip8.pc, 0x202);
    }

    #[test]
    fn test_no_impl() {
        let mut chip8 = Chip8::new();

        chip8.eval_opcode(0xFFFF);

        assert_eq!(chip8.pc, 0x202);
    }
}
