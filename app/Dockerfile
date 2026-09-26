# Stage 1: Planner
FROM lukemathwalker/cargo-chef:latest-rust-1.98-alpine AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Builder
FROM lukemathwalker/cargo-chef:latest-rust-1.98-alpine AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin ff-codex

# Stage 3: Runtime
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/ff-codex /app/ff-codex
USER nonroot
WORKDIR /app
EXPOSE 8080
CMD ["/app/ff-codex"]
