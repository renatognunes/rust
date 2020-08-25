## Rust Programming Language Notes

- Before running a Rust program, you must compile it using the Rust compiler by entering the rustc command and passing it the name of your source file.
- `println!()` calls a Rust macro. (Using a ! means that you’re calling a macro instead of a normal function).
- `rustc main.rs` (compile it using the Rust compiler).

### Cargo Package Manager

- `cargo new hello_cargo` (Creating a Project with Cargo).
- Cargo.toml (Cargo's configuration file similar to package.json).

#### Commands:

- `cargo build` (Creates an executable file in target/debug/hello_cargo rather than in your current directory).
- `cargo run` (to compile the code and then run the resulting executable all in one command).
- `cargo check` (Often, cargo check is much faster than cargo build, because it skips the step of producing an executable).

---
