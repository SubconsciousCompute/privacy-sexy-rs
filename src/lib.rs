//! - privacy-sexy is a data-driven application where it reads the necessary OS-specific logic from
//!   yaml files in [`collections`](https://github.com/sn99/privacy-sexy/tree/master/collections)
//! - 💡 Best practices
//!   - If you repeat yourself, try to utilize [YAML-defined functions](FunctionData)
//!   - Always try to add documentation and a way to revert a tweak in [scripts](ScriptData)
//! - 📖 Types in code: [`collections.rs`](https://github.com/sn99/privacy-sexy/blob/master/src/collection.rs)

pub mod collection;

pub use collection::CollectionData;

use serde::{Deserialize, Serialize};

/// Allowed values for OS
#[derive(Debug, Serialize, Deserialize)]
pub enum OS {
    /// Apple
    #[serde(rename = "macos")]
    MacOs,
    /// Microsoft
    #[serde(rename = "windows")]
    Windows,
    /// OpenSource 💕
    #[serde(rename = "linux")]
    Linux,
}

/// Main way to get rules in form of [`CollectionData`](collection::CollectionData)
pub fn get_collection(os: OS) -> Result<CollectionData, Box<dyn std::error::Error>> {
    let mut filename = "collections/".to_string();

    match os {
        OS::MacOs => {
            filename.push_str("macos.yaml");
        }
        OS::Windows => filename.push_str("windows.yaml"),
        OS::Linux => {
            panic!("No rules yet");
        }
    }

    // let file = File::open(filename)?;
    // let deser: CollectionData = serde_yaml::from_reader(file)?;

    // Ok(deser)
    CollectionData::read_file(filename)
}
