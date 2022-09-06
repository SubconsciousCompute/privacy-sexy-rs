use crate::OS;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

/// ### `Collection`
///
/// - A collection simply defines:
///   - different categories and their scripts in a tree structure
///   - OS specific details
/// - Also allows defining common [function](FunctionData)s to be used throughout the collection if
///   you'd like different scripts to share same code.
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

/// ### `Category`
///
/// - Category has a parent that has tree-like structure where it can have subcategories or subscripts.
/// - It's a logical grouping of different scripts and other categories.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryData {
    /// - ❗ Category must consist of at least one subcategory or script.
    /// - Children can be combination of scripts and subcategories.
    pub children: Vec<CategoryOrScriptData>,
    /// - Name of the category
    /// - ❗ Must be unique throughout the [Collection](CollectionData)
    pub category: String,
    pub docs: Option<DocumentationUrlsData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CategoryOrScriptData {
    CategoryData(CategoryData),
    ScriptData(ScriptData),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentationUrlsData {
    VecStrings(Vec<String>),
    String(String),
}

/// ### `FunctionParameter`
///
/// - Defines a parameter that function requires optionally or mandatory.
/// - Its arguments are provided by a [Script](ScriptData) through a [FunctionCall](FunctionCallData).
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterDefinitionData {
    /// - Name of the parameters that the function has.
    /// - Parameter names must be defined to be used in [expressions (templating)](./README.md#expressions).
    /// - ❗ Parameter names must be unique and include alphanumeric characters only.
    pub name: String,
    /// - Specifies whether the caller [Script](ScriptData) must provide any value for the parameter.
    /// - If set to `false` i.e. an argument value is not optional then it expects a non-empty value for the variable;
    ///   - Otherwise it throws.
    /// - 💡 Set it to `true` if a parameter is used conditionally;
    ///   - Or else set it to `false` for verbosity or do not define it as default value is `false` anyway.
    /// - 💡 Can be used in conjunction with [`with` expression](./README.md#with).
    pub optional: Option<bool>,
}

/// ### `Function`
///
/// - Functions allow re-usable code throughout the defined scripts.
/// - Functions are templates compiled by privacy.sexy and uses special expression expressions.
/// - A function can be of two different types (just like [scripts](ScriptData)):
///   1. Inline function: a function with an inline code.
///      - Must define `code` property and optionally `revertCode` but not `call`.
///   2. Caller function: a function that calls other functions.
///      - Must define `call` property but not `code` or `revertCode`.
/// - 👀 Read more on [Templating](./README.md) for function expressions and [example usages](./README.md#parameter-substitution).
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionData {
    /// - Name of the function that scripts will use.
    /// - Convention is to use camelCase, and be verbs.
    /// - E.g. `uninstallStoreApp`
    /// - ❗ Function names must be unique
    pub name: String,
    /// - Batch file commands that will be executed
    /// - 💡 [Expressions (templating)](./README.md#expressions) can be used in its value
    /// - 💡 If defined, best practice to also define `revertCode`
    /// - ❗ If not defined `call` must be defined
    pub code: Option<String>,
    /// - Code that'll undo the change done by `code` property.
    /// - E.g. let's say `code` sets an environment variable as `setx POWERSHELL_TELEMETRY_OPTOUT 1`
    ///   - then `revertCode` should be doing `setx POWERSHELL_TELEMETRY_OPTOUT 0`
    /// - 💡 [Expressions (templating)](./README.md#expressions) can be used in code
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    /// - A shared function or sequence of functions to call (called in order)
    /// - The parameter values that are sent can use [expressions (templating)](./README.md#expressions)
    /// - ❗ If not defined `code` must be defined
    pub call: Option<FunctionCallsData>,
    /// - List of parameters that function code refers to.
    /// - ❗ Must be defined to be able use in [`FunctionCall`](FunctionCallData) or [expressions (templating)](./README.md#expressions)
    /// `code`: *`string`* (**required** if `call` is undefined)
    /// - Batch file commands that will be executed
    /// - 💡 [Expressions (templating)](./README.md#expressions) can be used in its value
    /// - 💡 If defined, best practice to also define `revertCode`
    /// - ❗ If not defined `call` must be defined
    pub parameters: Option<Vec<ParameterDefinitionData>>,
}

/// - Defines key value dictionary for each parameter and its value
/// - E.g.
///
///   ```yaml
///     parameters:
///       userDefinedParameterName: parameterValue
///       # ...
///       appName: Microsoft.WindowsFeedbackHub
///   ```
///
/// - 💡 [Expressions (templating)](./README.md#expressions) can be used as parameter value
pub type FunctionCallParametersData = Value;

/// ### `FunctionCall`
///
/// - Describes a single call to a function by optionally providing values to its parameters.
/// - 👀 See [parameter substitution](./README.md#parameter-substitution) for an example usage
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCallData {
    /// - Name of the function to call.
    /// - ❗ Function with same name must defined in `functions` property of [Collection](CollectionData)
    pub function: String,
    /// - Defines key value dictionary for each parameter and its value
    /// - E.g.
    ///
    ///   ```yaml
    ///     parameters:
    ///       userDefinedParameterName: parameterValue
    ///       # ...
    ///       appName: Microsoft.WindowsFeedbackHub
    ///   ```
    ///
    /// - 💡 [Expressions (templating)](./README.md#expressions) can be used as parameter value
    pub parameters: Option<FunctionCallParametersData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCallsData {
    VecFunctionCallData(Vec<FunctionCallData>),
    FunctionCallData(FunctionCallData),
}

/// ### `Script`
///
/// - Script represents a single tweak.
/// - A script can be of two different types (just like [functions](FunctionData)):
///   1. Inline script; a script with an inline code
///      - Must define `code` property and optionally `revertCode` but not `call`
///   2. Caller script; a script that calls other functions
///      - Must define `call` property but not `code` or `revertCode`
/// - 🙏 For any new script, please add `revertCode` and `docs` values if possible.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptData {
    /// - Name of the script
    /// - ❗ Must be unique throughout the [Collection](CollectionData)
    pub name: String,
    /// - Batch file commands that will be executed
    /// - 💡 If defined, best practice to also define `revertCode`
    /// - ❗ If not defined `call` must be defined, do not define if `call` is defined.
    pub code: Option<String>,
    /// - Code that'll undo the change done by `code` property.
    /// - E.g. let's say `code` sets an environment variable as `setx POWERSHELL_TELEMETRY_OPTOUT 1`
    ///   - then `revertCode` should be doing `setx POWERSHELL_TELEMETRY_OPTOUT 0`
    /// - ❗ Do not define if `call` is defined.
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    /// - A shared function or sequence of functions to call (called in order)
    /// - ❗ If not defined `code` must be defined
    pub call: Option<FunctionCallsData>,
    /// - Single documentation URL or list of URLs for those who wants to learn more about the script
    /// - E.g. `https://docs.microsoft.com/en-us/windows-server/`
    pub docs: Option<DocumentationUrlsData>,
    /// - If not defined then the script will not be recommended
    /// - If defined it can be either
    ///   - `standard`: Only non-breaking scripts without limiting OS functionality
    ///   - `strict`: Scripts that can break certain functionality in favor of privacy and security
    pub recommend: Option<Recommend>,
}

/// ### `ScriptingDefinition`
///
/// - Defines global properties for scripting that's used throughout its parent [Collection](CollectionData).
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptingDefinitionData {
    pub language: String,
    #[serde(rename = "fileExtension")]
    pub file_extension: Option<String>,
    /// - Code that'll be inserted on top of user created script.
    /// - Global variables such as `$homepage`, `$version`, `$date` can be used using
    ///   [parameter substitution](./README.md#parameter-substitution) code syntax such as `Welcome to {{ $homepage }}!`
    #[serde(rename = "startCode")]
    pub start_code: String,
    #[serde(rename = "endCode")]
    /// - Code that'll be inserted at the end of user created script.
    /// - Global variables such as `$homepage`, `$version`, `$date` can be used using
    ///   [parameter substitution](./README.md#parameter-substitution) code syntax such as `Welcome to {{ $homepage }}!`
    pub end_code: String,
}

/// - If not defined then the script will not be recommended
/// - If defined it can be either
///   - `standard`: Only non-breaking scripts without limiting OS functionality
///   - `strict`: Scripts that can break certain functionality in favor of privacy and security
#[derive(Debug, Serialize, Deserialize)]
pub enum Recommend {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "strict")]
    Strict,
}
