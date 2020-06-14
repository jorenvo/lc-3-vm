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
pub const OPBR: usize = 0; // branch
pub const OPADD: usize = 1; // add
pub const OPLOAD: usize = 2; // load
pub const OPSTORE: usize = 3; // store
pub const OPJMPREG: usize = 4; // jump register
pub const OPAND: usize = 5; // bitwise and
pub const OPLOADREG: usize = 6; // load register
pub const OPSTOREREG: usize = 7; // store register
pub const OPRTI: usize = 8; // unused
pub const OPNOT: usize = 9; // bitwise not
pub const OPLOADIND: usize = 10; // load indirect
pub const OPSTOREIND: usize = 11; // store indirect
pub const OPJUMP: usize = 12; // jump
pub const OPRES: usize = 13; // reserved (unused)
pub const OPLOADEA: usize = 14; // load effective address
pub const OPTRAP: usize = 15; // execute trap

// Condition flags
pub const CONDPOSITIVE: u16 = 1;
pub const CONDZERO: u16 = 1 << 1;
pub const CONDNEGATIVE: u16 = 1 << 2;

// Misc
pub const DEFAULT_START: u16 = 0x3000;
