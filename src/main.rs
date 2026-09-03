#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();
            command = command.trim().to_string();
            if command == "exit" {
                break;
            } else if command.starts_with("echo ") {
                println!("{}", &command[5..]);
            } else if command.starts_with("type ") {
                let c = &command[5..];
                if c == "echo" || c == "exit" || c=="type"{
                    println!("{} is a shell builtin", c );
                }else{
                    println!("{}: not found", c );
                }
            }else {
                println!("{}: command not found", command);
            }
        }



}
