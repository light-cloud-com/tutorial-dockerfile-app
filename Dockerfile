# Stage 1: compile. The Rust toolchain is large, but it never reaches production.
FROM rust:1.97-slim AS builder
WORKDIR /app

# Build the dependencies first, with an empty main.rs, so this slow layer is
# cached and only rebuilt when Cargo.toml or Cargo.lock change.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: run. Only the binary goes into a small Debian image.
FROM debian:bookworm-slim
RUN useradd --system --uid 10001 app
COPY --from=builder /app/target/release/tutorial-dockerfile-app /usr/local/bin/app
USER app

# Light Cloud reads EXPOSE to know which port to send traffic to.
ENV PORT=8080
EXPOSE 8080
CMD ["app"]
