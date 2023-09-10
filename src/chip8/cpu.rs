use crate::chip8::instruction::Instruction;

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(rom_data: Vec<u8>) -> Memory {
        assert!(rom_data.len() <= 4096);
        let mut memory: Vec<u8> = vec![0; 4096];
        let start_address = 512;
        // TODO: Set up fonts in memory
        for (i, byte) in rom_data.iter().enumerate() {
            memory[start_address + i] = *byte;
        }
        Memory {
            data: memory,
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn set(& mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

pub struct VRAM {
    pub width: u8,
    pub height: u8,
    data: Vec<bool>,
}

impl VRAM {
    pub fn new() -> VRAM {
        let width: usize = 64;
        let height: usize = 32;
        VRAM {
            width: width as u8,
            height: height as u8,
            data: vec![false; width * height],
        }
    }

    pub fn get_cell(&self, x: u8, y: u8) -> bool {
        return self.data[y as usize * self.width as usize + x as usize];
    }

    fn set_cell(& mut self, x: u8, y: u8, value: bool) {
        self.data[y as usize * self.width as usize + x as usize] = value;
    }

    fn clear(& mut self) {
        self.data.clear();
        self.data.resize(self.width as usize * self.height as usize, false);
    }

}

pub struct Cpu {
    pc: u16,
    memory: Memory,
    registers: Vec<u8>,
    index_register: u16,
    vram: VRAM,
    call_stack: Vec<u16>,
    use_copy_shift: bool,
}

fn get_nibble_from_right(i: u8, value: u16) -> u8 {
    let shift = 4 * i;
    return ((value >> shift) & 0x000F) as u8;
}

impl Cpu {
    pub fn new(memory: Memory, use_copy_shift: bool) -> Cpu {
        return Cpu {
            pc: 512,
            memory,
            registers: vec![0; 16],
            index_register: 0,
            vram: VRAM::new(),
            call_stack: vec![],
            use_copy_shift,
        };
    }

    pub fn vram(&self) -> &VRAM {
        &self.vram
    }

    pub fn fetch(& mut self) -> u16 {
        let first_byte = self.memory.data[self.pc_as_index()];
        let second_byte = self.memory.data[self.pc_as_index() + 1];
        self.pc += 2;

        let mut result: u16 = first_byte as u16;
        result = result << 8;
        result |= second_byte as u16;

        return result;
    }

    pub fn decode(&self, raw: u16) -> Instruction {
        match raw {
            0x00E0 => Instruction::ClearScreen,
            0x00EE => Instruction::SubroutineReturn,
            0x1000..=0x1FFF => Instruction::Jump(raw & 0x0FFF),
            0x2000..=0x2FFF => Instruction::SubroutineCall(raw & 0x0FFF),
            0x6000..=0x6FFF => Instruction::SetVX {
                register: get_nibble_from_right(2, raw),
                value: (raw & 0x00FF) as u8,
            },
            0x7000..=0x7FFF => Instruction::AddVX {
                register: get_nibble_from_right(2, raw),
                value: (raw & 0x00FF) as u8,
            },
            0xA000..=0xAFFF => Instruction::SetI(raw & 0x0FFF),
            0xD000..=0xDFFF => Instruction::DisplayDraw {
                register_x: get_nibble_from_right(2, raw),
                register_y: get_nibble_from_right(1, raw),
                n: get_nibble_from_right(0, raw),
            },
            0x3000..=0x3FFF => Instruction::NoopImmediateEq {
                register: get_nibble_from_right(2, raw),
                value: (raw & 0x00FF) as u8,
            },
            0x4000..=0x4FFF => Instruction::NoopImmediateNotEq {
                register: get_nibble_from_right(2, raw),
                value: (raw & 0x00FF) as u8,
            },
            0x5000..=0x5FF0 => Instruction::NoopRegisterEq {
                register_x: get_nibble_from_right(2, raw),
                register_y: get_nibble_from_right(1, raw),
            },
            0x9000..=0x9FF0 => Instruction::NoopRegisterNotEq {
                register_x: get_nibble_from_right(2, raw),
                register_y: get_nibble_from_right(1, raw),
            },
            0x0800..=0x8FFF => {
                let masked = get_nibble_from_right(0, raw);
                match masked {
                    0x00 => Instruction::ArithmeticSet {
                        register_x: get_nibble_from_right(2, raw),
                        register_y: get_nibble_from_right(1, raw),
                    },
                    0x01 => Instruction::ArithmeticOr {
                        register_x: get_nibble_from_right(2, raw),
                        register_y: get_nibble_from_right(1, raw),
                    },
                    0x02 => Instruction::ArithmeticAnd {
                        register_x: get_nibble_from_right(2, raw),
                        register_y: get_nibble_from_right(1, raw),
                    },
                    0x03 => Instruction::ArithmeticXor {
                        register_x: get_nibble_from_right(2, raw),
                        register_y: get_nibble_from_right(1, raw),
                    },
                    0x04 => Instruction::ArithmeticAdd {
                        register_x: get_nibble_from_right(2, raw),
                        register_y: get_nibble_from_right(1, raw),
                    },
                    0x05 => Instruction::ArithmeticSubtract {
                        register_x: get_nibble_from_right(2, raw),
                        register_y: get_nibble_from_right(1, raw),
                    },
                    0x06 => Instruction::ArithmeticShiftRight {
                        register_x: get_nibble_from_right(2, raw),
                        register_y: get_nibble_from_right(1, raw),
                    },
                    0x0E => Instruction::ArithmeticShiftLeft {
                        register_x: get_nibble_from_right(2, raw),
                        register_y: get_nibble_from_right(1, raw),
                    },
                    _ => panic!("Unknown arithmetic instruction: {:#06X}", raw),
                }
            },
            0xF055..=0xFF55 => Instruction::Store(get_nibble_from_right(2, raw)),
            _ => panic!("Unknown instruction: {:#06X}", raw),
        }
    }

    pub fn execute(& mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearScreen => self.vram.clear(),
            Instruction::Jump(address) => {
                self.pc = address;
            },
            Instruction::SubroutineReturn => {
                self.pc = self.call_stack.pop().unwrap();
            },
            Instruction::SubroutineCall(address) => {
                self.call_stack.push(self.pc);
                self.pc = address;
            },
            Instruction::SetVX { register, value } => {
                self.set_register(register, value);
            },
            Instruction::AddVX { register, value } => {
                let existing_value = self.get_register(register);
                self.set_register(register, existing_value.wrapping_add(value));
            },
            Instruction::SetI(value) => {
                self.index_register = value;
            },
            Instruction::DisplayDraw { register_x, register_y, n } => {
                // TODO: Don't inline display dimensions
                // Set the X coordinate to the value in VX modulo 64
                let mut x = self.get_register(register_x) % 64;
                let start_x = x;
                // Set the Y coordinate to the value in VY modulo 32
                let mut y = self.get_register(register_y) % 32;
                // Set VF to 0
                self.set_register(0x0F, 0);
                let start_index = self.index_register;
                // For N rows
                for i in 0..n {
                    // Stop if you reach the bottom edge of the screen
                    if y >= self.vram.height {
                        break;
                    }
                    // Get the Nth byte of sprite data, counting from the memory address in the I register
                    let sprite_data = self.memory.get(start_index + (i as u16));
                    // For each of the 8 pixels/bits in this sprite row (from most signifant to least significant)
                    for bit_flag in (0..8).rev() {
                        // If you reach the right edge of the screen, stop drawing this row
                        if x >= self.vram.width {
                            break;
                        }
                        let sprite_bit_enabled = (1 << bit_flag) & sprite_data != 0;
                        // If the current pixel in the sprite row is on and the pixel at coordinates X,Y on the screen is also on...
                        if sprite_bit_enabled && self.vram.get_cell(x, y) {
                            // ... turn off the pixel and set VF to 1
                            self.vram.set_cell(x, y, false);
                            self.set_register(0x0F, 1);
                        }
                        // Or if the current pixel in the sprite row is on and the screen pixel is not...
                        else if sprite_bit_enabled && !self.vram.get_cell(x, y) {
                            // ... draw the pixel at the X and Y coordinates
                            self.vram.set_cell(x, y, true);
                        }

                        // Increment x
                        x += 1;
                    }
                    // Increment Y
                    y += 1;
                    x = start_x;
                }
            },
            Instruction::NoopImmediateEq { register, value } => {
                if self.get_register(register) == value {
                    self.do_noop();
                }
            },
            Instruction::NoopImmediateNotEq { register, value } => {
                if self.get_register(register) != value {
                    self.do_noop();
                }
            },
            Instruction::NoopRegisterEq { register_x, register_y } => {
                if self.get_register(register_x) == self.get_register(register_y) {
                    self.do_noop();
                }
            },
            Instruction::NoopRegisterNotEq { register_x, register_y } => {
                if self.get_register(register_x) != self.get_register(register_y) {
                    self.do_noop();
                }
            },
            Instruction::ArithmeticSet { register_x, register_y } => {
                let value_y = self.get_register(register_y);
                self.set_register(register_x, value_y);
            },
            Instruction::ArithmeticOr { register_x, register_y } => {
                let value_x = self.get_register(register_x);
                let value_y = self.get_register(register_y);
                self.set_register(register_x, value_x | value_y);
            },
            Instruction::ArithmeticAnd { register_x, register_y } => {
                let value_x = self.get_register(register_x);
                let value_y = self.get_register(register_y);
                self.set_register(register_x, value_x & value_y);
            },
            Instruction::ArithmeticXor { register_x, register_y } => {
                let value_x = self.get_register(register_x);
                let value_y = self.get_register(register_y);
                self.set_register(register_x, value_x ^ value_y);
            },
            Instruction::ArithmeticAdd { register_x, register_y } => {
                let value_x = self.get_register(register_x);
                let value_y = self.get_register(register_y);
                self.set_register(register_x, value_x.wrapping_add(value_y));
                let did_overflow = value_x > 0xFF - value_y;
                let carry = if did_overflow {1} else {0};
                self.set_register(0x0F, carry);
            },
            Instruction::ArithmeticSubtract { register_x, register_y } => {
                let value_x = self.get_register(register_x);
                let value_y = self.get_register(register_y);
                self.set_register(register_x, value_x.wrapping_sub(value_y));

                let carry = if value_x > value_y {
                    1
                } else {
                    0
                };

                self.set_register(0x0F, carry);

            },
            Instruction::ArithmeticShiftRight { register_x, register_y } => {
                if self.use_copy_shift {
                    let value_y = self.get_register(register_y);
                    self.set_register(register_x, value_y);
                }
                let value_x = self.get_register(register_x);
                let carry = value_x & 0x01;
                self.set_register(register_x, value_x >> 1);
                self.set_register(0x0F, carry);
            },
            Instruction::ArithmeticShiftLeft { register_x, register_y } => {
                if self.use_copy_shift {
                    let value_y = self.get_register(register_y);
                    self.set_register(register_x, value_y);
                }
                let value_x = self.get_register(register_x);
                let carry = value_x & 0b1000_0000;
                self.set_register(register_x, value_x << 1);
                self.set_register(0x0F, carry);
            },
            Instruction::Store(inclusive_end_register_x) => {
                // NOTE: For old CHIP-8 versions, index register should be incremented.
                let start_address = self.index_register;
                for i in 0..=inclusive_end_register_x {
                    let value = self.get_register(i);
                    self.memory.set(start_address + (i as u16), value);
                }
            },
        }
    }

    fn set_register(& mut self, register: u8, value: u8) {
        self.registers[register as usize] = value;
    }

    fn get_register(& mut self, register: u8) -> u8 {
        self.registers[register as usize]
    }

    fn pc_as_index(&self) -> usize {
        self.pc as usize
    }

    fn do_noop(& mut self) {
        self.pc += 2;
    }
}

