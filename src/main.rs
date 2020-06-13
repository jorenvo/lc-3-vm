#![warn(clippy::all)]
const MEMORY_SIZE: usize = 65_536;
static MEMORY: &[u16] = &[0; MEMORY_SIZE];

fn main() {
    println!("Hello, world!");
}
