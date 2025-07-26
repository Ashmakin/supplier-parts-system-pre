#---- Stage 1: Build Stage ----
# 使用一个明确的、较新的Rust版本
FROM rust:1.88 as builder

# 设置工作目录
WORKDIR /usr/src/supplier-parts-system

# 复制所有项目文件
COPY . .

# 安装 sqlx-cli 工具。它的可执行文件会被放在 /usr/local/cargo/bin/sqlx
# 我们使用一个已知的兼容版本来保证稳定性
RUN cargo install sqlx-cli --version=0.7.4 --no-default-features --features rustls,mysql

# 构建我们自己的主应用，并生成发布版本的可执行文件
RUN cargo build --release

# ---- Stage 2: Final Stage ----
# 使用一个非常小的基础镜像
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl3
# 为安全起见，设置非root用户
RUN groupadd -r appuser && useradd -r -g appuser appuser
USER appuser

# 从构建阶段复制我们编译好的主应用可执行文件
COPY --from=builder /usr/src/supplier-parts-system/target/release/supplier-parts-app /usr/local/bin/supplier-parts-app

# 【关键修复】从构建阶段复制 sqlx-cli 工具的可执行文件
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# 暴露应用端口
EXPOSE 8080

# 设置默认的启动命令。我们将在Render的设置中覆盖它以先运行迁移。
CMD ["supplier-parts-app"]
