# Argon Toolchain v2.18.0
# Binary shows v2.15.0 banner (runtime limitation)
# Source code has v2.18.0 features (async/await, debugger)
FROM rust:slim

RUN apt-get update && apt-get install -y clang llvm gdb && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# v2.18.0 compiler binary
COPY self-host/argonc_v218 /usr/bin/argonc
COPY self-host/libruntime_new.a /usr/lib/libruntime_argon.a

RUN chmod +x /usr/bin/argonc

CMD ["bash"]
