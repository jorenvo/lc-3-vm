#![warn(clippy::all)]

use crate::args::LC3Args;
use crate::computer::Computer;
use crate::reader::Reader;

mod args;
mod computer;
mod constants;
mod instruction;
mod reader;

fn main() {
    let args = LC3Args::parse();
    let reader = Reader::new(args.path_to_assembled);
    let words = reader.read();
    let mut computer = Computer::new();

    computer.init_memory(words);
    computer.run();
}
