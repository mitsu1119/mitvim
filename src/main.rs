use std::io::{stdin, stdout, Error, Read, Write};

fn main() -> Result<(), Error> {
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

    Ok(())
}
