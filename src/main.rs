use std::env::current_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{env, fs, os::unix::fs::PermissionsExt, path::Path};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        let mut parse = command.splitn(2, char::is_whitespace);
        let cmd = parse.next().unwrap();
        let args = parse.next().unwrap_or("").trim_start();
        if cmd == "exit" {
            break;
        } else if cmd == "echo" {
            println!("{args}");
        } else if cmd == "pwd" {
            let current_dir = env::current_dir().unwrap();
            println!("{} hjello 1 2 3 check check check", current_dir.display());
        } else if cmd == "type" {
            if args == "echo" || args == "exit" || args == "type" {
                println!("{} is a shell builtin", args);
            } else {
                exec(args);
            }
        } else {
            match Command::new(cmd).args(args.split_whitespace()).spawn() {
                Ok(mut child) => {
                    child.wait().unwrap();
                }
                Err(_) => {
                    println!("{}: command not found", cmd);
                }
            }
        }
    }
}

fn exec(args: &str) {
    let path = env::var("PATH").unwrap();
    let paths: Vec<&str> = path.split(":").collect();
    for dir in paths {
        let file = Path::new(dir).join(args);
        if file.is_file() {
            if let Ok(metadata) = fs::metadata(&file) {
                if metadata.permissions().mode() & 0o111 != 0 {
                    println!("{} is {}", args, file.to_str().unwrap());
                    return;
                }
            }
        }
    }

    println!("{}: not found", args);
}
