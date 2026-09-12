use std::env::set_current_dir;
use std::fs::OpenOptions;
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

        if command.is_empty() {
            continue;
        }
        let parsed_args = arguements(&command);
        if parsed_args.is_empty() {
            continue;
        }

        let cmd = &parsed_args[0];
        let args_vec = &parsed_args[1..];
        let mut args = args_vec.join(" ");
        let mut file = None;
        let mut redir = None;

        if let Some(index) = args_vec.iter().position(|args| {
            args == ">" || args == "1>" || args == "2>" || args == ">>" || args == "1>>"
        }) {
            let filename = &args_vec[index + 1];

            if args_vec[index] == ">>" || args_vec[index] == "1>>" {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(filename)
                    .unwrap();
                redir = Some(1);
            } else {
                if args_vec[index] == "2>" {
                    redir = Some(2);
                } else {
                    redir = Some(1);
                }
                file = Some(std::fs::File::create(filename).unwrap());
            }

            args = args_vec[..index].join(" ");
        }

        if cmd == "exit" {
            break;
        } else if cmd == "cd" {
            if args == "~" {
                if set_current_dir(env::var("HOME").unwrap()).is_err() {};
            } else {
                if set_current_dir(&args).is_err() {
                    println!("{}: No such file or directory", args);
                };
            }
        } else if cmd == "echo" {
            if redir == Some(1) {
                writeln!(file.as_mut().unwrap(), "{}", args).unwrap();
            } else {
                println!("{}", args);
            }
        } else if cmd == "pwd" {
            let current_dir = env::current_dir().unwrap();
            if redir == Some(1) {
                writeln!(file.as_mut().unwrap(), "{}", current_dir.display()).unwrap();
            } else {
                println!("{}", current_dir.display());
            }
        } else if cmd == "type" {
            if args == "echo" || args == "pwd" || args == "cd" || args == "exit" || args == "type" {
                println!("{} is a shell builtin", args);
            } else {
                exec(&args);
            }
        } else {
            let clean_args = if let Some(index) = args_vec
                .iter()
                .position(|a| a == ">" || a == "1>" || a == "2>" || a == ">>" || a == "1>>")
            {
                &args_vec[..index]
            } else {
                args_vec
            };

            let mut command_builder = Command::new(cmd);
            command_builder.args(clean_args);

            if let Some(out_file) = file {
                if redir == Some(1) {
                    command_builder.stdout(std::process::Stdio::from(out_file));
                } else {
                    command_builder.stderr(std::process::Stdio::from(out_file));
                }
            }
            match command_builder.spawn() {
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

fn arguements(args: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut in_double_quotes = false;
    let mut in_single_quotes: bool = false;
    let mut is_slash: bool = false;
    let mut current_word = String::new();

    for i in args.chars() {
        if is_slash {
            if in_double_quotes {
                if i == '"' || i == '\\' || i == '$' {
                    current_word.push(i);
                } else {
                    current_word.push('\\');
                    current_word.push(i);
                }
            } else {
                current_word.push(i);
            }
            is_slash = false;
            continue;
        }

        if i == '\\' && !in_single_quotes {
            is_slash = true;
            continue;
        }
        if i == '"' && !in_single_quotes {
            in_double_quotes = !in_double_quotes;
            continue;
        }

        if i == '\'' && !in_double_quotes {
            in_single_quotes = !in_single_quotes;
            continue;
        }

        if i == ' ' {
            if !in_double_quotes && !in_single_quotes {
                if !current_word.is_empty() {
                    v.push(current_word.clone());
                    current_word.clear();
                }
                continue;
            }
        }
        current_word.push(i);
    }
    if !current_word.is_empty() {
        v.push(current_word);
    }

    v
}
