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
pub const CONDPOSITIVE: u16 = 1;
pub const CONDZERO: u16 = 1 << 1;
pub const CONDNEGATIVE: u16 = 1 << 2;

// Traps
pub const TRAPGETC: u16 = 0x20; // get character from keyboard, not echoed onto the terminal
pub const TRAPOUT: u16 = 0x21; // output a character
pub const TRAPPUTS: u16 = 0x22; // output a word string
pub const TRAPIN: u16 = 0x23; // get character from keyboard, echoed onto the terminal
pub const TRAPPUTSP: u16 = 0x24; // output a byte string
pub const TRAPHALT: u16 = 0x25; // halt the program

// Memory mapped registers
pub const MRKBSR: usize = 0xFE00; // keyboard status
pub const MRKBDR: usize = 0xFE02; // keyboard data

// Misc
pub const DEFAULT_START: u16 = 0x3000;
