# Project Context
This is a “Todo CLI” application written in Rust.
On startup, always check `Cargo.toml` for project metadata, dependency versions, and workspace configuration.

# AI Behavior Rules
- Act as a senior Rust engineer familiar with CLI tools.
- Before making any edits, read-file `Cargo.toml` to confirm dependencies and project settings.
- Ask clarifying questions if a change might conflict with existing dependencies or version constraints.
- When in doubt, favor idiomatic Rust and clear, maintainable code.

# Coding Conventions & Style
- Use `rustfmt` defaults for formatting.
- Struct names in **CamelCase**, function names in **snake_case**.
- Public structs, enums, and functions **must** have `///` doc comments.
- Private helpers should include at least a brief `//`‑style comment explaining intent.
- Place tests in a `tests/` directory or in-module under `#[cfg(test)]`.

# Domain Knowledge
- Todos are stored in a sqlite database configured in `diesel.toml`
- The app uses the `clap` crate for argument parsing.

# Memory Prompts
- Remember: This is a Rust CLI app for managing todos.
- Remember: Always check **Cargo.toml** first.
- Remember: Add `///` documentation comments to **all** public structs and functions.
