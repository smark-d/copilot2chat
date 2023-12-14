FROM rust:latest

WORKDIR /usr/src/copilot2chat
COPY . .

RUN cargo install --path .

EXPOSE 2088

CMD ["copilot2chat"]
