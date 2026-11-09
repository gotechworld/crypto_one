# Build the release binary, then copy it into a small runtime image.
FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --locked --release

FROM debian:bookworm-slim

RUN useradd --create-home --uid 10001 appuser

COPY --from=builder /app/target/release/crypto_one /usr/local/bin/crypto_one

USER appuser

ENTRYPOINT ["crypto_one"]
