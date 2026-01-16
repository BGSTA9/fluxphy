FROM rust:latest as builder
WORKDIR /usr/src/fluxphy
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/fluxphy/target/release/fluxphy /usr/local/bin/fluxphy
ENTRYPOINT ["fluxphy"]
