pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
pub const CHIP8_RAM: usize = 4096;

const OPCODE_SIZE: usize = 2;

pub struct Chip8 {
    // Index Register for memory addresses
    pub i: u16,
    // General purpose registers
    pub v: [u8; 16],
    // u16 Program counter, not accessable from Chip-8 programs, stores current executing address.
    pub pc: usize,
    // Stack pointer, points to the topmost level of the stack
    pub sp: u8,
    pub stack: [u16; 16],
    pub memory: [u8; CHIP8_RAM],

    pub screen: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
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
        }
    }

    pub fn cycle(&self) -> () {
        // help
        let opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
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
