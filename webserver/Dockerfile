FROM rust:1.61.0 as builder
WORKDIR /usr/src/webserver
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ /usr/local/bin/
CMD ["web"]



