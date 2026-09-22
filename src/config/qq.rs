use serde::Deserialize;

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
