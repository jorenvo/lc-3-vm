use crate::constants;
use crate::instruction::Instruction;
use getch::Getch;
use std::io::Write;
use std::os::unix::io::AsRawFd;

pub struct Computer {
    memory: Vec<u16>,
    registers: Vec<u16>,
    running: bool,
    debug_mode: bool,
    getch: Getch,
}

impl Computer {
    pub fn new(debug_mode: bool) -> Computer {
        const MEMORY_SIZE: usize = 65_535; // 2^16 - 1
        const REGISTER_COUNT: usize = 16;
        let memory: Vec<u16> = vec![0; MEMORY_SIZE];
        let registers: Vec<u16> = vec![0; REGISTER_COUNT];
        let running = true;

        // Instantiate this early. It has the side-effect of disabling line-buffering in the terminal.
        let getch = Getch::new();

        Computer {
            memory,
            registers,
            running,
            debug_mode,
            getch,
        }
    }

    fn debug_println(&self, s: &str) {
        if self.debug_mode {
            println!("[DEBUG] {}", s);
        }
    }

    fn print_registers(&self, registers: &[usize]) {
        if self.debug_mode {
            for register in registers {
                self.debug_println(&format!(
                    "{:>2}: {:#018b} {:#06x} {:05}",
                    register,
                    self.registers[*register],
                    self.registers[*register],
                    self.registers[*register]
                ));
            }
        }
    }

    unsafe fn key_waiting(&self) -> bool {
        let nfds = 1;
        let null = std::ptr::null_mut();
        let mut readfds: libc::fd_set = std::mem::zeroed();
        libc::FD_ZERO(&mut readfds);
        libc::FD_SET(std::io::stdin().as_raw_fd(), &mut readfds);

        let mut timeout = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        libc::select(nfds, &mut readfds, null, null, &mut timeout) > 0
    }

