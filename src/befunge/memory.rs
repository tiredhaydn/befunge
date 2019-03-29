pub const MEM_WIDTH: usize = 80;
pub const MEM_HEIGHT: usize = 25;

#[cfg_attr(any(debug_assertions, test), derive(Debug))]
pub struct Memory {
    mem: Vec<Vec<u8>>,
}

impl Memory {
    pub fn new(mut data: Vec<Vec<u8>>) -> Memory {
        data.iter_mut().for_each(|line| {
            line.resize(MEM_WIDTH, ' ' as u8);
        });
        data.resize_with(MEM_HEIGHT, || {
            vec![' ' as u8; MEM_WIDTH]
        });
        Memory { mem: data }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.mem[row][col]
    }

    pub fn put(&mut self, val: u8, row: usize, col: usize) {
        self.mem[row][col] = val as u8;
    }

    pub fn show(&self, pc_pos: (usize, usize)) {
        for row in 0..MEM_HEIGHT {
            print!("\x1B[{};{}H", row + 1, 20);    // TODO: 画面用のマクロ書くかcrate探してこい
            for col in 0..MEM_WIDTH {
                if (row, col) == pc_pos {
                    print!("\x1B[7m");  // inverse TODO: 画面用のマクロ書くかcrate探してこい
                }
                let c = self.mem[row][col] as char;
                match c.is_ascii_graphic() {
                    true => print!("{}", c),
                    false => print!(" "),
                }
                print!("\x1B[0m");      // style reset TODO: 画面用のマクロ書くかcrate探してこい
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use crate::befunge::memory::{MEM_WIDTH, MEM_HEIGHT};

    #[test]
    fn get() {
        let data = "v @_       v\n>0\"!dlroW\"v \nv  :#     < \n>\" ,olleH\" v\n   ^       <"
            .lines()
            .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mem = Memory::new(data);

        let mut output = vec![vec![' '; MEM_WIDTH]; MEM_HEIGHT];
        for mut line in output {
            for cell in line.iter_mut() {
                *cell = mem.get(row, col) as u8 as char;
            }
        }
        let mut expected = vec![
            vec!['v', ' ', '@', '_', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'v'],
            vec!['>', '0', '"', '!', 'd', 'l', 'r', 'o', 'W', '"', 'v', ' '],
            vec!['v', ' ', ' ', ':', '#', ' ', ' ', ' ', ' ', ' ', '<', ' '],
            vec!['>', '"', ' ', ',', 'o', 'l', 'l', 'e', 'H', '"', ' ', 'v'],
            vec![' ', ' ', ' ', '^', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '<'],
        ];
        expected.iter_mut().for_each(|line| {
            line.resize(MEM_WIDTH, ' ');
        });
        expected.resize_with(MEM_HEIGHT, || {
            vec![' '; MEM_WIDTH]
        });
        debug_assert_eq!(output, expected);
    }
}
