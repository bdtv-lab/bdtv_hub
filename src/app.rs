mod event;
mod state;

use std::sync::Arc;

use smaragdine::Printer;
use tokio::{sync::mpsc, task::JoinSet};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{
    console::console,
    envconf::Config,
    qq::{get_ws_client, qq_connector},
    server::http_server,
    signal::shutdown_signal,
    warden::warden,
};
pub use event::EventToQQ;
pub use state::State;

/// 应用程序的主结构体
pub struct App {
    state: Arc<State>,
    token: CancellationToken,
    rx: mpsc::Receiver<EventToQQ>,
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
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
        // 启动 qq 消息发送
        if let Some(req) = get_ws_client(config.clone()).await {
            tasks.spawn(qq_connector(
            req,
            rx,
            token.clone(),
        ));
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
                error!("发生 panic: {e}");
            }
        }

        info!("服务已退出");
    }

    /// 安全获取一份 Printer 的引用
    pub fn printer(&self) -> Printer {
        self.state.printer.clone()
    }
}
