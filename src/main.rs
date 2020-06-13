#![warn(clippy::all)]

use crate::reader::Reader;

mod reader;

const MEMORY_SIZE: usize = 65_536;
static MEMORY: &[u16] = &[0; MEMORY_SIZE];

enum Registers {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    RPC, /* program counter */
    RCond,
    RCount,
}

const REGISTER_COUNT: usize = 16;
static REGISTERS: &[u16] = &[0; REGISTER_COUNT];

enum OpCode {
    OpBr,       /* branch */
    OpAdd,      /* add  */
    OpLoad,     /* load */
    OpStore,    /* store */
    OpJmpReg,   /* jump register */
    OpAnd,      /* bitwise and */
    OpLoadReg,  /* load register */
    OpStoreReg, /* store register */
    OpRTI,      /* unused */
    OpNot,      /* bitwise not */
    OpLoadInd,  /* load indirect */
    OpStoreInd, /* store indirect */
    OpJump,     /* jump */
    OpRes,      /* reserved (unused) */
    OpLoadEA,   /* load effective address */
    OpTrap,     /* execute trap */
}

enum CondFlags {
    Positive = 1,
    Zero = 1 << 1,
    Negative = 1 << 2,
}

fn main() {
    let r = Reader::new("/Users/macaroni/Code/lc-3/assembled/2048.obj".to_string());
    dbg!(r.read());
    println!("Hello, world!");
}
