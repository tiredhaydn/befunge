mod befunge;

use befunge::Befunge;
use std::{env, error};

fn main() -> Result<(), Box<error::Error>> {
    let path = env::args().nth(1).ok_or("usage: befunge SOURCE")?;
    let mut befunge = Befunge::new(path.as_str())?;
    match befunge.run() {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("\x1B[31;1H");
            eprintln!("{}", err.description());
            Err(err)
        },
    }
}
