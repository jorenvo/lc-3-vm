use crate::constants;
use crate::instruction::Instruction;

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
        } as u16;
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
            let inst = Instruction::new(self.memory[self.registers[constants::RPC] as usize]);

            // immediately go to next instruction, LDI assumes this
            self.registers[constants::RPC] += 1;

            println!("Processing instruction {}", inst);

            if inst.all() == 0 {
                println!("0 instruction, stopping");
                running = false;
                continue;
            }

            let opcode = inst.opcode();
            match opcode {
                constants::OPADD | constants::OPAND => {
                    if inst.is_imm() {
                        let immediate_value = self.sign_extend_to_16_bits(inst.imm5(), 5);

                        match opcode {
                            constants::OPADD => {
                                println!("got immediate add");
                                self.registers[inst.dr()] =
                                    self.registers[inst.sr1()] + immediate_value
                            }
                            _ => {
                                // constants::OPAND
                                println!("got immediate and");
                                self.registers[inst.dr()] =
                                    self.registers[inst.sr1()] & immediate_value
                            }
                        }
                    } else {
                        match opcode {
                            constants::OPADD => {
                                println!("got add");
                                self.registers[inst.dr()] =
                                    self.registers[inst.sr1()] + self.registers[inst.sr2()]
                            }
                            _ => {
                                // constants::OPAND
                                println!("got and");
                                self.registers[inst.dr()] =
                                    self.registers[inst.sr1()] & self.registers[inst.sr2()]
                            }
                        }
                    };

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPBR => {
                    println!("got opbr");
                    let pc_offset = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);

                    if (inst.n_flag() && self.registers[constants::CONDNEGATIVE] > 0)
                        || (inst.z_flag() && self.registers[constants::CONDZERO] > 0)
                        || (inst.p_flag() && self.registers[constants::CONDPOSITIVE] > 0)
                    {
                        println!("branching");
                        self.registers[constants::RPC] += pc_offset;
                    }
                }

                constants::OPJUMP => {
                    println!("got jump");
                    let mut reg = inst.base_r();

                    if reg == 0b111 {
                        reg = constants::R7;
                    }

                    self.registers[constants::RPC] = self.registers[reg];
                }

                constants::OPJMPSUBR => {
                    println!("got jump subr");
                    self.registers[constants::R7] = self.registers[constants::RPC];

                    self.registers[constants::RPC] = if inst.is_base_r() {
                        self.registers[inst.base_r()]
                    } else {
                        self.registers[constants::RPC]
                            + self.sign_extend_to_16_bits(inst.pc_offset11(), 11)
                    }
                }

                constants::OPLOAD => {
                    println!("got load");
                    let pc_offset = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    self.registers[inst.dr()] =
                        self.memory[(self.registers[constants::RPC] + pc_offset) as usize];

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPLOADIND => {
                    println!("got oploadind");
                    let pc_offset = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    let addr = self.memory[(self.registers[constants::RPC] + pc_offset) as usize];
                    self.registers[inst.dr()] = self.memory[addr as usize];

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPLOADREG => {
                    println!("got oploadreg");
                    let offset6 = self.sign_extend_to_16_bits(inst.offset6(), 6);
                    self.registers[inst.dr()] =
                        self.memory[(self.registers[inst.base_r()] + offset6) as usize];

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPLOADEA => {
                    println!("got oploadea");
                    let pc_offset9 = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    self.registers[inst.dr()] = self.registers[constants::RPC] + pc_offset9;

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPNOT => {
                    println!("got opnot");
                    self.registers[inst.dr()] = !self.registers[inst.sr1()];

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPSTORE => {
                    let pc_offset9 = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    self.memory[(self.registers[constants::RPC] + pc_offset9) as usize] =
                        self.registers[inst.dr()]; // this is really a source register, but it's in the position of the destination register
                }

                constants::OPRTI | constants::OPRES => {
                    panic!("OPRTI and OPRES not currently implemented");
                }

                _ => {
                    println!(
                        "Stopping computer because of unsupported opcode {:#06b}",
                        opcode
                    );

                    running = false;
                }
            };

            self.print_registers(&watched_registers);
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
