# privacy-sexy

Open-source tool to enforce privacy & security best-practices on Windows and MacOs, because privacy is sexy 🍑🍆

- privacy-sexy is a data-driven application where it reads the necessary OS-specific logic from
  yaml files in [`collections`](https://github.com/sn99/privacy-sexy/tree/master/collections)
- 💡 Best practices
    - If you repeat yourself, try to utilize [YAML-defined functions](FunctionData)
    - Always try to add documentation and a way to revert a tweak in [scripts](ScriptData)
- 📖 Types in code: [`collections.rs`](https://github.com/sn99/privacy-sexy/blob/master/src/collection.rs)

Usage:

- Preferred way
```rust
use privacy_sexy::OS::Windows;

fn main() {
    println!("{:#?}", privacy_sexy::get_collection(Windows))
}
```

- Option 1
```rust
use std::fs::File;

use privacy_sexy::CollectionData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "collections/windows.yaml";

    let file = File::open(filename)?;
    let deser: CollectionData = serde_yaml::from_reader(file)?;

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

Refer to [`docs`](https://github.com/undergroundwires/privacy.sexy/tree/master/docs) for external documentation