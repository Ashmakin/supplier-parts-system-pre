// src/config.rs

use dotenv::dotenv;
use sqlx::{MySql, MySqlPool, Pool};
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        //dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self { database_url }
    }

    pub async fn db_pool(&self) -> Pool<MySql> {
        MySqlPool::connect(&self.database_url)
            .await
            .expect("Failed to create database pool.")
    }
}

/// 专门为测试环境配置数据库连接
pub async fn configure_test_db() -> Pool<MySql> {
    dotenv().ok();

    // 1. 构建测试数据库的 URL
    // 我们将从主 DATABASE_URL 中替换数据库名称
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let test_db_url = db_url.replace("sccp_db", "sccp_db_test");

    // 2. 连接到测试数据库
    let pool = MySqlPool::connect(&test_db_url)
        .await
        .expect("Failed to connect to test database");

    // 3. (可选但推荐) 在运行测试前清理数据
    // 这确保了每次测试都在一个干净的环境中进行，互不干扰
    sqlx::query("DELETE FROM users").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM companies").execute(&pool).await.unwrap();

    pool
}
