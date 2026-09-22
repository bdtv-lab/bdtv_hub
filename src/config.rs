use std::fs;

use anyhow::Result;
use serde::Deserialize;

const CONFIG_FILE: &str = "config.yaml";

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    /// 服务监听地址
    pub listen: String,

    pub qq: Option<QqConfig>,

    pub check: WardenConfig,
}

#[derive(Deserialize, Debug, Clone)]
/// QQ Onebot v11 WebSocket 客户端配置文件
pub struct QqConfig {
    /// WebSocket 主机
    pub host: String,
    /// WebSocket 服务端口
    pub port: u16,
    /// WebSocket token
    pub token: Option<String>,
    /// 绑定的 QQ 群群号
    pub group_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
/// 心跳检查配置文件
pub struct WardenConfig {
    /// 心跳检查周期
    pub interval: u64,
    /// 心跳超时时间
    pub timeout: u64,
}

pub fn load_conf() -> Result<AppConfig> {
    let file = fs::File::open(CONFIG_FILE)?;
    let config = serde_norway::from_reader(file)?;

    Ok(config)
}
