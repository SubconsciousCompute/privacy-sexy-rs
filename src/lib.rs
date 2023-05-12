//! - privacy-sexy is a data-driven application where it reads the necessary OS-specific logic from
//!   yaml files in [`collections`](https://github.com/sn99/privacy-sexy/tree/master/collections)
//! - 💡 Best practices
//!   - If you repeat yourself, try to utilize [YAML-defined functions](FunctionData)
//!   - Always try to add documentation and a way to revert a tweak in [scripts](ScriptData)
//! - 📖 Types in code: [`collections.rs`](https://github.com/sn99/privacy-sexy/blob/master/src/collection.rs)

pub mod collection;

pub use collection::CollectionData;

use serde::{Deserialize, Serialize};
use std::{
    env::temp_dir,
    fs::{self, set_permissions, Permissions},
    os::unix::prelude::PermissionsExt,
    process::{Command, ExitStatus},
};

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
///
/// # Errors
///
/// Return [`Err`] according to [`CollectionData`]
///
/// # Panics
///
/// Panics for [`OS::Linux`]
pub fn get_collection(os: &OS) -> Result<CollectionData, Box<dyn std::error::Error>> {
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

    CollectionData::from_file(filename)
}

/// Runs the script
///
/// # Errors
///
/// Returns an [`Err`] if it is unable to do one of the following:
/// - write to the temp script file
/// - change it's permissions (for unix)
/// - execute the script
pub fn run_script(
    script_string: &str,
    file_extension: Option<String>,
    os: &OS,
) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    let mut tmp_file = temp_dir();
    tmp_file.push("privacy-sexy");
    if let Some(ext) = file_extension {
        tmp_file.push(".");
        tmp_file.push(ext);
    }

    fs::write(&tmp_file, script_string)?;
    match os {
        OS::Windows => (),
        _ => set_permissions(&tmp_file, Permissions::from_mode(0o755))?,
    }

    Ok(match os {
        OS::MacOs => Command::new("open")
            .args(["-a", "Terminal.app", tmp_file.to_str().unwrap_or_default()])
            .spawn(),
        OS::Windows => Command::new(&tmp_file).arg("").spawn(),
        OS::Linux => Command::new("").arg("").spawn(), // TODO
    }?
    .wait()?)
}
