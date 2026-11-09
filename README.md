# crypto_one

A small Rust demo that derives an encryption key from a passphrase, encrypts a payload with AES-256-GCM, and authenticates the ciphertext with HMAC-SHA256.

The program prints the ciphertext and the MAC as hex. Each run uses a fresh nonce and salt, so the printed values change even when the passphrase and plaintext stay the same.

## What it does

1. Derives a 256-bit key from the passphrase with PBKDF2-HMAC-SHA256 (600,000 rounds) and a random 16-byte salt.
2. Encrypts `Sensitive data payload.` with AES-256-GCM and a random 96-bit nonce.
3. Computes an HMAC-SHA256 over the ciphertext, using the same derived key.
4. Prints both results:

```text
Ciphertext: <hex>
MAC: <hex>
```

This is a one-shot demonstration. The salt and nonce are not saved, so the ciphertext cannot be decrypted in a later run.

## Layout

| Path | Role |
| --- | --- |
| `src/main.rs` | Wires the steps together and prints the results |
| `src/crypto/key.rs` | Passphrase-to-key derivation |
| `src/crypto/aes_256_gcm.rs` | AES-256-GCM encrypt |
| `src/crypto/sha256.rs` | HMAC-SHA256 |
| `src/utils.rs` | Nonce generation |
| `src/error.rs` | Shared error type |

`cryptohash/` is a separate, unused crate and is not part of this build.

## Requirements

- [Rust](https://rustup.rs/) with edition 2024 support (Rust 1.85 or newer)
- A network connection the first time, so Cargo can download dependencies

Check the toolchain:

```bash
rustc --version
cargo --version
```

## Compile and run

From this directory:

```bash
cargo build
cargo run
```

`cargo run` compiles if needed, then runs the program. The first build downloads crates and takes longer. Key derivation also takes a moment because of the PBKDF2 iteration count.

Release build:

```bash
cargo build --release
cargo run --release
```

The optimized binary is at `target/release/crypto_one`.

## Docker

Build and run the same release binary in a container:

```bash
docker build -t crypto_one .
docker run --rm crypto_one
```

The image builds with `rust:1-bookworm` and runs the binary on `debian:bookworm-slim`. Output changes on every run because the salt and nonce are random.

## Check the project

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

`cargo test` covers the AES-256-GCM round trip and checks that the HMAC changes when the input changes.

GitHub Actions runs the same checks on every push and pull request (`.github/workflows/ci.yml`): formatting, Clippy, tests, a release build, and a release smoke run. Commit `Cargo.lock` so CI can use `--locked`.
