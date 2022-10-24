# syntax=docker/dockerfile:1

# Using multistage build:
# 	https://docs.docker.com/develop/develop-images/multistage-build/
# 	https://whitfin.io/speeding-up-rust-docker-builds/

########################## BUILD IMAGE  ##########################
FROM ghcr.io/j-be/rust-diesel-postgres:1.64-armv7 as build

# Copies the complete project
# To avoid copying unneeded files, use .dockerignore
COPY . .
COPY ./rust-toolchain ./rust-toolchain

RUN rustup target add armv7-unknown-linux-gnueabihf

WORKDIR /app/streamium-server

RUN apt-get install -y libpq-dev:amd64
RUN cargo build --release --target=armv7-unknown-linux-gnueabihf || true
RUN apt-get install -y libpq-dev:armhf
RUN cargo build --release --target=armv7-unknown-linux-gnueabihf
