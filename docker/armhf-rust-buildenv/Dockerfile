FROM ubuntu:18.04

################################################################################
## Toolchain and libs
##

# Install toolchain
RUN apt-get update
RUN apt-get install -y crossbuild-essential-armhf

# Enable armhf packages
COPY docker/armhf-rust-buildenv/files/armhf.list $HOME/etc/apt/sources.list.d/
RUN dpkg --add-architecture armhf

# Install additional libraries
RUN apt-get update || echo ""
RUN apt-get install -y \
    curl \
    libpq-dev:armhf

################################################################################
## Rust
##

# Copy cargo config
COPY docker/armhf-rust-buildenv/files/cargo.conf $HOME/.cargo/config

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >> $HOME/rustup.sh
RUN sh $HOME/rustup.sh -y \
    --default-toolchain nightly \
    --target armv7-unknown-linux-gnueabihf

################################################################################
## Copy sources
##

COPY streamium-db/ /streamium-db/
COPY streamium-importer/ /streamium-importer/
COPY streamium-server/ /streamium-server/

################################################################################
## Final stuff
##

WORKDIR /streamium-server/
VOLUME /streamium-server/target

ENTRYPOINT $HOME/.cargo/bin/cargo build --target=armv7-unknown-linux-gnueabihf  --release
