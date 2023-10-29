FROM rust:1.73 as chef

# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --bin https-api-proxy

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev curl && rm -rf /var/lib/apt/lists/*
WORKDIR app
COPY --from=builder /app/target/release/https-api-proxy /usr/local/bin
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/https-api-proxy"]