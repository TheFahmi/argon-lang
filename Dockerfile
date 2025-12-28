# Argon Toolchain v2.3
# Pre-built with multi-threading support
FROM rust:slim

RUN apt-get update && apt-get install -y clang llvm && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy pre-built compiler and runtime
COPY self-host/argonc_v23 /usr/bin/argonc
COPY self-host/libruntime_new.a /usr/lib/libruntime_argon.a

# Verify installation
RUN chmod +x /usr/bin/argonc

CMD ["bash"]
