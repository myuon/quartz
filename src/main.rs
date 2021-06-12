use std::io::Read;

use lexer::run_lexer;

mod lexer;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;

    println!("{:?}", run_lexer(&buffer));

    Ok(())
}
