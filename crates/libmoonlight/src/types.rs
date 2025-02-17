use crate::get_moonlight_dir;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoonlightBranch {
    Stable,
    Nightly,
}

impl Display for MoonlightBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stable => write!(f, "stable"),
            Self::Nightly => write!(f, "nightly"),
        }
    }
}

impl MoonlightBranch {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Stable => "Stable",
            Self::Nightly => "Nightly",
        }
    }

    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Stable => {
                "Periodic updates and fixes when they're ready. Suggested for most users."
            }
            Self::Nightly => {
                "In-progress development snapshots while it's being worked on. May contain issues."
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Branch {
    Stable,
    PTB,
    Canary,
    Development,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stable => write!(f, "Stable"),
            Self::PTB => write!(f, "PTB"),
            Self::Canary => write!(f, "Canary"),
            Self::Development => write!(f, "Development"),
        }
    }
}

impl Branch {
    #[must_use]
    pub fn config(&self) -> PathBuf {
        get_moonlight_dir().join(format!("{}.json", self.to_string().to_lowercase()))
    }

    pub fn kill_discord(&self) {
        let name = match self {
            Self::Stable => "Discord",
            Self::PTB => "DiscordPTB",
            Self::Canary => "DiscordCanary",
            Self::Development => "DiscordDevelopment",
        };

        match std::env::consts::OS {
            "windows" => {
                std::process::Command::new("taskkill")
                    .args(["/F", "/IM", &format!("{name}.exe")])
                    .output()
                    .ok();
            }

            "macos" | "linux" => {
                std::process::Command::new("killall")
                    .args([name])
                    .output()
                    .ok();
            }

            _ => unimplemented!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectedInstall {
    pub branch: Branch,
    pub path: PathBuf,
}

// Just DetectedInstall but tracking patched for the UI
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstallInfo {
    pub install: DetectedInstall,
    pub patched: bool,
    pub has_config: bool,
}

// Lot more in here but idc
#[derive(Deserialize, Debug)]
pub struct GitHubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubRelease {
    pub name: String,
    pub assets: Vec<GitHubReleaseAsset>,
}
