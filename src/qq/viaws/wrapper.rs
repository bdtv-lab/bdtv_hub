use std::sync::Arc;

use anyhow::{Result, bail};
use onebot_v11::{
    MessageSegment,
    api::{
        payload::{ApiPayload, GetLoginInfo, SendGroupMsg, SetGroupCard},
        resp::{self, ApiRespData::GetLoginInfoResponse},
    },
    message::segment::TextData,
};

use crate::qq::viaws::WsReq;

impl WsReq {
    /// 向特定群聊发送一条文本消息
    pub(super) async fn send_group_msg(&self, group_id: i64, message: String) -> Result<()> {
        let conn = Arc::clone(&self.conn);
        let _ = conn
            .call_api(ApiPayload::SendGroupMsg(SendGroupMsg {
                group_id,
                message: vec![MessageSegment::Text {
                    data: TextData { text: message },
                }],
                auto_escape: false,
            }))
            .await?;

        Ok(())
    }

    /// 获取登录信息
    pub(super) async fn get_login_info(&self) -> Result<resp::GetLoginInfoResponse> {
        let conn = Arc::clone(&self.conn);
        let resp = conn
            .call_api(ApiPayload::GetLoginInfo(GetLoginInfo {}))
            .await?;

        let GetLoginInfoResponse(data) = resp.data else {
            bail!("expected GetLoginInfoResponse, got {:?}", resp.data);
        };

        Ok(data)
    }

    /// 设置特定用户的群名片
    pub(super) async fn set_group_card(
        &self,
        group_id: i64,
        user_id: i64,
        card: String,
    ) -> Result<()> {
        let conn = Arc::clone(&self.conn);
        let _ = conn
            .call_api(ApiPayload::SetGroupCard(SetGroupCard {
                group_id,
                user_id,
                card,
            }))
            .await?;

        Ok(())
    }
}
