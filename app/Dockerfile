FROM rust:1.97-slim AS build-env
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --bin ff-codex
RUN mkdir -p /bundle \
    && cp /app/target/release/ff-codex /bundle/ff-codex \
    && chown -R 65532:65532 /bundle

FROM gcr.io/distroless/cc-debian12
COPY --chown=65532:65532 --from=build-env /bundle /bundle
USER nonroot
WORKDIR /bundle
EXPOSE 8080
CMD ["/bundle/ff-codex"]
