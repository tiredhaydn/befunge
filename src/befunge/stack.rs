use std::io;
use std::io::Write;

#[cfg_attr(any(debug_assertions, test), derive(Debug, PartialEq))]
pub struct Stack {
    body: Vec<i32>
}

impl Stack {
    pub fn new() -> Stack {
        Stack { body: Vec::<i32>::new() }
    }

    pub fn push(&mut self, val: i32) {
        self.body.push(val);
    }

    pub fn pop(&mut self) -> i32 {
        assert!(self.body.len() > 0);
        self.body.pop().unwrap()
    }

    pub fn show(&self) {
        for row in 1..30 {
            print!("\x1B[{};{}H", row, 19);    // TODO: 画面用のマクロ書くかcrate探してこい
            print!("\x1B[1K");      // 行削除、カーソル以前
        }
        let mut row = 1;
        self.body.iter().for_each(|val|{
            print!("\x1B[{};{}H", row, 1);    // TODO: 画面用のマクロ書くかcrate探してこい
            print!("{:10}: '{}'", *val, *val as u8 as char);
            io::stdout().flush().unwrap();
            row += 1;
        })
    }

    #[cfg(test)]
    pub fn top(&self) -> i32 {
        *self.body.last().unwrap()
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.body.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_length_of_new_stack_is_zero() {
        debug_assert_eq!(Stack::new().len(), 0);
    }

    #[test]
    fn the_top_element_equals_the_last_pushed_element() {
        let mut stack = Stack::new();
        let elem = 123;
        stack.push(elem);
        debug_assert_eq!(stack.top(), elem);
    }

    #[test]
    fn first_in_last_out() {
        let mut stack = Stack::new();
        let elem1 = 123;
        let elem2 = 456;
        let elem3 = 789;
        stack.push(elem1);
        stack.push(elem2);
        stack.push(elem3);
        debug_assert_eq!(stack.pop(), elem3);
        debug_assert_eq!(stack.pop(), elem2);
        debug_assert_eq!(stack.pop(), elem1);
    }
}
