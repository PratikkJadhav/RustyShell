#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    while input != "x" {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("{}",input," :command not found");
    }

}
