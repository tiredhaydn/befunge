/// https://ja.wikipedia.org/wiki/Befunge

mod console;
mod memory;
mod stack;
mod program_counter;

use crate::befunge::console::Console;
use crate::befunge::memory::Memory;
use crate::befunge::program_counter::ProgramCounter;
use crate::befunge::stack::Stack;
use std::time::{SystemTime, Duration};
use std::fs;
use std::thread::sleep;

pub struct Befunge {
    memory: Memory,
    stack: Stack,
    console: Console,
    pc: ProgramCounter,
}

impl Befunge {
    pub fn new(path: &str) -> Result<Befunge, Box<std::error::Error>> {
        let s = fs::read_to_string(path)?;
        let data = s.lines()
            .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Befunge {
            memory: Memory::new(data),
            stack: Stack::new(),
            console: Console::new(),
            pc: ProgramCounter::new(),
        })
    }

    #[allow(dead_code)]
    pub fn hello_world_sample() -> Result<Befunge, Box<std::error::Error>> {
        let data = "v @_       v\n>0\"!dlroW\"v \nv  :#     < \n>\" ,olleH\" v\n   ^       <"
            .lines()
            .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Befunge {
            memory: Memory::new(data),
            stack: Stack::new(),
            console: Console::new(),
            pc: ProgramCounter::new(),
        })
    }

    #[allow(dead_code)]
    pub fn factorial_sample() -> Result<Befunge, Box<std::error::Error>> {
        let data = "5 100p:v     \nv *g00:_00g.@\n>00p1-:^     "
            .lines()
            .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Befunge {
            memory: Memory::new(data),
            stack: Stack::new(),
            console: Console::new(),
            pc: ProgramCounter::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<std::error::Error>> {
        use self::program_counter::Direction::*;

        println!("\x1B[2J");    // clear TODO: 画面用のマクロ書くかcrate探してこい
        let time_for_rand = SystemTime::now();
        loop {
            match self.fetch() {
                // Controls
                '^' => self.pc.set_direction(Up),
                'v' => self.pc.set_direction(Down),
                '>' => self.pc.set_direction(Right),
                '<' => self.pc.set_direction(Left),
                '_' => self.pc.set_direction(if self.stack.pop() == 0 { Right } else { Left }),
                '|' => self.pc.set_direction(if self.stack.pop() == 0 { Down } else { Up }),
                '?' => {                    // random direction
                    let d = time_for_rand.elapsed().unwrap_or(Duration::new(0, 0));
                    let rand = (d.as_secs() + d.as_millis() as u64 + d.as_micros() as u64 + d.as_nanos() as u64) % 4;
                    match rand {
                        0 => self.pc.set_direction(Up),
                        1 => self.pc.set_direction(Down),
                        2 => self.pc.set_direction(Right),
                        3 => self.pc.set_direction(Left),
                        _ => unreachable!(),
                    }
                }
                ' ' => {}                   // nop
                '#' => self.pc.next(),      // skip
                '@' => break,               // halt

                // Literals
                c @ '0'...'9' => self.stack.push(c.to_digit(10).unwrap() as i32),
                '"' => {
                    loop {
                        let c = self.fetch();
                        if c == '"' { break; }
                        self.stack.push(c as i32);
                    }
                }

                // Console
                '&' => {
                    let val = self.console.read_int();
                    self.stack.push(val);
                }
                '~' => {
                    let c = self.console.read_char();
                    self.stack.push(c as i32);
                }
                '.' => {
                    let val = self.stack.pop();
                    self.console.write_int(val);
                }
                ',' => {
                    let val = self.stack.pop();
                    self.console.write_char(val as u8 as char);
                }

                // Arithmetic and Logical operations
                c @ '+' | c @ '-' |
                c @ '*' | c @ '/' | c @ '%' |
                c @ '`' => {
                    let y = self.stack.pop();
                    let x = self.stack.pop();
                    match c {
                        '+' => self.stack.push(x + y),
                        '-' => self.stack.push(x - y),
                        '*' => self.stack.push(x * y),
                        '/' => self.stack.push(x / y),
                        '%' => self.stack.push(x % y),
                        '`' => self.stack.push(if x > y { 1 } else { 0 }),
                        _ => unreachable!("unknown operator"),
                    }
                }
                '!' => {
                    let x = self.stack.pop();
                    self.stack.push(if x == 0 { 1 } else { 0 });
                }

                // Stack
                ':' => {        // duplicate
                    let x = self.stack.pop();
                    self.stack.push(x);
                    self.stack.push(x);
                }
                '\\' => {       // swap
                    let y = self.stack.pop();
                    let x = self.stack.pop();
                    self.stack.push(y);
                    self.stack.push(x);
                }
                '$' => {        // remove top
                    self.stack.pop();
                }

                // Memory
                'g' => {
                    use self::memory::{MEM_WIDTH, MEM_HEIGHT};

                    let row = self.stack.pop();
                    let col = self.stack.pop();

                    debug_assert!(0 <= row && row <= (MEM_HEIGHT - 1) as i32);
                    debug_assert!(0 <= col && col <= (MEM_WIDTH - 1) as i32);
                    let row = row as usize;
                    let col = col as usize;

                    let val = self.memory.get(row, col);
                    self.stack.push(val as i32);
                }
                'p' => {
                    use self::memory::{MEM_WIDTH, MEM_HEIGHT};

                    let row = self.stack.pop();
                    let col = self.stack.pop();
                    let val = self.stack.pop();

                    debug_assert!(0 <= row && row <= (MEM_HEIGHT - 1) as i32);
                    debug_assert!(0 <= col && col <= (MEM_WIDTH - 1) as i32);
                    debug_assert!(0 <= val && val <= 256);  // for ascii code
                    let row = row as usize;
                    let col = col as usize;

                    self.memory.put(val as u8, row, col);
                }
                c @ _ => { unreachable!("unknown character!:{:?}", c); }
            }
        }
        print!("\x1B[{};{}H", 31, 1);   // TODO: 画面用のマクロ書くかcrate探してこい
        Ok(())
    }

    fn fetch(&mut self) -> char {
        self.pc.next();
        self.memory.show(self.pc.pos());
        self.stack.show();
        sleep(Duration::from_millis(80));      // TODO: 設定ファイルとかで変更できるようにしとけや！いちいちビルドさせんな！
        let (row, col) = self.pc.pos();
        self.memory.get(row, col) as u8 as char
    }
}
