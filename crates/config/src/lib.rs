mod format;
mod lint;
mod toml_version;
pub use format::FormatOptions;
pub use format::LineEnding;
pub use lint::LintOptions;
pub use toml_version::TomlVersion;

/// # Tombi
///
/// **Tombi** (鳶) is a toolkit for TOML; providing a formatter/linter and language server.
/// See the [GitHub repository](https://github.com/yassun7010/tombi) for more information.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct Config {
    pub toml_version: TomlVersion,
    pub format: FormatOptions,
    pub lint: LintOptions,
}
