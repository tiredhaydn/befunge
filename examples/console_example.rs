#[path = "../src/befunge/console.rs"]
mod console;

fn main() {
    use console::Console;
    let mut console = Console::new();
    let x = console.read_int();
    console.write_int(x);

    let c = console.read_char();
    console.write_char(c);

    let c = console.read_char();
    console.write_char(c);

    let c = console.read_char();
    console.write_char(c);
}