    fn read_mem(&mut self, address: usize) -> u16 {
        if address == constants::MRKBSR {
            let key_waiting = unsafe { self.key_waiting() };
            if key_waiting {
                self.memory[constants::MRKBSR] = 1 << 15;
                self.memory[constants::MRKBDR] = self.getch.getch().unwrap() as u16;
            } else {
                self.memory[constants::MRKBSR] = 0;
            }
        }

        self.memory[address as usize]
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

    fn halt(&mut self) {
        self.running = false;
    }

    fn handle_trap(&mut self, trap: u16) {
        match trap {
            constants::TRAPPUTS => {
                let mut string_addr = self.registers[constants::R0] as usize;

                while self.read_mem(string_addr) != 0 {
                    print!("{}", self.read_mem(string_addr) as u8 as char);
                    string_addr += 1;
                }

                std::io::stdout().flush().unwrap();
            }

            constants::TRAPOUT => {
                print!("{}", self.registers[constants::R0] as u8 as char);
                std::io::stdout().flush().unwrap();
            }

            constants::TRAPGETC => {
                let c = self.getch.getch().unwrap() as char;
                self.registers[constants::R0] = c as u16;
            }

            constants::TRAPHALT => {
                self.debug_println("Program exited.");
                self.halt();
            }

            _ => panic!("Trap {} not implemented", trap),
        }
    }

    pub fn init_memory(&mut self, words: Vec<u16>) {
        let origin = words[0] as usize;
        self.memory[origin..(origin + words.len() - 1)].copy_from_slice(&words[1..]);
    }

    pub fn run(&mut self) {
        self.registers[constants::RPC] = constants::DEFAULT_START;

        let watched_registers = vec![
            constants::R0,
            constants::R1,
            constants::R4,
            constants::RPC,
            constants::RCOND,
        ];
        while self.running {
            let inst = Instruction::new(self.read_mem(self.registers[constants::RPC] as usize));

            // immediately set RPC to the next instruction, offsets relative to the program counter are always +1
            self.registers[constants::RPC] += 1;

            self.debug_println(&format!("Processing instruction {}", inst));

            if inst.all() == 0 {
                self.debug_println("0 instruction, stopping");
                self.halt();
                continue;
            }

            let opcode = inst.opcode();
            match opcode {
                constants::OPADD | constants::OPAND => {
                    if inst.is_imm() {
                        let immediate_value = self.sign_extend_to_16_bits(inst.imm5(), 5);

                        match opcode {
                            constants::OPADD => {
                                self.debug_println("got immediate add");
                                self.registers[inst.dr()] =
                                    self.registers[inst.sr1()].wrapping_add(immediate_value)
                            }
                            _ => {
                                // constants::OPAND
                                self.debug_println("got immediate and");
                                self.registers[inst.dr()] =
                                    self.registers[inst.sr1()] & immediate_value
                            }
                        }
                    } else {
                        match opcode {
                            constants::OPADD => {
                                self.debug_println("got add");
                                self.registers[inst.dr()] = self.registers[inst.sr1()]
                                    .wrapping_add(self.registers[inst.sr2()])
                            }
                            _ => {
                                // constants::OPAND
                                self.debug_println("got and");
                                self.registers[inst.dr()] =
                                    self.registers[inst.sr1()] & self.registers[inst.sr2()]
                            }
                        }
                    };

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPBR => {
                    self.debug_println("got opbr");

                    if (inst.n_flag()
                        && self.registers[constants::RCOND] == constants::CONDNEGATIVE)
                        || (inst.z_flag()
                            && self.registers[constants::RCOND] == constants::CONDZERO)
                        || (inst.p_flag()
                            && self.registers[constants::RCOND] == constants::CONDPOSITIVE)
                    {
                        self.debug_println("branching");
                        let pc_offset = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                        self.registers[constants::RPC] =
                            self.registers[constants::RPC].wrapping_add(pc_offset);
                    }
                }

                constants::OPJUMP => {
                    self.debug_println("got jump");
                    let mut reg = inst.base_r();

                    if reg == 0b111 {
                        reg = constants::R7;
                    }

                    self.registers[constants::RPC] = self.registers[reg];
                }

                constants::OPJMPSUBR => {
                    self.debug_println("got jump subr");
                    self.registers[constants::R7] = self.registers[constants::RPC];

                    self.registers[constants::RPC] = if inst.is_base_r() {
                        self.registers[inst.base_r()]
                    } else {
                        self.registers[constants::RPC]
                            .wrapping_add(self.sign_extend_to_16_bits(inst.pc_offset11(), 11))
                    };
                }

                constants::OPLOAD => {
                    self.debug_println("got load");
                    let pc_offset = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    let address = self.registers[constants::RPC].wrapping_add(pc_offset);
                    self.registers[inst.dr()] = self.read_mem(address as usize);

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPLOADIND => {
                    self.debug_println("got oploadind");
                    let pc_offset = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    let addr = self
                        .read_mem(self.registers[constants::RPC].wrapping_add(pc_offset) as usize);
                    self.registers[inst.dr()] = self.read_mem(addr as usize);

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPLOADREG => {
                    self.debug_println("got oploadreg");
                    let offset6 = self.sign_extend_to_16_bits(inst.offset6(), 6);
                    self.registers[inst.dr()] =
                        self.read_mem(self.registers[inst.base_r()].wrapping_add(offset6) as usize);

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPLOADEA => {
                    self.debug_println("got oploadea");
                    let pc_offset9 = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    self.registers[inst.dr()] =
                        self.registers[constants::RPC].wrapping_add(pc_offset9);

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPNOT => {
                    self.debug_println("got opnot");
                    self.registers[inst.dr()] = !self.registers[inst.sr1()];

                    self.update_flags(self.registers[inst.dr()]);
                }

                constants::OPSTORE => {
                    self.debug_println("got opstore");
                    let pc_offset9 = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    let address = self.registers[constants::RPC].wrapping_add(pc_offset9);
                    self.memory[address as usize] = self.registers[inst.dr()]; // this is really a source register, but it's in the position of the destination register
                }

                constants::OPSTOREIND => {
                    self.debug_println("got opstoreind");
                    let pc_offset9 = self.sign_extend_to_16_bits(inst.pc_offset9(), 9);
                    let offset = self.registers[constants::RPC] + pc_offset9;

                    let address = self.read_mem(offset as usize);
                    self.memory[address as usize] = self.registers[inst.dr()]; // this is really a source register
                }

                constants::OPSTOREREG => {
                    self.debug_println("got opstorereg");
                    let offset6 = self.sign_extend_to_16_bits(inst.offset6(), 6);
                    let address = self.registers[inst.base_r()].wrapping_add(offset6);
                    self.memory[address as usize] = self.registers[inst.dr()]; // this is really a source register
                }

                constants::OPTRAP => {
                    self.debug_println("got optrap");
                    self.registers[constants::R7] = self.registers[constants::RPC];
                    self.handle_trap(inst.trap_vect8());
                }

                constants::OPRTI | constants::OPRES => {
                    panic!("OPRTI and OPRES not currently implemented");
                }

                _ => {
                    self.debug_println(&format!("Bad opcode {:#06b}", opcode));
                    self.halt();
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
        let c = Computer::new(false);
        assert_eq!(
            c.sign_extend_to_16_bits(0b0000_0000_0000_0100, 5),
            0b0000_0000_0000_0100
        );
    }

    #[test]
    fn test_sign_extend_negative() {
        let c = Computer::new(false);
        assert_eq!(
            c.sign_extend_to_16_bits(0b0000_0000_0001_0100, 5),
            0b1111_1111_1111_0100
        );
    }
}
