FROM rust:1.59 as builder
WORKDIR /usr/src/myapp
COPY . .
WORKDIR /usr/src/myapp/client
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]
