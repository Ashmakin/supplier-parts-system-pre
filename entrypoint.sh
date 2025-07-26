#!/bin/sh
# 这个脚本将作为我们容器的入口点

# 当任何命令失败时，立即退出脚本，这能防止应用在数据库迁移失败时启动
set -e

# 1. 运行数据库迁移
# 我们明确地告诉 sqlx-cli 在哪里找到数据库 URL
echo "Running database migrations..."
/usr/local/bin/sqlx migrate run

# 2. 启动主应用程序
# 'exec' 会用主应用进程替换掉脚本进程，这是一个好的实践
echo "Starting application..."
exec /usr/local/bin/supplier-parts-app
