#![allow(dead_code)]
// Registers
pub const R0: usize = 0;
pub const R1: usize = 1;
pub const R2: usize = 2;
pub const R3: usize = 3;
pub const R4: usize = 4;
pub const R5: usize = 5;
pub const R6: usize = 6;
pub const R7: usize = 7;
pub const RPC: usize = 8;
pub const RCOND: usize = 9;
pub const RCOUNT: usize = 10;

// Opcodes
pub const OPBR: u16 = 0; // branch
pub const OPADD: u16 = 1; // add
pub const OPLOAD: u16 = 2; // load
pub const OPSTORE: u16 = 3; // store
pub const OPJMPSUBR: u16 = 4; // jump register
pub const OPAND: u16 = 5; // bitwise and
pub const OPLOADREG: u16 = 6; // load register
pub const OPSTOREREG: u16 = 7; // store register
pub const OPRTI: u16 = 8; // unused
pub const OPNOT: u16 = 9; // bitwise not
pub const OPLOADIND: u16 = 10; // load indirect
pub const OPSTOREIND: u16 = 11; // store indirect
pub const OPJUMP: u16 = 12; // jump
pub const OPRES: u16 = 13; // reserved (unused)
pub const OPLOADEA: u16 = 14; // load effective address
pub const OPTRAP: u16 = 15; // execute trap

// Condition flags
pub const CONDPOSITIVE: usize = 1;
pub const CONDZERO: usize = 1 << 1;
pub const CONDNEGATIVE: usize = 1 << 2;

// Misc
pub const DEFAULT_START: u16 = 0x3000;
