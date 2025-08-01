// 声明所有顶层模块，让编译器知道它们的存在
use actix::Actor; // <-- 新增
use crate::services::chat_server::ChatServer; // <-- 新增
pub mod api;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;
pub mod config;
mod tests;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::{env, fs};
use actix_files::Files;
use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 如果目录不存在，就递归创建它
    if let Err(e) = fs::create_dir_all("./uploads") {
        eprintln!("Failed to create uploads directory: {}", e);
        // 如果确实需要阻止启动，可以直接 panic
        // panic!("Could not create uploads directory: {}", e);
    }

    // 从 .env 文件加载环境变量
    dotenv().ok();
    // 初始化日志记录器
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 读取数据库连接URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    // 创建数据库连接池
    // --- 使用新的Config模块来获取配置和数据库连接池 ---
    let config = Config::from_env();
    let pool = config.db_pool().await;

    log::info!("Database pool created successfully.");


    // 获取服务地址和端口
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    log::info!("🚀 Server starting at http://{}", server_addr);
    // --- 在 HttpServer::new 之前，启动ChatServer Actor ---
    let chat_server = ChatServer::default().start();

    // 启动HTTP服务器
    HttpServer::new(move || {
        // 配置CORS（跨域资源共享）
        let cors = Cors::default()
            .allow_any_origin() // 允许任何来源的请求，在生产环境中应配置得更严格
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // 将数据库连接池共享给所有处理器
            .app_data(web::Data::new(pool.clone()))
            // --- 将ChatServer的地址共享给所有处理器 ---
            .app_data(web::Data::new(chat_server.clone()))
            // 启用日志中间件
            .wrap(Logger::default())
            // 启用CORS中间件
            .wrap(cors)
            // 配置API路由
            .configure(api::config)
            .service(Files::new("/uploads", "./uploads").show_files_listing())
    })
        .bind(&server_addr)?
        .run()
        .await
}
