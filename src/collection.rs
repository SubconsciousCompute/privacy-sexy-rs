use std::{fs::File, io, path::Path};

use regex::{Captures, Regex};
use reqwest::{blocking::get, IntoUrl};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    util::{beautify, parse_start_end, piper},
    OS,
};

/// Error type emitted during parsing
#[derive(Debug)]
pub enum ParseError {
    /// Emitted when a function is not found, with the name of the [`FunctionData`]
    Function(String),
    /// Emitted when a (non-optional) parameter is not provided, with the name of the [`ParameterDefinitionData`]
    Parameter(String),
    /// Emitted when neither call or code are not provided, with the name of the [`ScriptData`]
    CallCode(String),
}

/**
### `Collection`

- A collection simply defines:
  - different categories and their scripts in a tree structure
  - OS specific details
- Also allows defining common [function](FunctionData)s to be used throughout the collection if
  you'd like different scripts to share same code.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionData {
    /// - Operating system that the [Collection](CollectionData) is written for.
    /// - 📖 See [crate](OS) enum for allowed values.
    pub os: OS,
    /// - Defines the scripting language that the code of other action uses.
    pub scripting: ScriptingDefinitionData,
    /// - Each [category](CategoryData) is rendered as different cards in card presentation.
    /// - ❗ A [Collection](CollectionData) must consist of at least one category.
    pub actions: Vec<CategoryData>,
    /// - Functions are optionally defined to re-use the same code throughout different scripts.
    pub functions: Option<Vec<FunctionData>>,
}

/// Emitted when reading [`CollectionData`] from file fails
#[derive(Debug, Error)]
pub enum CollectionError {
    /// Refer to [`io::Error`]
    #[error(transparent)]
    IOError(#[from] io::Error),
    /// Refer to [`serde_yaml::Error`]
    #[error(transparent)]
    SerdeError(#[from] serde_yaml::Error),
    /// Refer to [`reqwest::Error`]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl CollectionData {
    /**
    Reads [`CollectionData`] from file at `path`

    # Errors

    Returns [`CollectionError`] if:
    - file cannot be opened OR
    - contents cannot be deserialized into [`CollectionData`]
    */
    pub fn from_file(path: impl AsRef<Path>) -> Result<CollectionData, CollectionError> {
        Ok(serde_yaml::from_reader::<File, CollectionData>(File::open(path)?)?)
    }

    /**
    Fetches [`CollectionData`] from `url`

    # Errors

    Returns [`CollectionError`] if:
    - `url` cannot be fetched OR
    - contents cannot be deserialized into [`CollectionData`]
    */
    pub fn from_url(url: impl IntoUrl) -> Result<CollectionData, CollectionError> {
        Ok(serde_yaml::from_slice::<CollectionData>(&get(url)?.bytes()?)?)
    }

    /**
    Parses [`CollectionData`] into String

    # Errors

    Returns [`ParseError`] if the object is not parsable
    */
    pub fn parse(
        &self,
        names: Option<&Vec<&str>>,
        revert: bool,
        recommend: Option<Recommend>,
    ) -> Result<String, ParseError> {
        Ok(format!(
            "{}\n\n\n{}\n\n\n{}",
            parse_start_end(&self.scripting.start_code),
            self.actions
                .iter()
                .map(|action| action.parse(names, &self.functions, self.os, revert, recommend))
                .collect::<Result<Vec<String>, ParseError>>()?
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join("\n\n\n"),
            parse_start_end(&self.scripting.end_code),
        ))
    }
}

/**
### `Category`

- Category has a parent that has tree-like structure where it can have subcategories or subscripts.
- It's a logical grouping of different scripts and other categories.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryData {
    /// - ❗ Category must consist of at least one subcategory or script.
    /// - Children can be combination of scripts and subcategories.
    pub children: Vec<CategoryOrScriptData>,
    /// - Name of the category
    /// - ❗ Must be unique throughout the [Collection](CollectionData)
    pub category: String,
    /// - Single documentation URL or list of URLs for those who wants to learn more about the script
    /// - E.g. `https://docs.microsoft.com/en-us/windows-server/`
    pub docs: Option<DocumentationUrlsData>,
}

impl CategoryData {
    /**
    Parses [`CategoryData`] into String

    # Errors

    Returns [`ParseError`] if the object is not parsable
    */
    fn parse(
        &self,
        names: Option<&Vec<&str>>,
        funcs: &Option<Vec<FunctionData>>,
        os: OS,
        revert: bool,
        recommend: Option<Recommend>,
    ) -> Result<String, ParseError> {
        let (names, recommend) = if names.map_or(false, |ns| ns.contains(&self.category.as_str())) {
            (None, None)
        } else {
            (names, recommend)
        };

        Ok(self
            .children
            .iter()
            .map(|child| child.parse(names, funcs, os, revert, recommend))
            .collect::<Result<Vec<String>, ParseError>>()?
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join("\n\n\n"))
    }
}

