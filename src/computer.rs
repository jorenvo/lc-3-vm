use crate::constants;

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

    fn print_registers(&self, registers: &[usize]) {
        for register in registers {
            println!("{:>2}: {:#018b}", register, self.registers[*register]);
        }
    }

    fn update_flags(&mut self, result: i16) {
        self.registers[constants::RCOND] = match result {
            result if result < 0 => constants::CONDNEGATIVE,
            result if result == 0 => constants::CONDZERO,
            result if result > 0 => constants::CONDPOSITIVE,
            _ => panic!("What?"),
        }
    }

    pub fn init_memory(&mut self, words: Vec<u16>) {
        let origin = words[0] as usize;
        self.memory[origin..(origin + words.len() - 1)].copy_from_slice(&words[1..]);
    }

    pub fn run(&mut self) {
        self.registers[constants::RPC] = constants::DEFAULT_START;

        let watched_registers = vec![constants::R0, constants::RPC, constants::RCOND];
        let mut running = true;
        while running {
            let instruction = self.memory[self.registers[constants::RPC] as usize];

            println!("Processing instruction {:#018b}", instruction);

            let opcode = instruction >> 12;
            match opcode {
                0b0001 => {
                    let dest_register = ((instruction >> 9) & 0b111) as usize;
                    let src_register = ((instruction >> 6) & 0b111) as usize;
                    let is_immediate_add = (instruction & 0b10_0000) > 0;

                    if is_immediate_add {
                        let immediate_value = instruction & 0b1_1111; // TODO sign extend, negative numbers
                        self.registers[dest_register] =
                            self.registers[src_register] + immediate_value;
                    } else {
                        panic!("Register add, not supported yet");
                    };

                    self.update_flags(self.registers[dest_register] as i16);
                    println!("got add");
                }
                0b0101 => println!("got and"),
                _ => {
                    println!(
                        "Stopping computer because of unsupported opcode {:#06b}",
                        opcode
                    );

                    running = false;
                }
            };

            self.print_registers(&watched_registers);

            // go to next instruction
            self.registers[constants::RPC] += 1;
        }
    }
}
