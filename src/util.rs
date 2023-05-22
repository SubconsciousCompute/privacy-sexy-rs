use crate::OS;

use chrono::Local;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::fs;

/**
Wraps the `code_string` in comments and adds an echo call

# Examples

```no_run
assert_eq!(r#"
## ------------------------------------------------------------
## ---------------------Clear bash history---------------------
## ------------------------------------------------------------
echo --- Clear bash history
rm -f ~/.bash_history
## ------------------------------------------------------------
"#,
beautify("rm -f ~/.bash_history", "Clear bash history", &OS::Linux, false)
)
```
*/
pub fn beautify(code_string: &str, name: &str, os: OS, revert: bool) -> String {
    let mut name = name.to_string();
    if revert {
        name.push_str(" (revert)");
    }

    if let OS::Windows = os {
        format!(
            ":: {0:-^60}\n:: {1:-^60}\n:: {0:-^60}\necho --- {1}\n{2}\n:: {0:-^60}",
            "", name, code_string
        )
    } else {
        format!(
            "# {0:-^60}\n# {1:-^60}\n# {0:-^60}\necho --- {1}\n{2}\n# {0:-^60}",
            "", name, code_string
        )
    }
}

/**
Applies pipe on `text`. Following pipes are available:
- escapeDoubleQuotes
- inlinePowerShell

# Panics

Panics for invalid regex expressions

# Examples

```no_run
assert_eq!("\"^\"\"Hello\"^\"\"", piper("escapeDoubleQuotes", "\"Hello\""));
```
*/
pub fn piper(pipe: &str, text: &str) -> String {
    match pipe {
        "escapeDoubleQuotes" => text.replace('\"', "\"^\"\""),
        "inlinePowerShell" => {
            // Inline comments
            let t = Regex::new(r"<#\s*(.*)#>|#\s*(.*)")
                .unwrap()
                .replace_all(text, |c: &Captures| {
                    c.get(1)
                        .map_or(String::new(), |m| format!("<# {} #>", m.as_str().trim()))
                });

            // Here strings
            let t = Regex::new(r#"@(['"])\s*(?:\r\n|\r|\n)((.|\n|\r)+?)(\r\n|\r|\n)['"]@"#)
                .unwrap()
                .replace_all(&t, |c: &Captures| {
                    let (quotes, escaped_quotes, separator) = match c.get(1).map_or("'", |m| m.as_str()) {
                        "'" => ("'", "''", "'+\"`r`n\"+'"),
                        _ => ("\"", "`\"", "`r`n"),
                    };

                    format!(
                        "{0}{1}{0}",
                        quotes,
                        Regex::new(r"\r\n|\r|\n")
                            .unwrap()
                            .split(&c.get(2).map_or("", |m| m.as_str()).replace(quotes, escaped_quotes))
                            .collect::<Vec<&str>>()
                            .join(separator)
                    )
                });

            // Merge lines with back tick
            let t = Regex::new(r" +`\s*(?:\r\n|\r|\n)\s*").unwrap().replace_all(&t, " ");

            // Merge lines
            Regex::new(r"\r\n|\r|\n")
                .unwrap()
                .split(&t)
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .collect::<Vec<&str>>()
                .join("; ")
        }
        _ => text.to_string(),
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct CargoParams {
    #[serde(default)]
    package: PkgParams,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct PkgParams {
    #[serde(default)]
    homepage: String,
    #[serde(default)]
    version: String,
}

/**
Substitutes global variables in `code_string`

Supported global variables:
- $date
- $homepage
- $version

Refer to [parameter substitution](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#parameter-substitution)
for more info & usage examples
*/
pub fn parse_start_end(code_string: &str) -> String {
    let cargo_params =
        toml::from_str::<CargoParams>(&fs::read_to_string("Cargo.toml").unwrap_or_default()).unwrap_or_default();

    code_string
        .to_string()
        .replace("{{ $date }}", &Local::now().to_rfc2822())
        .replace("{{ $homepage }}", &cargo_params.package.homepage)
        .replace("{{ $version }}", &cargo_params.package.version)
}
