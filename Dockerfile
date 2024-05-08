FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
ARG service_variant
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim as runner
RUN apt-get update -y
RUN apt-get install wget -y
# Required to install mysql
# libmysqlclient-dev necessary for diesel's mysql integration
RUN apt-get install -y default-libmysqlclient-dev
# Add Oracle MySQL repository
RUN apt-get update
RUN apt-get install -y gnupg lsb-release wget

# Copy data-service executable to the readied runner image
FROM runner as service
COPY --from=builder /usr/local/cargo/bin/adapter-client-stripe /usr/local/bin/adapter-client-stripe
EXPOSE 8000
CMD ["adapter-client-stripe"]