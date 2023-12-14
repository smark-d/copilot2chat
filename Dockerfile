# Stage 1: Build
FROM rust:latest AS builder

WORKDIR /usr/src/copilot2chat
COPY . .

RUN cargo install --path .

# Stage 2: Package
FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/copilot2chat /usr/local/bin/copilot2chat

EXPOSE 2088

CMD ["copilot2chat"]
