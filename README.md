# privacy-sexy

[![docs.rs](https://img.shields.io/docsrs/privacy-sexy?style=flat-square)](https://docs.rs/privacy-sexy/latest/privacy_sexy/)
[![Crates.io](https://img.shields.io/crates/v/privacy-sexy?style=flat-square)](https://crates.io/crates/privacy-sexy)

Open-source tool to enforce privacy & security best-practices on Windows and MacOs, because privacy is sexy 🍑🍆

- privacy-sexy is a data-driven application where it reads the necessary OS-specific logic from
  yaml files in [`collections`](collections)
- 💡 Best practices
    - If you repeat yourself, try to utilize [YAML-defined functions](FunctionData)
    - Always try to add documentation and a way to revert a tweak in [scripts](ScriptData)
- 📖 Types in code: [`collections.rs`](src/collection.rs)

## Usage

- Preferred way
```rust
use privacy_sexy::OS::Windows;

fn main() {
    println!("{:#?}", privacy_sexy::get_collection(Windows))
}
```

- Option 1
```rust
use privacy_sexy::CollectionData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "collections/windows.yaml";
    
    let deser = CollectionData::from_file(filename)?;
    println!("{:#?}", deser);
    Ok(())
}
```

- Option 2
```rust
use std::fs::File;
use std::io::Read;

use privacy_sexy::CollectionData;

fn main() -> Result<(), serde_yaml::Error> {
    let filename = "collections/windows.yaml";

    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let deser: CollectionData = serde_yaml::from_str(&content)?;
            println!("{:#?}", deser);
            Ok(())
        }
        Err(error) => {
            println!("There is an error {}: {}", filename, error);
            Ok(())
        }
    }
}
```

## Cli

```rust
Commands

Usage: privacy-sexy [OPTIONS] <COMMAND>

Commands:
  echo  Generate & print the script
  run   Generate & run the script
  help  Print this message or the help of the given subcommand(s)

Options:
  -t, --strict       Recommend strict
  -d, --standard     Recommend standard
  -n, --name <NAME>  Name of script(s) required
  -r, --revert       Revert script(s)
  -h, --help         Print help
  -V, --version      Print version
```

Refer to [`docs`](https://github.com/undergroundwires/privacy.sexy/tree/master/docs) for external documentation