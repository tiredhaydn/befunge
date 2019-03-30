mod befunge;

use befunge::Befunge;
use std::env;

fn main() -> Result<(), Box<std::error::Error>> {
    let path = env::args().nth(1);
    if let None = path {
        eprintln!("usage: befunge SOURCE");
        return Ok(())
    }
    let mut befunge = Befunge::new(path.unwrap().as_str())?;
    befunge.run()
}
