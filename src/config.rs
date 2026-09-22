pub mod check;
pub mod node;
pub mod qq;

use std::{fs, path::Path};

use anyhow::{Result, bail};
use serde::Deserialize;

use crate::config::{check::WardenConfig, node::NodeConfig, qq::QqConfig};

const CONFIG_FILE: &str = "config.yaml";

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    /// 服务监听地址
    pub listen: String,

    pub qq: Option<QqConfig>,

    pub node: NodeConfig,

    pub check: WardenConfig,
}

pub fn load_conf() -> Result<AppConfig> {
    if !Path::new(CONFIG_FILE).is_file() {
        bail!("missing {}", CONFIG_FILE,)
    }

    let file = fs::File::open(CONFIG_FILE)?;
    let config = serde_norway::from_reader(file)?;

    Ok(config)
}
