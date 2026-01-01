#!/bin/bash
echo "Starting Cryo REPL..."
# Check for tty
if [ -t 1 ]; then
    TTY_FLAG="-it"
else
    TTY_FLAG="-i"
fi

docker run $TTY_FLAG --rm -v "$(pwd -W 2>/dev/null || pwd):/src" -w //src cryo-toolchain bash -c "cryoc tools/repl.ar && clang++ -O0 -Wno-override-module tools/repl.ar.ll /usr/lib/libruntime_cryo.a -o tools/repl -lpthread -ldl && ./tools/repl"
