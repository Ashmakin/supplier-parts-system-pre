# ---- Stage 1: Build Stage ----
# 使用官方的Rust镜像作为构建环境
FROM rust:1.88 as builder

# 设置工作目录
WORKDIR /usr/src/supplier-parts-system

# 复制项目文件
COPY . .

# 安装 sqlx-cli 用于数据库迁移
RUN cargo install sqlx-cli --no-default-features --features rustls,mysql

# 运行数据库迁移 (这一步确保我们的生产数据库有正确的表结构)
# 注意: 部署时需要设置 DATABASE_URL 环境变量
RUN sqlx migrate run

# 构建发布版本的可执行文件
RUN cargo build --release

# ---- Stage 2: Final Stage ----
# 使用一个非常小的基础镜像来减小最终镜像的体积和安全风险
FROM debian:bullseye-slim

# 设置非root用户运行，增加安全性
RUN groupadd -r appuser && useradd -r -g appuser appuser
USER appuser

# 从构建阶段复制编译好的可执行文件到最终镜像
COPY --from=builder /usr/src/supplier-parts-system/target/release/supplier-parts-app /usr/local/bin/supplier-parts-app

# 暴露我们应用运行的端口 (8080)
EXPOSE 8080

# 容器启动时运行的命令
CMD ["supplier-parts-app"]