/// Enum to hold possible values
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CategoryOrScriptData {
    /// Refer to [Collection](CategoryData)
    CategoryData(CategoryData),
    /// Refer to [Collection](ScriptData)
    ScriptData(ScriptData),
}

impl CategoryOrScriptData {
    /**
    Parses [`CategoryOrScriptData`] into String

    # Errors

    Returns [`ParseError`] if the object is not parsable
    */
    fn parse(
        &self,
        names: Option<&Vec<&str>>,
        funcs: &Option<Vec<FunctionData>>,
        os: OS,
        revert: bool,
        recommend: Option<Recommend>,
    ) -> Result<String, ParseError> {
        match self {
            CategoryOrScriptData::CategoryData(data) => data.parse(names, funcs, os, revert, recommend),
            CategoryOrScriptData::ScriptData(data) => data.parse(names, funcs, os, revert, recommend),
        }
    }
}

/// - Single documentation URL or list of URLs for those who wants to learn more about the script
/// - E.g. `https://docs.microsoft.com/en-us/windows-server/`
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentationUrlsData {
    /// Multiple URLs
    VecStrings(Vec<String>),
    /// Single URL
    String(String),
}

/**
### `FunctionParameter`

- Defines a parameter that function requires optionally or mandatory.
- Its arguments are provided by a [Script](ScriptData) through a [FunctionCall](FunctionCallData).
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterDefinitionData {
    /**
    - Name of the parameters that the function has.
    - Parameter names must be defined to be used in
    [expressions (templating)](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#expressions).
    - ❗ Parameter names must be unique and include alphanumeric characters only.
    */
    pub name: String,
    /**
    - Specifies whether the caller [Script](ScriptData) must provide any value for the parameter.
    - If set to `false` i.e. an argument value is not optional then it expects a non-empty value for the variable;
      - Otherwise it throws.
    - 💡 Set it to `true` if a parameter is used conditionally;
      - Or else set it to `false` for verbosity or do not define it as default value is `false` anyway.
    - 💡 Can be used in conjunction with
    [`with` expression](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#with).
    */
    #[serde(default)]
    pub optional: bool,
}

/**
### `Function`

- Functions allow re-usable code throughout the defined scripts.
- Functions are templates compiled by privacy.sexy and uses special expression expressions.
- A function can be of two different types (just like [scripts](ScriptData)):
  1. Inline function: a function with an inline code.
     - Must define `code` property and optionally `revertCode` but not `call`.
  2. Caller function: a function that calls other functions.
     - Must define `call` property but not `code` or `revertCode`.
- 👀 Read more on [Templating](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md) for function expressions
    and [example usages](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#parameter-substitution).
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionData {
    /**
    - Name of the function that scripts will use.
    - Convention is to use camelCase, and be verbs.
    - E.g. `uninstallStoreApp`
    - ❗ Function names must be unique
    */
    pub name: String,
    /**
    - Batch file commands that will be executed
    - 💡 [Expressions (templating)](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#expressions)
        can be used in its value
    - 💡 If defined, best practice to also define `revertCode`
    - ❗ If not defined `call` must be defined
    */
    pub code: Option<String>,
    /**
    - Code that'll undo the change done by `code` property.
    - E.g. let's say `code` sets an environment variable as `setx POWERSHELL_TELEMETRY_OPTOUT 1`
      - then `revertCode` should be doing `setx POWERSHELL_TELEMETRY_OPTOUT 0`
    - 💡 [Expressions (templating)](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#expressions)
        can be used in code
    */
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    /**
    - A shared function or sequence of functions to call (called in order)
    - The parameter values that are sent can use [expressions (templating)](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#expressions)
    - ❗ If not defined `code` must be defined
    */
    pub call: Option<FunctionCallsData>,
    /**
    - List of parameters that function code refers to.
    - ❗ Must be defined to be able use in [`FunctionCall`](FunctionCallData) or
        [expressions (templating)](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#expressions)
    `code`: *`string`* (**required** if `call` is undefined)
    - Batch file commands that will be executed
    - 💡 [Expressions (templating)](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#expressions)
        can be used in its value
    - 💡 If defined, best practice to also define `revertCode`
    - ❗ If not defined `call` must be defined
    */
    pub parameters: Option<Vec<ParameterDefinitionData>>,
}

