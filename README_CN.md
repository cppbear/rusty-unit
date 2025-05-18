# install

:./compiler$ cargo install --path . --features instrumentation,analysis,file_writer \
:./compiler$ cargo install --path . --features instrumentation

# 完全使用

## 安装 PostgreSQL 14+

sudo apt install postgresql postgresql-contrib \
sudo systemctl start postgresql

## 创建用户和数据库

sudo -u postgres psql -c "CREATE USER rustyunit WITH PASSWORD 'postgres';" \
sudo -u postgres psql -c "CREATE DATABASE rustyunit_db OWNER rustyunit;"

## 安装 redis

sudo apt install redis-server \
sudo systemctl start redis-server

## 安装 jdk-17

sudo apt install openjdk-17-jdk \
wget https://services.gradle.org/distributions/gradle-8.0-bin.zip \
unzip gradle-8.0-bin.zip \
sudo mv gradle-8.0 /opt/ \
echo 'export PATH=$PATH:/opt/gradle-8.0/bin' >> ~/.bashrc \
source ~/.bashrc \

## VSCode：

安装 Lombok Annotations Support for VS Code 扩展

## 在项目根目录执行

make build \
脚本不能直接用，需要手动执行命令

## 设置环境变量

export RU_BIN=\$(pwd)/../../bin \
export RU_MONITOR=$(pwd)/../../compiler/src/monitor.rs

## 执行分析

make analyze # 生成 analysis/ 目录 \
脚本不能直接用，需要手动执行命令

## 在目标项目根目录执行

make dynamosa RUN=1 # 生成 rusty-unit-1 目录

## 执行测试套件

make execute # 输出测试结果

## 生成覆盖率报告

make coverage # 生成 coverage/data.json
