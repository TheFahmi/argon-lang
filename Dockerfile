# Argon Toolchain v2.16.0
FROM rust:slim

RUN apt-get update && apt-get install -y clang llvm && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiler v2.16.0 and runtime
COPY self-host/argonc_v216 /usr/bin/argonc
COPY self-host/libruntime_new.a /usr/lib/libruntime_argon.a

RUN chmod +x /usr/bin/argonc

CMD ["bash"]
