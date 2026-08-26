#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_string() == "exit" {
            break;
        }

        if input.trim().to_string().starts_with == "echo" {
            println!("{}", &input[5:]);
        }else if {
            println!("{}: command not found", input.trim())
        }

    }


}
