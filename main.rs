use std::{io::{self, Write}};

fn main() -> io::Result<()>{
    let mut term = terminal::stdout();
    term.write_all(b"a whole new world")?;

    Ok(())
}
