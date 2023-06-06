/*!
- privacy-sexy is a data-driven application where it reads the necessary OS-specific logic from
  yaml files in [`collections`](https://github.com/SubconsciousCompute/privacy-sexy/tree/master/collections)
- 💡 Best practices
  - If you repeat yourself, try to utilize [YAML-defined functions](collection::FunctionData)
  - Always try to add documentation and a way to revert a tweak in [scripts](collection::ScriptData)
- 📖 Types in code: [`collections.rs`](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/collection.rs)
*/
pub mod collection;
mod util;

use std::{
    env, fmt, fs,
    process::{Command, ExitStatus},
};

use collection::{CollectionData, CollectionReadError};
use serde::{Deserialize, Serialize};

/// Allowed values for OS
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

impl OS {
    /**
    Returns [`OS`] respective to current system

    # Panics

    Panics if current operating system is not supported
    */
    pub fn get_system_os() -> Self {
        match std::env::consts::OS {
            "macos" => OS::MacOs,
            "linux" => OS::Linux,
            "windows" => OS::Windows,
            _ => panic!("Unsupported OS!"),
        }
    }
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OS::MacOs => write!(f, "macos"),
            OS::Linux => write!(f, "linux"),
            OS::Windows => write!(f, "windows"),
        }
    }
}

/**
Main way to get rules in form of [`CollectionData`]

# Errors

Refer to [`CollectionReadError`]
*/
pub fn get_collection(os: OS) -> Result<CollectionData, CollectionReadError> {
    CollectionData::from_file(format!("collections/{os}.yaml"))
}

/**
Runs the script

# Errors

Returns [`Err`] if it is unable to:
- write to the temp script file OR
- change it's permissions (for unix) OR
- execute the script
*/
pub fn run_script(
    script_string: &str,
    file_extension: Option<String>,
) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    let mut tmp_file = env::temp_dir();
    tmp_file.push("privacy-sexy");
    if let Some(ext) = file_extension {
        tmp_file.set_extension(ext);
    }

    fs::write(&tmp_file, script_string)?;

    #[cfg(target_family = "unix")]
    {
        use std::os::unix::prelude::PermissionsExt;
        fs::set_permissions(&tmp_file, fs::Permissions::from_mode(0o755))?;
    }

    Ok(Command::new(tmp_file.to_str().unwrap_or_default()).spawn()?.wait()?)
}
