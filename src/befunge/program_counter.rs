use crate::befunge::memory::{MEM_WIDTH, MEM_HEIGHT};
use self::Direction::*;

#[cfg_attr(any(debug_assertions, test), derive(Debug, PartialEq))]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[cfg_attr(any(debug_assertions, test), derive(Debug))]
pub struct ProgramCounter {
    dir: Direction,
    pc: (usize, usize),
}

#[cfg(any(debug_assertions, test))]
impl PartialEq for ProgramCounter {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc
    }
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            dir: Right,
            pc: (0, MEM_WIDTH - 1),     // (row, column), initial value for first calling next()
        }
    }

    pub fn next(&mut self) {
        match self.dir {
            Up => self.pc.0 = (self.pc.0 - 1) % MEM_HEIGHT,
            Down => self.pc.0 = (self.pc.0 + MEM_HEIGHT + 1) % MEM_HEIGHT,
            Right => self.pc.1 = (self.pc.1 + 1) % MEM_WIDTH,
            Left => self.pc.1 = (self.pc.1 + MEM_WIDTH - 1) % MEM_WIDTH,
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pc
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.dir = dir;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up() {
        let orig = ProgramCounter::new();
        let mut pc2 = ProgramCounter::new();
        pc2.set_direction(Up);
        for _ in 0..MEM_HEIGHT {
            pc2.next();
        }
        debug_assert_eq!(orig, pc2);
    }

    #[test]
    fn down() {
        let orig = ProgramCounter::new();
        let mut pc2 = ProgramCounter::new();
        pc2.set_direction(Down);
        for _ in 0..MEM_HEIGHT {
            pc2.next();
        }
        debug_assert_eq!(orig, pc2);
    }

    #[test]
    fn left() {
        let orig = ProgramCounter::new();
        let mut pc2 = ProgramCounter::new();
        pc2.set_direction(Left);
        for _ in 0..MEM_WIDTH {
            pc2.next();
        }
        debug_assert_eq!(orig, pc2);
    }

    #[test]
    fn right() {
        let orig = ProgramCounter::new();
        let mut pc2 = ProgramCounter::new();
        pc2.set_direction(Right);
        for _ in 0..MEM_WIDTH {
            pc2.next();
        }
        debug_assert_eq!(orig, pc2);
    }
}
