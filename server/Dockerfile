FROM lukemathwalker/cargo-chef:latest-rust-1-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
COPY .env.dev .env
RUN cargo build --release --bin binge-at-home --features migrate

FROM debian:bullseye AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/binge-at-home /usr/local/bin
COPY docker/config/app.yml config/app.yml
COPY templates/assets assets
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/binge-at-home"]
