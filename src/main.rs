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
        command = input.trim().to_string();
                if command == "exit" {
                    break;
                } else if command.starts_with("echo ") {
                    println!("{}", &command[5..]);
                } else {
                    println!("{}: command not found", command);
                }

    }


}
