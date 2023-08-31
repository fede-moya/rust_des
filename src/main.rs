use serde::Deserialize;
use std::collections::HashMap;

// Struct definitions

#[derive(Debug, Deserialize)]
pub struct Download {
    pub name: String,
    #[serde(flatten)]
    pub details: DownloadDetails,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DownloadDetails {
    pub url: String,
    pub file_type: String,
    pub file_name: String,
}

#[derive(Debug, Deserialize)]
struct Plugins {
    plugins: PluginsInner,
}

#[derive(Debug, Deserialize)]
struct PluginsInner {
    downloads: HashMap<String, DownloadDetails>,
}

// Main function

fn main() {
    let toml_str = r#"
    config_version = 1

    [plugins.downloads.my_download]
    url = "https://github.com/koalaman/shellcheck/releases/download/v0.9.0/shellcheck-v0.9.0.linux.x86_64.tar.xz"
    file_name = "shellcheck"
    file_type = "tarxz"
    "#;

    let plugins: Plugins = toml::from_str(toml_str).unwrap();

    for (name, details) in plugins.plugins.downloads.iter() {
        let download = Download {
            name: name.clone(),
            details: details.clone(),
        };
        println!("{:?}", download);
    }
}

