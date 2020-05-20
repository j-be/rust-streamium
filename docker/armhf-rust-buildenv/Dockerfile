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
    libsasl2-modules-gssapi-mit:armhf \
    libpq5:armhf \
    libpq5:amd64

################################################################################
## Rust
##

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >> $HOME/rustup.sh
RUN RUST_HOME=/opt/cargo CARGO_HOME=/opt/cargo sh $HOME/rustup.sh -y \
    --default-toolchain nightly \
    --target armv7-unknown-linux-gnueabihf

# Copy cargo config
COPY docker/armhf-rust-buildenv/files/cargo.conf /cargo.conf
RUN mkdir -p $HOME/.cargo && mv /cargo.conf $HOME/.cargo/config

################################################################################
## Configure environment
##

VOLUME /root/.cargo/registry
ENV PATH="/opt/cargo/bin/:${PATH}"

################################################################################
## Copy sources
##

COPY streamium-db/ /streamium-db/
COPY streamium-importer/ /streamium-importer/
COPY streamium-server/ /streamium-server/

################################################################################
## Copy buildscript
##
COPY docker/armhf-rust-buildenv/files/build_with.sh /usr/local/sbin/build_with
RUN chmod +x /usr/local/sbin/build_with

################################################################################
## Build
##

WORKDIR /streamium-server/
ENTRYPOINT rm -rf target/* && build_with armhf || build_with amd64 || build_with armhf
