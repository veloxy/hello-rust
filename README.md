# Running the code

To run this code, you will need to have the following dependencies installed:

- `sea_orm`: A Rust library for interacting with databases
- `axum`: A Rust library for building web servers
- `cargo-watch`: A command-line utility for automatically running a command when a file changes

You can install these dependencies using [Cargo](https://doc.rust-lang.org/cargo/), the Rust package manager. 

```bash
cargo install --path .
```

To install `cargo-watch`, use the following command:

```bash
cargo install cargo-watch
```

Once you have the dependencies installed, you can run the code using `cargo watch` to automatically rebuild the code whenever a file changes:

```bash
cargo watch -x run --package hello-rust
```

Whenever a file changes, `cargo watch` will automatically run `cargo run` again.

The code uses an SQLite database. The database file will be stored in the `./var/sqlite.db` file. You must first create the database as follows:

```bash
sqlite3 ./var/sqlite.db ".databases"
```