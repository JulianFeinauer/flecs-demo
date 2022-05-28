FROM rust:1.59 as builder
WORKDIR /usr/src/myapp
COPY . .
WORKDIR /usr/src/myapp/client
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/client /usr/local/bin/client
CMD ["client"]
