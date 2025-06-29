# toml_reader

A simple Rust library to load and access TOML configuration files with lazy static initialization.

## Features

- Deserialize TOML config into Rust structs using `serde`
- Global, thread-safe, lazy initialization with `once_cell`
- Easy API: initialize once, then get config anywhere

## Usage

Add this to your `Cargo.toml`:

```toml
toml_reader = { path = "../toml_reader" }
```

```rust
use toml_reader::{init, get};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init("config.toml")?;
    let config = get();
    println!("DB host: {}", config.database.host);
    Ok(())
}
```
