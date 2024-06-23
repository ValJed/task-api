FROM rust:1.77 as builder
WORKDIR /usr/src/task-api
COPY . .
RUN cargo install --path .
RUN cargo build --release 

FROM debian:bookworm-slim
RUN apt update && apt install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/task-api /usr/local/bin/task-api

CMD ["/usr/local/bin/task-api"]
