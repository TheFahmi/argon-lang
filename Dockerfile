FROM rust:slim

# Install C++ compiler (g++) and clang
RUN apt-get update && apt-get install -y \
    g++ clang llvm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Source
COPY Cargo.toml .
COPY src/ ./src/

# Build Argon (Release mode for max speed)
# This produces a Linux binary inside the container
RUN cargo build --release

# Copies binary to path
RUN cp target/release/argon /usr/bin/argon

# Copy Benchmarks
COPY benchmarks/comparison/ ./benchmarks/

# Work in benchmark dir
WORKDIR /app/benchmarks

# Make script executable
RUN chmod +x run.sh

# Run benchmarks
CMD ["./run.sh"]
