use bdtv_hub::{App, load_conf, logging};
use dotenvy::dotenv;
use tracing::error;

#[tokio::main]
async fn main() {
    // 加载 .env 文件到环境变量
    dotenv().ok();

    // 加载配置
    let config = match load_conf() {
        Ok(config) => config,
        Err(e) => {
            error!("failed to load config: {e}");
            return;
        }
    };

    // 创建应用程序实例
    let app = App::new(config);

    // 初始化日志系统
    logging::init(app.printer());

    // 运行应用程序
    app.run().await;
}
