FROM rust:1-alpine3.17 as builder
WORKDIR /usr/src/kracked-web
COPY . .
RUN apk add musl-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM nginx:stable-alpine3.17-slim 
RUN apk update && apk upgrade

# Install supervisor
RUN apk add supervisor
RUN mkdir -p /var/log/supervisor

WORKDIR /usr/src/kracked-web

COPY nginx.conf /etc/nginx/nginx.conf
COPY domain.crt /etc/nginx/domain.crt
COPY domain.key /etc/nginx/domain.key

RUN mkdir -p /var/www/blogs
COPY /var/www/blogs /var/www/blogs

COPY --from=builder /usr/local/cargo/bin/kracked-web /usr/src/kracked-web
COPY ./frontend /usr/src/kracked-web/frontend
EXPOSE 443
EXPOSE 3000

# Copy the supervisor configuration
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
# Run the application
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
