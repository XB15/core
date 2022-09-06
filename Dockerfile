FROM ubuntu:18.04

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
        curl \
        git \
        file \
        build-essential \
        libclang-dev \
#        libc++-dev \
#        libc++abi-dev \
        gcc-multilib \
    && apt-get install -y \
        gcc-aarch64-linux-gnu \
        g++-aarch64-linux-gnu

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y \
    && . "$HOME/.cargo/env" \
    && rustup toolchain install nightly \
    && rustup target add aarch64-unknown-linux-gnu

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash \
    && export NVM_DIR="$HOME/.nvm" \
    && [ -s "$NVM_DIR/nvm.sh" ] && . "$NVM_DIR/nvm.sh" \
    && nvm install v16 \
    && npm i --location=global yarn \
    && yarn config set --home enableTelemetry 0