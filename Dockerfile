FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim as service
COPY --from=builder /usr/local/cargo/bin/adapter-client-stripe /usr/local/bin/adapter-client-stripe
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["adapter_client_stripe"]
