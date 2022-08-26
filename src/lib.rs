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
