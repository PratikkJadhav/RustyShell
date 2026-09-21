
---

# Custom Rust Shell ( CodeCrafters ) 

A lightweight, POSIX-style shell built entirely from scratch in Rust. This project was developed to explore low-level Linux systems engineering concepts, focusing heavily on process lifecycle management, file descriptor manipulation, and I/O stream routing.

## Features

* **Command Execution:** Dynamically resolves and spawns external binaries located within the system's `$PATH`.
* **Robust Argument Parsing:** Custom tokenizer that correctly handles single quotes (`'`), double quotes (`"`), and backslash escapes (`\`) to process arguments containing spaces and special characters.
* **I/O Redirection:** Directly manipulates OS file descriptors to route streams to files, completely bypassing the terminal window.
* Standard Output Redirection (`>`, `1>`)
* Standard Error Redirection (`2>`)
* Append Mode (`>>`, `1>>`)


* **Asynchronous Background Jobs:** Intercepts the `&` operator to decouple child processes from the main shell thread, allowing programs to run in the background without blocking the REPL loop.
* **Core Built-ins:**
* `cd`: Directory navigation (supports absolute/relative paths and `~` for `$HOME`).
* `pwd`: Prints the current working directory.
* `echo`: Standard output printing.
* `type`: Identifies whether a command is a shell built-in or resolves to an external binary.
* `exit`: Graceful termination.



## Architecture

This shell relies strictly on the Rust Standard Library (`std`), avoiding heavy external dependencies to directly interface with Unix mechanics:

* `std::process::Command`: Used as a builder pattern to configure environment paths and arguments before spawning child processes.
* `std::process::Stdio` & `std::fs::OpenOptions`: Used to hijack standard output and standard error streams, converting raw file handles into OS-level standard I/O adapters.
* `std::os::unix::fs::PermissionsExt`: Used to verify the executable bit (`0o111`) on binaries resolved during `$PATH` traversal.

## Getting Started

Ensure you have Rust and Cargo installed, then clone the repository and run:

```bash
cargo run

```

Once inside the shell, you can execute standard Unix commands and combine them with the implemented features:

```bash
$ ls -l /tmp > output.txt
$ echo "Error log" 2> errors.txt
$ sleep 10 &
[1] 84470
$ 

```

---
