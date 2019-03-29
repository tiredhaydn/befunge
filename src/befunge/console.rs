use std::io;
use std::io::{Read, Write};

pub struct Console {
    cursor_pos: (usize, usize)  // (row, col)
}

impl Console {
    pub fn new() -> Console {
        Console { cursor_pos: (30, 1) }
    }

    pub fn read_int(&mut self) -> i32 {
        print!("\x1B[{row};{col}H", row = self.cursor_pos.0, col = self.cursor_pos.1);  // TODO: 画面用のマクロ書くかcrate探してこい
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("unknown IO error!");
        self.cursor_pos = (self.cursor_pos.0 + 1, 1);
        buffer.trim().parse::<i32>().expect("parse error!")
    }

    // TODO: \nの残りカスの扱いをどうしよう？
    //      read_lineかread_to_endで読み込んで一文字ずつ返す？
    //      カーソル位置制御の都合上、上記方法が必須か？
    //      それにしてもやっつけ感がひどいコードや…
    pub fn read_char(&mut self) -> char {
        print!("\x1B[{row};{col}H", row = self.cursor_pos.0, col = self.cursor_pos.1);  // TODO: 画面用のマクロ書くかcrate探してこい
        io::stdout().flush().unwrap();
        let result = io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .unwrap() as char;
        if result == '\n' { self.cursor_pos = (self.cursor_pos.0 + 1, 1); }
        result
    }

    pub fn write_int(&mut self, val: i32) {
        print!("\x1B[{row};{col}H", row = self.cursor_pos.0, col = self.cursor_pos.1);  // TODO: 画面用のマクロ書くかcrate探してこい
        self.cursor_pos.1 += val.to_string().len() + 1;
        print!("{} ", val);
        io::stdout().flush().unwrap();
    }

    pub fn write_char(&mut self, c: char) {
        print!("\x1B[{row};{col}H", row = self.cursor_pos.0, col = self.cursor_pos.1);  // TODO: 画面用のマクロ書くかcrate探してこい
        self.cursor_pos.1 += 1;
        print!("{}", c);
        io::stdout().flush().unwrap();
    }
}
