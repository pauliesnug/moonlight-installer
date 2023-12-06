use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum MoonlightBranch {
    Stable,
    Nightly,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum InstallType {
    Windows,
    MacOS,
    Linux,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum Branch {
    Stable,
    PTB,
    Canary,
    Development,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DetectedInstall {
    pub install_type: InstallType,
    pub branch: Branch,
    pub path: PathBuf,
}

#[derive(serde::Deserialize, Debug)]
pub struct GitHubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GitHubRelease {
    pub name: String,
    pub assets: Vec<GitHubReleaseAsset>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Error {
    pub message: String,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}
