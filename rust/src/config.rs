use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

use crate::test::FileFlags;
use crate::test::FileSystemFeature;
use serde::{Deserialize, Serialize};

mod auth;
pub use auth::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommonFeatureConfig {}

/// Configuration for file-system specific features.
/// Please see the book for more details.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FeaturesConfig {
    #[serde(default)]
    pub file_flags: HashSet<FileFlags>,
    #[serde(default)]
    pub secondary_fs: Option<PathBuf>,
    #[serde(flatten)]
    pub fs_features: HashMap<FileSystemFeature, CommonFeatureConfig>,
}

/// Adjustable file-system specific settings.
/// Please see the book for more details.
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsConfig {
    #[serde(default = "default_naptime")]
    pub naptime: f64,
    pub allow_remount: bool,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        SettingsConfig {
            naptime: default_naptime(),
            allow_remount: false,
        }
    }
}

const fn default_naptime() -> f64 {
    1.0
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// File-system features.
    pub features: FeaturesConfig,
    pub settings: SettingsConfig,
    pub dummy_auth: DummyAuthConfig,
}
