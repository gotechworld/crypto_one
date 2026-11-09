# Compile the release binary in a full Rust image.
FROM rust:1-bookworm AS builder

# Use /app as the build directory.
WORKDIR /app

# Copy the manifest and lockfile first so dependency resolution is reproducible.
COPY Cargo.toml Cargo.lock ./

# Copy the crate source that Cargo will compile.
COPY src ./src

# Build the optimized binary using the committed lockfile.
RUN cargo build --locked --release

# Run the binary in a small image that does not include the Rust toolchain.
FROM debian:bookworm-slim

# Create an unprivileged user so the container does not run as root.
RUN useradd --create-home --uid 10001 appuser

# Copy only the finished binary from the build stage.
COPY --from=builder /app/target/release/crypto_one /usr/local/bin/crypto_one

# Drop privileges before starting the program.
USER appuser

# Run the program when the container starts.
ENTRYPOINT ["crypto_one"]
