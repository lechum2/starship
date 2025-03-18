use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct SalesforceCliConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_files: Vec<&'a str>,
}

impl Default for SalesforceCliConfig<'_> {
    fn default() -> Self {
        SalesforceCliConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: " ",
            style: "bold blue",
            disabled: false,
            detect_files: vec!["sfdx-project.json"],
        }
    }
}
