use std::io::{stdin, stdout, Error, Read, Write};

use terminal::Terminal;

mod terminal;

fn main() -> Result<(), Error> {
    Terminal::to_raw_mode()?;

    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = [0];
        stdin().read(&mut input)?;

        match input[0] {
            b'q' => break,
            _ => {
                println!("{:?}", input);
            }
        }
    }

    Terminal::to_cooked_mode()?;

    Ok(())
}
