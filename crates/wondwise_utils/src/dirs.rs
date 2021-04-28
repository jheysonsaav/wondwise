// Copyright 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.
use crate::logs::{Log, LogLevel};
use directories::ProjectDirs;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct WondwiseDirs {
    config_dir: PathBuf,
    cache_dir: PathBuf,
    data_dir: PathBuf,
}

impl WondwiseDirs {
    pub fn load() -> Self {
        if let Some(dirs) = ProjectDirs::from("sh", "wondwise", "wondwise") {
            Self {
                config_dir: dirs.config_dir().to_owned(),
                cache_dir: dirs.cache_dir().to_owned(),
                data_dir: dirs.data_dir().to_owned(),
            }
        } else {
            Self {
                config_dir: PathBuf::new(),
                cache_dir: PathBuf::new(),
                data_dir: PathBuf::new(),
            }
        }
    }

    pub fn verify(&self) {
        if !self.config_dir.exists() {
            match create_dir_all(self.config_dir.to_owned()) {
                Ok(_) => {
                    let _ = Log::new(LogLevel::Debug, 0, "Created directory").show();
                }
                Err(_) => {
                    Log::new(LogLevel::Error, 1, "Error for create directory").show();
                }
            }
        } else {
            Log::new(LogLevel::Debug, 0, "The directory exist").show();
        }

        if !self.cache_dir.exists() {
            match create_dir_all(self.cache_dir.to_owned()) {
                Ok(_) => {
                    Log::new(LogLevel::Debug, 0, "Created directory").show();
                }
                Err(_) => {
                    Log::new(LogLevel::Error, 1, "Error for create directory").show();
                }
            }
        } else {
            Log::new(LogLevel::Debug, 0, "The directory exist").show();
        }

        if !self.data_dir.exists() {
            match create_dir_all(self.data_dir.to_owned()) {
                Ok(_) => {
                    Log::new(LogLevel::Debug, 0, "Created data directory").show();
                }
                Err(_) => {
                    Log::new(LogLevel::Error, 1, "Error for create data directory").show();
                }
            }
        } else {
            Log::new(LogLevel::Debug, 0, "The directory exist").show();
        }
    }

    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }

    pub fn cache_dir(&self) -> &Path {
        self.cache_dir.as_path()
    }

    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_configdir_load() {
        let dirs = WondwiseDirs::load();

        // ConfigDir
        #[cfg(target_os = "linux")]
        assert_eq!(
            dirs.config_dir.to_str().unwrap(),
            format!("{}/.config/wondwise", env::var("HOME").unwrap()).as_str()
        );

        #[cfg(target_os = "windows")]
        assert_eq!(
            dirs.config_dir.to_str().unwrap(),
            format!(
                "{}\\wondwise\\wondwise\\config",
                env::var("APPDATA").unwrap()
            )
            .as_str()
        );

        #[cfg(target_os = "macos")]
        assert_eq!(
            dirs.config_dir.to_str().unwrap(),
            format!(
                "{}/Library/Application Support/sh.wondwise.wondwise",
                env::var("HOME").unwrap()
            )
            .as_str()
        );
    }

    #[test]
    fn test_datadir_load() {
        let dirs = WondwiseDirs::load();

        // DataDir
        #[cfg(target_os = "linux")]
        assert_eq!(
            dirs.data_dir.to_str().unwrap(),
            format!("{}/.local/share/wondwise", env::var("HOME").unwrap()).as_str()
        );

        #[cfg(target_os = "windows")]
        assert_eq!(
            dirs.data_dir.to_str().unwrap(),
            format!(
                "{}\\AppData\\Roaming\\wondwise\\wondwise\\data",
                env::var("USERPROFILE").unwrap()
            )
            .as_str()
        );

        #[cfg(target_os = "macos")]
        assert_eq!(
            dirs.data_dir.to_str().unwrap(),
            format!(
                "{}/Library/Application Support/sh.wondwise.wondwise",
                env::var("HOME").unwrap()
            )
            .as_str()
        );
    }

    #[test]
    fn test_cachedir_load() {
        let dirs = WondwiseDirs::load();

        // CacheDir
        #[cfg(target_os = "linux")]
        assert_eq!(
            dirs.cache_dir.to_str().unwrap(),
            format!("{}/.cache/wondwise", env::var("HOME").unwrap()).as_str()
        );

        #[cfg(target_os = "windows")]
        assert_eq!(
            dirs.cache_dir.to_str().unwrap(),
            format!(
                "{}\\AppData\\Local\\wondwise\\wondwise\\cache",
                env::var("USERPROFILE").unwrap()
            )
            .as_str()
        );

        #[cfg(target_os = "macos")]
        assert_eq!(
            dirs.cache_dir.to_str().unwrap(),
            format!(
                "{}/Library/Caches/sh.wondwise.wondwise",
                env::var("HOME").unwrap()
            )
            .as_str()
        );
    }
}
