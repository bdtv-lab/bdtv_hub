pub mod chat;

use serde::Deserialize;

use crate::config::node::chat::ChatConfig;

#[derive(Deserialize, Debug, Clone)]
/// MC 服务器端配置
pub struct NodeConfig {
    /// 聊天规则配置
    pub chat: ChatConfig,
}
