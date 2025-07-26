# ---- Stage 1: Build Stage ----
# 使用一个明确的、较新的Rust版本 (基于Debian Bookworm)
FROM rust:1.88 as builder

WORKDIR /usr/src/supplier-parts-system

# 复制所有项目文件
COPY . .

# 安装一个已知的兼容版本的 sqlx-cli
RUN cargo install sqlx-cli --version=0.7.4 --no-default-features --features rustls,mysql

# 构建我们的主应用
RUN cargo build --release

# ---- Stage 2: Final Stage ----
# 使用与构建环境匹配的操作系统版本，以避免GLIBC问题
FROM debian:bookworm-slim

# 安装运行时的必要依赖 (OpenSSL)
RUN apt-get update && apt-get install -y libssl3

# 为安全起见，设置非root用户
RUN groupadd -r appuser && useradd -r -g appuser appuser
USER appuser

# 从构建阶段复制编译好的主应用
COPY --from=builder /usr/src/supplier-parts-system/target/release/supplier-parts-app /usr/local/bin/supplier-parts-app

# 从构建阶段复制 sqlx-cli 工具
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# --- 新增步骤：复制并设置启动脚本 ---
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# 暴露应用端口
EXPOSE 8080

# 【关键改动】将容器的启动命令设置为我们的脚本
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
