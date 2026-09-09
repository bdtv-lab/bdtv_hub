use anyhow::Result;
use tracing::debug;

use crate::{qq::viaws::WsReq, types::Player};

impl WsReq {
    /// 向 qq 群发送玩家加入的信息
    pub(super) async fn send_player_join(&self, player: Player) -> Result<()> {
        debug!(
            "Sending message to QQ: Player {} joined server",
            player.nickname
        );
        self.send_group_msg(self.group_id, format!("{} 加入了服务器", player.nickname))
            .await?;

        Ok(())
    }

    /// 向 qq 群发送玩家退出的信息
    pub(super) async fn send_player_left(&self, player: Player) -> Result<()> {
        debug!(
            "Sending message to QQ: Player {} left server",
            player.nickname
        );
        self.send_group_msg(self.group_id, format!("{} 离开了服务器", player.nickname))
            .await?;

        Ok(())
    }

    /// 更新 qq 群的群名片为在线玩家数
    pub(super) async fn send_player_count_change(&self, count: usize) -> Result<()> {
        let login_info = self.get_login_info().await?;

        let card = if count == 0 {
            format!("{}", login_info.nickname)
        } else {
            format!("{} 人在线", count)
        };

        self.set_group_card(self.group_id, login_info.user_id, card)
            .await?;

        Ok(())
    }
}
