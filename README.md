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

> Note: This is a rust port of [privacy.sexy](https://github.com/undergroundwires/privacy.sexy)

## Usage

See [`examples`](examples)

## Cli

```sh
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