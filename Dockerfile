# ---- Stage 1: Build Stage ----
FROM rust:1.79 as builder

WORKDIR /usr/src/sccp-backend

# 复制项目文件
COPY . .

# 安装 sqlx-cli, 它的可执行文件会被放在 /usr/local/cargo/bin/sqlx
RUN cargo install sqlx-cli --version=0.7.4 --no-default-features --features rustls,mysql

# 构建我们自己的应用
RUN cargo build --release

# ---- Stage 2: Final Stage ----
FROM debian:bullseye-slim

# 设置非root用户
RUN groupadd -r appuser && useradd -r -g appuser appuser
USER appuser

# 从构建阶段复制我们编译好的应用
COPY --from=builder /usr/src/sccp-backend/target/release/sccp-backend /usr/local/bin/sccp-backend
# 从构建阶段复制 sqlx-cli 工具
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

EXPOSE 8080

# 默认启动命令只运行应用, 我们将在Render的设置中覆盖它
CMD ["sccp-backend"]
