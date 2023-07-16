FROM rust:latest as builder
WORKDIR /usr/src/kracked-web
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    pkg-config \
    libpq-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/kracked-web
COPY --from=builder /usr/local/cargo/bin/kracked-web /usr/src/kracked-web
COPY ./frontend /usr/src/kracked-web/frontend
EXPOSE 3000
CMD ["./kracked-web"]