use std::{collections::HashMap, fs};

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub fn default_as_false() -> bool { false }

/// This is a struct will be deserialized from the given filename.
///
/// version indicates the version of the kernel.
/// config indicates the configuration of the kernel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KernelConfig {
    pub version: Option<String>,
    #[serde(default)]
    global: KernelGlobalConfig,
    /// Config list for kernel. This field will be converted to rust cfg.
    #[serde(default)]
    pub bin: HashMap<String, BinaryConfig>,
}

/// Global configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KernelGlobalConfig {
    #[serde(default)]
    configs: HashMap<String, String>,
    #[serde(default)]
    env: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryConfig {
    pub target: String,
    #[serde(skip)]
    global_config: KernelGlobalConfig,
    #[serde(default="default_as_false")]
    pub build_std: bool,
    #[serde(default)]
    configs: HashMap<String, String>,
    #[serde(default)]
    env: HashMap<String, String>,
}

impl BinaryConfig {
    pub fn get_configs(&self) -> HashMap<String, String> {
        let mut configs = self.global_config.configs.clone();
        configs.extend(self.configs.clone());
        configs
    }

    pub fn get_envs(&self) -> HashMap<String, String> {
        let mut envs = self.global_config.env.clone();
        envs.extend(self.env.clone());
        envs
    }
}

impl KernelConfig {
    pub fn get_bin_config(&self, bin: &str) -> Result<BinaryConfig> {
        self.bin
            .get(bin)
            .ok_or(anyhow!("can't find binary_config"))
            .map(|x| {
                let mut config = x.clone();
                config.global_config = self.global.clone();
                config
            })
    }
}

pub fn read_bin_config(path: &str, bin: &str) -> Result<BinaryConfig> {
    let os_config = read_toml(path)?;
    os_config.get_bin_config(bin)
}

pub fn read_toml(path: &str) -> Result<KernelConfig> {
    let fcontent = fs::read_to_string(path)?;
    let kernel_config: KernelConfig = toml::from_str(&fcontent)?;
    Ok(kernel_config)
}
