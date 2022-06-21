FROM rust as builder
WORKDIR /usr/src/web-server
COPY . .
RUN cargo install --debug --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/web-server /usr/local/bin/web-server
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["web-server"]