use crate::constants;

pub struct Computer {
    memory: Vec<u16>,
    registers: Vec<u16>,
}

impl Computer {
    pub fn new() -> Computer {
        const MEMORY_SIZE: usize = 65_535; // 2^16 - 1
        const REGISTER_COUNT: usize = 16;
        let memory: Vec<u16> = vec![0; MEMORY_SIZE];
        let registers: Vec<u16> = vec![0; REGISTER_COUNT];

        Computer { memory, registers }
    }

    fn print_registers(&self, registers: &[usize]) {
        for register in registers {
            println!("{:>2}: {:#018b}", register, self.registers[*register]);
        }
    }

    /// Sign extends an `n_bits`-bits number to 16 bits
    ///
    /// * `to_extend`: number to sign extend
    /// * `n_bits`: `to_extend` is an `n_bits` bits number
    fn sign_extend_to_16_bits(&self, to_extend: u16, n_bits: u16) -> u16 {
        let sign_bit = to_extend >> (n_bits - 1);
        let is_negative = sign_bit > 0;

        // Sign extend 5 bit negative number to 8 bit example:
        //    1 0000
        // 1111 1111
        if is_negative {
            let mask: u16 = u16::MAX << (n_bits - 1);
            to_extend | mask
        } else {
            to_extend
        }
    }

    fn is_negative(&self, a: u16) -> bool {
        (a >> 15) > 0
    }

    fn update_flags(&mut self, result: u16) {
        self.registers[constants::RCOND] = if self.is_negative(result) {
            constants::CONDNEGATIVE
        } else if result == 0 {
            constants::CONDZERO
        } else {
            constants::CONDPOSITIVE
        };
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
                        let immediate_value =
                            self.sign_extend_to_16_bits(instruction & 0b1_1111, 5);
                        self.registers[dest_register] =
                            self.registers[src_register] + immediate_value;
                    } else {
                        panic!("Register add, not supported yet");
                    };

                    self.update_flags(self.registers[dest_register]);
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

#[cfg(test)]
mod tests {
    use crate::computer::Computer;

    #[test]
    fn test_sign_extend_positive() {
        let c = Computer::new();
        assert_eq!(
            c.sign_extend_to_16_bits(0b0000_0000_0000_0100, 5),
            0b0000_0000_0000_0100
        );
    }

    #[test]
    fn test_sign_extend_negative() {
        let c = Computer::new();
        assert_eq!(
            c.sign_extend_to_16_bits(0b0000_0000_0001_0100, 5),
            0b1111_1111_1111_0100
        );
    }
}
