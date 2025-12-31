FROM rust:slim

# Install C++ compiler (g++), clang, and LLVM
RUN apt-get update && apt-get install -y \
    g++ clang llvm time \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Source
COPY Cargo.toml .
COPY src/ ./src/
COPY self-host/ ./self-host/

# Build Argon Interpreter (Release mode for max speed)
RUN cargo build --release

# Copy binary to path
RUN cp target/release/argon /usr/bin/argon

# Copy Benchmarks and stdlib
COPY benchmarks/comparison/ ./benchmarks/
COPY stdlib/ ./stdlib/

# Work in benchmark dir
WORKDIR /app/benchmarks

# Make script executable
RUN chmod +x run.sh

# Run benchmarks
CMD ["./run.sh"]