impl FunctionData {
    /**
    Parses [`FunctionData`] into String

    # Errors

    Returns [`ParseError`] if the object is not parsable
    */
    fn parse(
        &self,
        params: &Option<FunctionCallParametersData>,
        funcs: &Option<Vec<FunctionData>>,
        os: OS,
        revert: bool,
    ) -> Result<String, ParseError> {
        let mut parsed = {
            if let Some(fcd) = &self.call {
                fcd.parse(funcs, os, revert)?
            } else if let Some(code_string) = if revert { &self.revert_code } else { &self.code } {
                code_string.to_string()
            } else {
                return Err(ParseError::CallCode(self.name.clone()));
            }
        };

        if let Some(vec_pdd) = &self.parameters {
            for pdd in vec_pdd {
                parsed = match params.as_ref().and_then(|p| p.get(&pdd.name)) {
                    Some(v) => {
                        if pdd.optional {
                            parsed = Regex::new(&format!(
                                r"(?s)\{{\{{\s*with\s*\${}\s*\}}\}}\s?(.*?)\s?\{{\{{\s*end\s*\}}\}}",
                                &pdd.name
                            ))
                            .unwrap()
                            .replace_all(&parsed, |c: &Captures| {
                                c.get(1)
                                    .map_or("", |m| m.as_str())
                                    .replace("{{ . ", &format!("{{{{ ${} ", &pdd.name))
                            })
                            .to_string();
                        }

                        Regex::new(format!(r"\{{\{{\s*\${}\s*((\|\s*\w*\s*)*)\}}\}}", &pdd.name).as_str())
                            .unwrap()
                            .replace_all(&parsed, |c: &Captures| {
                                c.get(1)
                                    .map_or("", |m| m.as_str())
                                    .split('|')
                                    .map(str::trim)
                                    .filter(|p| !p.is_empty())
                                    .fold(v.as_str().unwrap().to_string(), |v, pipe| piper(pipe.trim(), &v))
                            })
                    }
                    None => {
                        if pdd.optional {
                            Regex::new(&format!(
                                r"(?s)\{{\{{\s*with\s*\${}\s*\}}\}}\s?(.*?)\s?\{{\{{\s*end\s*\}}\}}",
                                &pdd.name
                            ))
                            .unwrap()
                            .replace_all(&parsed, "")
                        } else {
                            return Err(ParseError::Parameter(pdd.name.clone()));
                        }
                    }
                }
                .to_string();
            }
        }

        Ok(parsed)
    }
}

/**
- Defines key value dictionary for each parameter and its value
- E.g.

  ```yaml
    parameters:
      userDefinedParameterName: parameterValue
      # ...
      appName: Microsoft.WindowsFeedbackHub
  ```

- 💡 [Expressions (templating)](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#expressions)
    can be used as parameter value
*/
pub type FunctionCallParametersData = serde_yaml::Value;

/**
### `FunctionCall`

- Describes a single call to a function by optionally providing values to its parameters.
- 👀 See [parameter substitution](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#parameter-substitution)
    for an example usage
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCallData {
    /// - Name of the function to call.
    /// - ❗ Function with same name must defined in `functions` property of [Collection](CollectionData)
    pub function: String,
    /**
    - Defines key value dictionary for each parameter and its value
    - E.g.

      ```yaml
        parameters:
          userDefinedParameterName: parameterValue
          # ...
          appName: Microsoft.WindowsFeedbackHub
      ```

    - 💡 [Expressions (templating)](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#expressions)
        can be used as parameter value
    */
    pub parameters: Option<FunctionCallParametersData>,
}

impl FunctionCallData {
    /**
    Parses [`FunctionCallData`] into String

    # Errors

    Returns [`ParseError`] if the object is not parsable
    */
    fn parse(&self, funcs: &Option<Vec<FunctionData>>, os: OS, revert: bool) -> Result<String, ParseError> {
        funcs
            .as_ref()
            .and_then(|vec_fd| vec_fd.iter().find(|fd| fd.name == self.function))
            .map_or(Err(ParseError::Function(self.function.clone())), |fd| {
                fd.parse(&self.parameters, funcs, os, revert)
            })
    }
}

/// Possible parameters of a function call i.e. either one parameter or multiple parameters
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCallsData {
    /// Multiple Parameter
    VecFunctionCallData(Vec<FunctionCallData>),
    /// Single Parameter
    FunctionCallData(FunctionCallData),
}

