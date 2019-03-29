mod befunge;

use befunge::Befunge;
use std::env;

fn main() -> Result<(), Box<std::error::Error>> {
    let path = env::args().nth(1).unwrap();
    let mut befunge = Befunge::new(path.as_str())?;
    befunge.run()
}
