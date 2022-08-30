//! - privacy-sexy is a data-driven application where it reads the necessary OS-specific logic from
//!   yaml files in [`collections`](./../collections/)
//! - 💡 Best practices
//!   - If you repeat yourself, try to utilize [YAML-defined functions](#Function)
//!   - Always try to add documentation and a way to revert a tweak in [scripts](#Script)
//! - 📖 Types in code: [`collections.rs`](./../src/collections.rs)
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

/// ### `Collection`
///
/// - A collection simply defines:
///   - different categories and their scripts in a tree structure
///   - OS specific details
/// - Also allows defining common [function](#Function)s to be used throughout the collection if
///   you'd like different scripts to share same code.
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionData {
    /// - Operating system that the [Collection](#collection) is written for.
    pub os: String,
    /// - Defines the scripting language that the code of other action uses.
    pub scripting: ScriptingDefinitionData,
    /// - Each [category](#category) is rendered as different cards in card presentation.
    /// - ❗ A [Collection](#collection) must consist of at least one category.
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
    /// - ❗ Must be unique throughout the [Collection](#collection)
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
/// - Its arguments are provided by a [Script](#script) through a [FunctionCall](#FunctionCall).
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterDefinitionData {
    /// - Name of the parameters that the function has.
    /// - Parameter names must be defined to be used in [expressions (templating)](./templating.md#expressions).
    /// - ❗ Parameter names must be unique and include alphanumeric characters only.
    pub name: String,
    /// - Specifies whether the caller [Script](#script) must provide any value for the parameter.
    /// - If set to `false` i.e. an argument value is not optional then it expects a non-empty value for the variable;
    ///   - Otherwise it throws.
    /// - 💡 Set it to `true` if a parameter is used conditionally;
    ///   - Or else set it to `false` for verbosity or do not define it as default value is `false` anyway.
    /// - 💡 Can be used in conjunction with [`with` expression](./templating.md#with).
    pub optional: Option<bool>,
}

/// ### `Function`
///
/// - Functions allow re-usable code throughout the defined scripts.
/// - Functions are templates compiled by privacy.sexy and uses special expression expressions.
/// - A function can be of two different types (just like [scripts](#script)):
///   1. Inline function: a function with an inline code.
///      - Must define `code` property and optionally `revertCode` but not `call`.
///   2. Caller function: a function that calls other functions.
///      - Must define `call` property but not `code` or `revertCode`.
/// - 👀 Read more on [Templating](./templating.md) for function expressions and [example usages](./templating.md#parameter-substitution).
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionData {
    /// - Name of the function that scripts will use.
    /// - Convention is to use camelCase, and be verbs.
    /// - E.g. `uninstallStoreApp`
    /// - ❗ Function names must be unique
    pub name: String,
    /// - Batch file commands that will be executed
    /// - 💡 [Expressions (templating)](./templating.md#expressions) can be used in its value
    /// - 💡 If defined, best practice to also define `revertCode`
    /// - ❗ If not defined `call` must be defined
    pub code: Option<String>,
    /// - Code that'll undo the change done by `code` property.
    /// - E.g. let's say `code` sets an environment variable as `setx POWERSHELL_TELEMETRY_OPTOUT 1`
    ///   - then `revertCode` should be doing `setx POWERSHELL_TELEMETRY_OPTOUT 0`
    /// - 💡 [Expressions (templating)](./templating.md#expressions) can be used in code
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    /// - A shared function or sequence of functions to call (called in order)
    /// - The parameter values that are sent can use [expressions (templating)](./templating.md#expressions)
    /// - ❗ If not defined `code` must be defined
    pub call: Option<FunctionCallsData>,
    /// - List of parameters that function code refers to.
    /// - ❗ Must be defined to be able use in [`FunctionCall`](#functioncall) or [expressions (templating)](./templating.md#expressions)
    /// `code`: *`string`* (**required** if `call` is undefined)
    /// - Batch file commands that will be executed
    /// - 💡 [Expressions (templating)](./templating.md#expressions) can be used in its value
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
/// - 💡 [Expressions (templating)](./templating.md#expressions) can be used as parameter value
pub type FunctionCallParametersData = Value;

/// ### `FunctionCall`
///
/// - Describes a single call to a function by optionally providing values to its parameters.
/// - 👀 See [parameter substitution](./templating.md#parameter-substitution) for an example usage
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCallData {
    /// - Name of the function to call.
    /// - ❗ Function with same name must defined in `functions` property of [Collection](#collection)
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
    /// - 💡 [Expressions (templating)](./templating.md#expressions) can be used as parameter value
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
/// - A script can be of two different types (just like [functions](#function)):
///   1. Inline script; a script with an inline code
///      - Must define `code` property and optionally `revertCode` but not `call`
///   2. Caller script; a script that calls other functions
///      - Must define `call` property but not `code` or `revertCode`
/// - 🙏 For any new script, please add `revertCode` and `docs` values if possible.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptData {
    /// - Name of the script
    /// - ❗ Must be unique throughout the [Collection](#collection)
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
/// - Defines global properties for scripting that's used throughout its parent [Collection](#collection).
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptingDefinitionData {
    pub language: String,
    #[serde(rename = "fileExtension")]
    pub file_extension: Option<String>,
    /// - Code that'll be inserted on top of user created script.
    /// - Global variables such as `$homepage`, `$version`, `$date` can be used using
    ///   [parameter substitution](./templating.md#parameter-substitution) code syntax such as `Welcome to {{ $homepage }}!`
    #[serde(rename = "startCode")]
    pub start_code: String,
    #[serde(rename = "endCode")]
    /// - Code that'll be inserted at the end of user created script.
    /// - Global variables such as `$homepage`, `$version`, `$date` can be used using
    ///   [parameter substitution](./templating.md#parameter-substitution) code syntax such as `Welcome to {{ $homepage }}!`
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

/*
Machine generated struct for .yaml configs

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Windows {
    pub os: String,
    pub scripting: Scripting,
    pub actions: Vec<Action>,
    pub functions: Vec<FunctionElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub category: String,
    pub children: Vec<ActionChild>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionChild {
    pub category: Option<String>,
    pub children: Option<Vec<PurpleChild>>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub docs: Option<IndecentDocs>,
    pub recommend: Option<Recommend>,
    pub call: Option<FriskyCall>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleCall {
    pub function: PurpleFunction,
    pub parameters: PurpleParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleParameters {
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    #[serde(rename = "serviceName")]
    pub service_name: Option<String>,
    #[serde(rename = "defaultStartupMode")]
    pub default_startup_mode: Option<DefaultStartupMode>,
    pub message: Option<String>,
    #[serde(rename = "ignoreWindows11")]
    pub ignore_windows11: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyCall {
    pub function: String,
    pub parameters: FluffyParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyParameters {
    pub code: Option<String>,
    #[serde(rename = "serviceName")]
    pub service_name: Option<String>,
    #[serde(rename = "defaultStartupMode")]
    pub default_startup_mode: Option<DefaultStartupMode>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    #[serde(rename = "featureName")]
    pub feature_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurpleChild {
    pub name: Option<String>,
    pub code: Option<String>,
    pub recommend: Option<Recommend>,
    pub category: Option<String>,
    pub children: Option<Vec<FluffyChild>>,
    pub docs: Option<IndigoDocs>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    pub call: Option<MischievousCall>,
    pub recomend: Option<Recommend>,
    pub enabler: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledCall {
    pub function: PurpleFunction,
    pub parameters: TentacledParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledParameters {
    #[serde(rename = "serviceName")]
    pub service_name: Option<String>,
    #[serde(rename = "defaultStartupMode")]
    pub default_startup_mode: Option<DefaultStartupMode>,
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    pub setting: Option<String>,
    #[serde(rename = "powerShellValue")]
    pub power_shell_value: Option<String>,
    #[serde(rename = "featureName")]
    pub feature_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FluffyChild {
    pub category: Option<String>,
    pub docs: Option<StickyDocs>,
    pub children: Option<Vec<TentacledChild>>,
    pub name: Option<String>,
    pub recommend: Option<Recommend>,
    pub code: Option<String>,
    pub call: Option<BraggadociousCall>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyCall {
    pub function: FluffyFunction,
    pub parameters: StickyParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyParameters {
    pub property: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "default")]
    pub parameters_default: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    #[serde(rename = "setDefaultOnWindows11")]
    pub set_default_on_windows11: Option<bool>,
    #[serde(rename = "packageName")]
    pub package_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoCall {
    pub function: TentacledFunction,
    pub parameters: IndigoParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoParameters {
    #[serde(rename = "serviceName")]
    pub service_name: Option<String>,
    #[serde(rename = "defaultStartupMode")]
    pub default_startup_mode: Option<DefaultStartupMode>,
    #[serde(rename = "processName")]
    pub process_name: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    #[serde(rename = "packageName")]
    pub package_name: Option<String>,
    #[serde(rename = "featureName")]
    pub feature_name: Option<String>,
    #[serde(rename = "capabilityName")]
    pub capability_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TentacledChild {
    pub name: Option<String>,
    pub recommend: Option<Recommend>,
    pub code: Option<String>,
    pub docs: Option<TentacledDocs>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    pub call: Option<Call1>,
    pub category: Option<String>,
    pub children: Option<Vec<StickyChild>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndecentCall {
    pub function: FluffyFunction,
    pub parameters: IndecentParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndecentParameters {
    pub property: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "default")]
    pub parameters_default: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    #[serde(rename = "setDefaultOnWindows11")]
    pub set_default_on_windows11: Option<bool>,
    #[serde(rename = "serviceName")]
    pub service_name: Option<String>,
    #[serde(rename = "defaultStartupMode")]
    pub default_startup_mode: Option<DefaultStartupMode>,
    #[serde(rename = "filePath")]
    pub file_path: Option<String>,
    #[serde(rename = "packageName")]
    pub package_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HilariousCall {
    pub function: TentacledFunction,
    pub parameters: HilariousParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HilariousParameters {
    #[serde(rename = "processName")]
    pub process_name: Option<String>,
    pub property: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "default")]
    pub parameters_default: Option<String>,
    #[serde(rename = "packageName")]
    pub package_name: Option<String>,
    #[serde(rename = "featureName")]
    pub feature_name: Option<String>,
    #[serde(rename = "capabilityName")]
    pub capability_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyChild {
    pub name: Option<String>,
    pub docs: Option<FluffyDocs>,
    pub call: Option<Call2>,
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    pub recommend: Option<Recommend>,
    pub category: Option<String>,
    pub children: Option<Vec<IndigoChild>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmbitiousCall {
    pub function: TentacledFunction,
    pub parameters: AmbitiousParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmbitiousParameters {
    pub property: String,
    pub value: String,
    #[serde(rename = "default")]
    pub parameters_default: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndigoChild {
    pub name: Option<String>,
    pub docs: Option<PurpleDocs>,
    pub call: Option<Vec<ChildCallClass>>,
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    pub category: Option<String>,
    pub children: Option<Vec<IndecentChild>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildCallClass {
    pub function: FluffyFunction,
    pub parameters: CunningParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CunningParameters {
    pub property: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "default")]
    pub parameters_default: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndecentChild {
    pub name: String,
    pub docs: String,
    pub code: String,
    #[serde(rename = "revertCode")]
    pub revert_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionElement {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub code: Option<String>,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
    pub call: Option<FunctionCall>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CunningCall {
    pub function: FluffyFunction,
    pub parameters: MagentaParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MagentaParameters {
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "defaultStartupMode")]
    pub default_startup_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MagentaCall {
    pub function: TentacledFunction,
    pub parameters: FriskyParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriskyParameters {
    pub code: String,
    #[serde(rename = "revertCode")]
    pub revert_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub optional: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scripting {
    pub language: String,
    #[serde(rename = "startCode")]
    pub start_code: String,
    #[serde(rename = "endCode")]
    pub end_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyCall {
    FluffyCall(FluffyCall),
    PurpleCallArray(Vec<PurpleCall>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousCall {
    FluffyCallArray(Vec<FluffyCall>),
    TentacledCall(TentacledCall),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BraggadociousCall {
    IndigoCall(IndigoCall),
    StickyCallArray(Vec<StickyCall>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Call1 {
    HilariousCall(HilariousCall),
    IndecentCallArray(Vec<IndecentCall>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Call2 {
    AmbitiousCall(AmbitiousCall),
    IndecentCallArray(Vec<IndecentCall>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleDocs {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyDocs {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledDocs {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyDocs {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoDocs {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentDocs {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCall {
    CunningCallArray(Vec<CunningCall>),
    MagentaCall(MagentaCall),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PurpleFunction {
    DisableFeature,
    DisablePerUserService,
    DisableService,
    RunInlineCode,
    SetVsCodeSetting,
    ShowWarning,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DefaultStartupMode {
    Automatic,
    Manual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FluffyFunction {
    DisableServiceInRegistry,
    RenameSystemFile,
    RunInlineCode,
    RunInlineCodeAsTrustedInstaller,
    SetMpPreference,
    UninstallStoreApp,
    UninstallSystemApp,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TentacledFunction {
    DisableFeature,
    DisableService,
    KillProcessWhenItStarts,
    RunPowerShell,
    SetMpPreference,
    UninstallCapability,
    UninstallStoreApp,
    UninstallSystemApp,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Recommend {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "strict")]
    Strict,
}
*/
