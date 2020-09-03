# confy

Chat with us: [Discord](https://discord.gg/dwq4Zme)

Zero-boilerplate configuration management.

Focus on storing the right data, instead of worrying about how or where to store it.

```rust
use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    api_key: String,
}

fn main() -> Result<(), ::std::io::Error> {
    let cfg: MyConfig = confy::load("my-app-name")?;
    dbg!(cfg);
    Ok(())
}
```

## Breakings changes
Starting with version 0.4.0 the configuration file are stored in the expected place for your system. See the [`directories`] crates for more information.
Before version 0.4.0, the configuration file was written in the current directory.

[`directories`]: https://crates.io/crates/directories
