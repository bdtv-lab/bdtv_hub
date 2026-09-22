use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// 心跳检查配置文件
pub struct WardenConfig {
    /// 心跳检查周期
    pub interval: u64,
    /// 心跳超时时间
    pub timeout: u64,
}
