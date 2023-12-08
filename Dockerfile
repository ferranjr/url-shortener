FROM rust:1.74.0-buster as builder
WORKDIR /app
COPY .. .
RUN cargo install --path .


FROM debian:buster-slim as runner

COPY --from=builder /usr/local/cargo/bin/url-shortener /usr/local/bin/url-shortener

EXPOSE 8080

CMD ["url-shortener"]