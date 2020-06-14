use crate::constants;

enum CondFlags {
    Positive = 1,
    Zero = 1 << 1,
    Negative = 1 << 2,
}

pub struct Computer {
    memory: Vec<u16>,
    registers: Vec<u16>,

    memory_size: usize,
    register_count: usize,
}

impl Computer {
    pub fn new() -> Computer {
        const MEMORY_SIZE: usize = 65_535; // 2^16 - 1
        const REGISTER_COUNT: usize = 16;
        let memory: Vec<u16> = vec![0; MEMORY_SIZE];
        let registers: Vec<u16> = vec![0; REGISTER_COUNT];

        Computer {
            memory,
            registers,
            memory_size: MEMORY_SIZE,
            register_count: REGISTER_COUNT,
        }
    }

    pub fn init_memory(&mut self, words: Vec<u16>) {
        let origin = words[0] as usize;
        self.memory[origin..(origin + words.len() - 1)].copy_from_slice(&words[1..]);
    }

    pub fn run(&mut self) {
        self.registers[constants::RPC] = constants::DEFAULT_START;

        let running = true;
        while running {
            let opcode = self.memory[self.registers[constants::RPC] as usize];
            match opcode {
                0x1 => {
                    println!("got add");
                    self.registers[constants::RPC] += 16;
                }
                _ => {
                    panic!("Unsupported operand {:#018b}", opcode);
                }
            };
        }
    }
}
