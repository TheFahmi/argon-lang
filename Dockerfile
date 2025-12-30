# Argon Toolchain v2.7.1
# Pre-built with snake_case intrinsics
FROM rust:slim

RUN apt-get update && apt-get install -y clang llvm && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy pre-built compiler and runtime
COPY self-host/new_argonc /usr/bin/argonc
COPY self-host/libruntime_new.a /usr/lib/libruntime_argon.a

# Verify installation
RUN chmod +x /usr/bin/argonc

CMD ["bash"]
