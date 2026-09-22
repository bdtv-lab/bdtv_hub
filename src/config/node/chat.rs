use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// 聊天规则配置
pub struct ChatConfig {
    #[serde(with = "serde_regex")]
    /// 忽略聊天的正则表达式
    pub ignored_regex: Vec<Regex>,
}
