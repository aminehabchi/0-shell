# 🐚 0-Shell

A minimalist Unix-like shell written in **Rust**, built from scratch to replicate essential Unix commands without relying on external binaries like `bash` or `sh`.

---

## 📖 Overview

This project aims to build a lightweight, standalone shell inspired by tools like **BusyBox**, tailored for embedded Linux environments. You’ll implement core shell commands directly using **Rust’s system-level capabilities**, including file handling, process management, and error handling.

---

## 🎯 Objectives

- Master file and directory manipulation using Rust's standard and low-level libraries.
- Build a working shell that:
  - Displays a prompt (`$`)
  - Accepts and parses input
  - Executes commands and returns to prompt after execution
  - Exits gracefully on `exit` command or `Ctrl+D`

---

## 🧠 Learning Goals

- Use Unix syscalls and Rust's abstractions safely
- Implement custom shell commands from scratch
- Create and manage a command execution loop
- Handle error states and invalid input robustly

---

## 🗃️ Project Structure

This is a **Rust workspace**, where each shell command is a separate **library crate**, and the shell core logic is in a **binary crate** named `terminal`.

Each command crate contains only functions (no `main()`), and is consumed inside `terminal`'s main loop.

---
## 🔧 Commands to Implement

| Command | Requirements                                |
|---------|---------------------------------------------|
| echo    | Print args to stdout                        |
| cd      | Change directory                            |
| ls      | Support `-l`, `-a`, `-F`                     |
| pwd     | Print current working directory             |
| cat     | Concatenate and display file content        |
| cp      | Copy files                                  |
| rm      | Remove files/directories (`-r` support)     |
| mv      | Move or rename files                        |
| mkdir   | Create new directories                      |
| exit    | Exit the shell                              |
## ⚙️ Setup

### ✅ Clone the Project

```bash
git clone https://github.com/aminehabchi/0-shell
cd 0-shell

✅ Run the Shell
cargo run -p terminal
🧪 Running a Single Command

Each command crate (like cat, pwd) is a library. You cannot run them directly, but they are invoked from terminal.


👥 Team Tasks
AliLouhab cat, rm , exit
AmineHabchi	mv, cp , pwd
AbdelouahabBouchik	cd, echo
IsmailIchi	ls, mkdir

🚀 Workflow Instructions
Create a library crate for your command:

cargo new your-command --lib
Cargo will auto-add your crate to the [workspace] members in Cargo.toml.
add your library crate path to the binary crate toml dependencies
rm = { path = "../echo" }

Write your logic in src/lib.rs and expose functions like:

pub fn cat(args: &[&str]) -> Result<(), String> { ... }
Use your command in terminal/src/main.rs by importing it:
use cat::cat;
Commit & push your changes:
git add .
git commit -m "Implement cat command"
git push origin your-branch



🤝 Contribution Guidelines
Always work in your own branch.

Every command lives in its own library crate.

Keep terminal/src/main.rs as the central loop for executing commands.


🧾 License
MIT License. Feel free to work, modify, and use your features.

✨ Special Thanks
Shoutout to @aminehabchi for initializing the project repo and leading the workspace setup.