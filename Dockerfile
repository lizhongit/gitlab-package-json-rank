FROM rust:1.39 AS builder

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release

FROM ubuntu:latest
ENV TZ=Asia/Shanghai
WORKDIR /root/
RUN apt update && apt install libssl-dev -y && apt install ca-certificates -y
COPY --from=builder /usr/src/myapp/target/release/gitlab-package-json-rank .
CMD ["./gitlab-package-json-rank", "./config.yaml"]
