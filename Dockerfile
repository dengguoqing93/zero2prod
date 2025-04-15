FROM rust:latest
# 把工作目录设置为app
# `app` 文件夹将由docker为我们创建，防止它不存在    
WORKDIR /app

# 为链接配置安装所需的系统依赖 
RUN apt  update && apt  install -y lld clang
# 把当前目录下的所有文件复制到app目录下
COPY . .
ENV SQLX_OFFLINE=true
# 开始构建二进制文件，使用release参数以提高速度
RUN cargo build --release

# 运行app
ENTRYPOINT ["./target/release/emailReport"]

