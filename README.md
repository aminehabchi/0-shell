# 0-Shell

A minimalist Unix-like shell built in **Rust** — no `bash`, no `sh`, just pure system-level control.

---

## 📖 About This Project

**0-Shell** is a lightweight, standalone shell inspired by tools like **BusyBox**, built for embedded Linux environments. It uses **Rust** to implement core shell commands **from scratch**, without relying on external programs.

---

## 🎯 What You’ll Build

A fully working shell that:

- Displays a prompt like `$`
- Accepts and parses user input
- Executes built-in commands
- Handles errors gracefully
- Exits on `exit` or `Ctrl+D`

---

## 🧠 What You’ll Learn

- How to use Rust’s standard and low-level system libraries
- How to write your own versions of Unix commands
- How to build a modular command-execution loop

---

## 🗂 Project Structure

This is a **Rust workspace**:

- Each shell command is its own **library crate** (like `cat`, `rm`, `cd`, etc.)
- The main shell logic is in a binary crate called `terminal`
- The `terminal` crate imports and uses the command crates

---

## 🧰 Built-In Commands

| Command | Description                              |
|---------|------------------------------------------|
| `echo`  | Print text to stdout                     |
| `cd`    | Change the current working directory     |
| `ls`    | List directory contents (`-l`, `-a`, `-F`) |
| `pwd`   | Show the current working directory       |
| `cat`   | Display file contents                    |
| `cp`    | Copy files                               |
| `rm`    | Remove files or directories (`-r` flag)  |
| `mv`    | Move or rename files                     |
| `mkdir` | Create directories                       |
| `exit`  | Exit the shell                           |

---

## ⚙️ Getting Started

### ✅ Clone the Repository

```bash
git clone https://github.com/aminehabchi/0-shell
cd 0-shell
cargo run -p terminal
```

---
## 👥 Team Members & Responsibilities

| Name                  | Assigned Commands           |
|-----------------------|-----------------------------|
| **Ali Louhab**        | `cat`, `rm`, `exit`, `mkdir` |
| **Amine Habchi**      | `mv`, `cp`, `pwd`, `ls`     |
| **Abdelouahab Bouchik** | `cd`, `echo`              |
---

## 🙏 Special Thanks

Thanks to [@aminehabchi](https://github.com/aminehabchi) for setting up the workspace and kicking off the project.
