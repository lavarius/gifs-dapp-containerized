# Use an official Ubuntu as a parent image
FROM ubuntu:24.04

# Set environment variables to avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev \
    llvm \
    libclang-dev \
    protobuf-compiler \
    libssl-dev \
    curl \
    sudo \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install 1.78.0
RUN rustup default 1.78.0
RUN rustup component add rustfmt
RUN rustup component add clippy

# Install Cargo 1.78
# RUN cargo --version

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.17.25/install)"
# Ensure the Solana CLI is added to the PATH
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"
RUN solana --version

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.0 anchor-cli --locked
RUN anchor --version

# Install yarn
RUN npm install -g yarn@1.22.22

# Set up the working directory
WORKDIR /workdir

# Set default command to start a shell
CMD ["/bin/bash"]