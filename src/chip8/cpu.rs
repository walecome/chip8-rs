use crate::chip8::instruction::Instruction;

struct Memory {
    data: Vec<u8>,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            data: vec![0; 4096],
        }

    }
}

pub struct Cpu {
    pc: u16,
    memory: Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu {
            pc: 0,
            memory: Memory::new(),
        };
    }

    pub fn fetch(& mut self) -> u16 {
        let first_byte = self.memory.data[self.pc_as_index()];
        let second_byte = self.memory.data[self.pc_as_index() + 1];
        self.pc += 2;

        let mut result: u16 = first_byte as u16;
        result = result << 1;
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
                register: (raw & 0x0F00) as u8,
                value: (raw & 0x00FF) as u8,
            },
            0x7000..=0x7FFF => Instruction::AddVX {
                register: (raw & 0x0F00) as u8,
                value: (raw & 0x00FF) as u8,
            },
            0xA000..=0xAFFF => Instruction::SetI(raw & 0x0FFF),
            0xD000..=0xDFFF => Instruction::DisplayDraw {
                x: (raw & 0x0F00) as u8,
                y: (raw & 0x00F0) as u8,
                n: (raw & 0x000F) as u8,
            },
            _ => panic!("Unknown instruction: {:x}", raw),
        }
    }

    pub fn execute(&self, instruction: Instruction) {
    }

    fn pc_as_index(&self) -> usize {
        self.pc as usize
    }
}

