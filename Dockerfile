FROM rust:1.83 AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:latest

WORKDIR /app

COPY --from=builder /app/target/release/api-server /usr/local/bin/api-server

EXPOSE 8080

CMD ["api-server"]