impl FunctionCallsData {
    /**
    Parses [`FunctionCallsData`] into String

    # Errors

    Returns [`ParseError`] if the object is not parsable
    */
    fn parse(&self, funcs: &Option<Vec<FunctionData>>, os: OS, revert: bool) -> Result<String, ParseError> {
        match &self {
            FunctionCallsData::VecFunctionCallData(vec_fcd) => Ok(vec_fcd
                .iter()
                .map(|fcd| fcd.parse(funcs, os, revert))
                .collect::<Result<Vec<String>, ParseError>>()?
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
                .join("\n\n")),
            FunctionCallsData::FunctionCallData(fcd) => fcd.parse(funcs, os, revert),
        }
    }
}

/**
### `Script`

- Script represents a single tweak.
- A script can be of two different types (just like [functions](FunctionData)):
  1. Inline script; a script with an inline code
     - Must define `code` property and optionally `revertCode` but not `call`
  2. Caller script; a script that calls other functions
     - Must define `call` property but not `code` or `revertCode`
- 🙏 For any new script, please add `revertCode` and `docs` values if possible.
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptData {
    /// - Name of the script
    /// - ❗ Must be unique throughout the [Collection](CollectionData)
    pub name: String,
    /**
    - Batch file commands that will be executed
    - 💡 If defined, best practice to also define `revertCode`
    - ❗ If not defined `call` must be defined, do not define if `call` is defined.
    */
    pub code: Option<String>,
    /**
    - Code that'll undo the change done by `code` property.
    - E.g. let's say `code` sets an environment variable as `setx POWERSHELL_TELEMETRY_OPTOUT 1`
      - then `revertCode` should be doing `setx POWERSHELL_TELEMETRY_OPTOUT 0`
    - ❗ Do not define if `call` is defined.
    */
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    /// - A shared function or sequence of functions to call (called in order)
    /// - ❗ If not defined `code` must be defined
    pub call: Option<FunctionCallsData>,
    /// - Single documentation URL or list of URLs for those who wants to learn more about the script
    /// - E.g. `https://docs.microsoft.com/en-us/windows-server/`
    pub docs: Option<DocumentationUrlsData>,
    /**
    - If not defined then the script will not be recommended
    - If defined it can be either
      - `standard`: Only non-breaking scripts without limiting OS functionality
      - `strict`: Scripts that can break certain functionality in favor of privacy and security
    */
    pub recommend: Option<Recommend>,
}

impl ScriptData {
    /**
    Parses [`ScriptData`] into String

    # Errors

    Returns [`ParseError`] if the object is not parsable
    */
    fn parse(
        &self,
        names: Option<&Vec<&str>>,
        funcs: &Option<Vec<FunctionData>>,
        os: OS,
        revert: bool,
        recommend: Option<Recommend>,
    ) -> Result<String, ParseError> {
        if (recommend.is_some() && recommend > self.recommend)
            || names.map_or(false, |n| !n.contains(&self.name.as_str()))
        {
            Ok(String::new())
        } else if let Some(fcd) = &self.call {
            Ok(beautify(&fcd.parse(funcs, os, revert)?, &self.name, os, revert))
        } else if let Some(code_string) = if revert { &self.revert_code } else { &self.code } {
            Ok(beautify(code_string, &self.name, os, revert))
        } else {
            Err(ParseError::CallCode(self.name.clone()))
        }
    }
}

/**
### `ScriptingDefinition`

- Defines global properties for scripting that's used throughout its parent [Collection](CollectionData).
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptingDefinitionData {
    /// Name of the Script
    pub language: String,
    /// Optional file extension for the said script
    #[serde(rename = "fileExtension")]
    pub file_extension: Option<String>,
    /**
    - Code that'll be inserted on top of user created script.
    - Global variables such as `$homepage`, `$version`, `$date` can be used using
      [parameter substitution](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#parameter-substitution)
      code syntax such as `Welcome to {{ $homepage }}!`
    */
    #[serde(rename = "startCode")]
    pub start_code: String,
    /**
    - Code that'll be inserted at the end of user created script.
    - Global variables such as `$homepage`, `$version`, `$date` can be used using
      [parameter substitution](https://github.com/SubconsciousCompute/privacy-sexy/blob/master/src/README.md#parameter-substitution)
      code syntax such as `Welcome to {{ $homepage }}!`
    */
    #[serde(rename = "endCode")]
    pub end_code: String,
}

/**
- If not defined then the script will not be recommended
- If defined it can be either
  - `standard`: Only non-breaking scripts without limiting OS functionality
  - `strict`: Scripts that can break certain functionality in favor of privacy and security
*/
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Recommend {
    /// - `strict`: Scripts that can break certain functionality in favor of privacy and security
    #[serde(rename = "strict")]
    Strict,
    /// - `standard`: Only non-breaking scripts without limiting OS functionality
    #[serde(rename = "standard")]
    Standard,
}
