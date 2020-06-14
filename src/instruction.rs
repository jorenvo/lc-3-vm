use std::fmt;

pub struct Instruction {
    content: u16,
}

impl Instruction {
    pub fn new(content: u16) -> Instruction {
        Instruction { content }
    }

    /// Extracts specified chunk
    ///
    /// * `start`: start of chunk, inclusive
    /// * `end`: end of chunk, inclusive
    pub fn chunk(&self, start: u16, end: u16) -> u16 {
        let mask: u16 = u16::MAX;
        let length = end - start + 1;
        self.content >> start & mask >> (16 - length)
    }

    pub fn all(&self) -> u16 {
        self.content
    }

    pub fn opcode(&self) -> u16 {
        self.chunk(12, 15)
    }

    pub fn dr(&self) -> usize {
        self.chunk(9, 11) as usize
    }

    pub fn sr1(&self) -> usize {
        self.chunk(6, 8) as usize
    }

    pub fn base_r(&self) -> usize {
        self.sr1()
    }

    pub fn sr2(&self) -> usize {
        self.chunk(0, 2) as usize
    }

    pub fn is_imm(&self) -> bool {
        self.chunk(5, 5) > 0
    }

    pub fn imm5(&self) -> u16 {
        self.chunk(0, 4)
    }

    pub fn pc_offset9(&self) -> u16 {
        self.chunk(0, 8)
    }

    pub fn pc_offset11(&self) -> u16 {
        self.chunk(0, 10)
    }

    pub fn n_flag(&self) -> bool {
        self.chunk(11, 11) > 0
    }

    pub fn z_flag(&self) -> bool {
        self.chunk(10, 10) > 0
    }

    pub fn p_flag(&self) -> bool {
        self.chunk(9, 9) > 0
    }

    pub fn is_base_r(&self) -> bool {
        self.n_flag()
    }

    pub fn offset6(&self) -> u16 {
        self.chunk(0, 5)
    }

    pub fn trap_vect8(&self) -> u16 {
        self.chunk(0, 7)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#018b}", self.content)
    }
}

#[cfg(test)]
mod test {
    use crate::instruction::Instruction;

    #[test]
    fn test_get_chunks_of_add() {
        let inst = Instruction::new(0b0001_1110_1011_0111);
        assert_eq!(inst.chunk(12, 15), 0b0001);
        assert_eq!(inst.chunk(9, 11), 0b111);
        assert_eq!(inst.chunk(6, 8), 0b010);
        assert_eq!(inst.chunk(5, 5), 0b1);
        assert_eq!(inst.chunk(0, 4), 0b1_0111);
    }
}
