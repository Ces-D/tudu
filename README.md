# Tudu CLI 🦀

**Your friendly, fast, and simple command-line task manager.**

Tudu is a CLI tool written in Rust to help you manage your todos and projects directly from your terminal. Never leave your keyboard to manage your tasks again!

[![Build Status](https://img.shields.io/github/actions/workflow/status/your-repo/rust.yml?branch=main&style=for-the-badge)](https://github.com/your-repo/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/tudu?style=for-the-badge)](https://crates.io/crates/tudu)

---

## ✨ Features

- **Create, Update, and Close Todos:** Manage the entire lifecycle of your tasks.
- **Project Organization:** Group your todos into projects for better organization.
- **View and List:** Get a clear overview of your tasks, either individually or as a list.
- **Persistent Storage:** Your tasks are saved in a local SQLite database.
- **Fast and Efficient:** Built with Rust for performance and reliability.
- **Simple Command Structure:** Intuitive commands that are easy to remember.

---

## 🚀 Installation

Ensure you have Rust and Cargo installed on your system.

You can install `tudu` directly from the source:

```bash
# Clone the repository
git clone 
cd tudu

# Install the binary
cargo install --path .
```

---

## ⚙️ Usage

Tudu uses a simple `COMMAND -> SUBCOMMAND` structure.

### Initial Setup

Before you start, run the database migrations to set up the local database:

```bash
tudu migrations
```
This will create the database file if it doesn't exist and run any pending migrations.

### Core Commands

Here are the main commands available in `tudu`:

#### `new`
Create a new todo or project.

- **Create a new todo:**
  ```bash
  # tudu new todo [OPTIONS] <TITLE>
  tudu new todo "Finish the README file" --project "Tudu Project"
  ```

- **Create a new project:**
  ```bash
  # tudu new project <NAME>
  tudu new project "Tudu Project"
  ```

#### `list`
List all your todos.

- **List all todos:**
  ```bash
  tudu list todo
  ```

#### `view`
View the details of a specific todo.

- **View a todo by its ID:**
  ```bash
  # tudu view todo <ID>
  tudu view todo 1
  ```

#### `update`
Update an existing todo or project.

- **Update a todo's title:**
  ```bash
  # tudu update todo <ID> --title <NEW_TITLE>
  tudu update todo 1 --title "Finish the awesome README file"
  ```

- **Update a project's name:**
    ```bash
    # tudu update project <ID> --name <NEW_NAME>
    tudu update project 1 --name "My Awesome Project"
    ```

#### `close`
Close (complete) a todo or project.

- **Close a todo by its ID:**
  ```bash
  # tudu close todo <ID>
  tudu close todo 1
  ```

- **Close a project by its ID:**
    ```bash
    # tudu close project <ID>
    tudu close project 1
    ```

---

## 🛠️ Building from Source

If you want to contribute or build `tudu` manually:

```bash
# Clone the repository
git clone https://github.com/your-repo/tudu.git
cd tudu

# Build for development
cargo build

# Build for release (optimized)
cargo build --release
```
The binary will be located in the `target/debug/` or `target/release/` directory.

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1.  Fork the repository.
2.  Create your feature branch (`git checkout -b feature/AmazingFeature`).
3.  Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4.  Push to the branch (`git push origin feature/AmazingFeature`).
5.  Open a Pull Request.

---

## 📄 License

This project is licensed under the MIT License.
