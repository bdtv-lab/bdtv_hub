pub mod event;
mod state;

use std::sync::Arc;

use smaragdine::Printer;
use tokio::{sync::mpsc, task::JoinSet};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{
    config::AppConfig, console::console, qq::qq_connector, server::http_server,
    signal::shutdown_signal, warden::warden,
};
pub use event::{EventToClient, EventToClientWrapper, EventToQQ};
pub use state::State;

/// 应用程序的主结构体
pub struct App {
    state: Arc<State>,
    token: CancellationToken,
    rx: mpsc::Receiver<EventToQQ>,
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        // 初始化 QQ 通信通道
        let (tx, rx) = mpsc::channel(100);

        // 初始化应用状态
        let state = Arc::new(State::new(tx.clone()));

        // 创建取消 token
        let token = CancellationToken::new();

        Self {
            state,
            token,
            rx,
            config,
        }
    }

    pub async fn run(self) {
        // 创建任务集合
        let mut tasks = JoinSet::new();

        // 分离所有权
        let App {
            state,
            token,
            rx,
            config,
        } = self;

        // 启动控制台
        tasks.spawn(console(Arc::clone(&state), token.clone()));
        // 如果配置了 qq，启动 qq 消息发送
        match config.qq.clone() {
            Some(qq_config) => {
                tasks.spawn(qq_connector(
                    qq_config,
                    Arc::clone(&state),
                    rx,
                    token.clone(),
                ));
            }
            None => drop(rx),
        }
        // 启动 http 服务器
        tasks.spawn(http_server(
            config.clone(),
            Arc::clone(&state),
            token.clone(),
        ));
        // 启动在线状态巡检
        tasks.spawn(warden(config.clone(), Arc::clone(&state), token.clone()));
        // 启动关闭信号监听
        tasks.spawn(shutdown_signal(token.clone()));

        // 阻塞等待任务事件
        while let Some(res) = tasks.join_next().await {
            if let Err(e) = res {
                error!("can not join task: {e}");
            }
        }

        info!("bye");
    }

    /// 安全获取一份 Printer 的引用
    pub fn printer(&self) -> Printer {
        self.state.printer.clone()
    }
}
