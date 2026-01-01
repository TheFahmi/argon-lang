/**
 * Cryo WASM Loader
 * JavaScript helper for loading and interacting with Cryo WASM modules
 * Version: 2.24.0
 */

class CryoWASM {
    constructor() {
        this.instance = null;
        this.memory = null;
        this.exports = null;
        this.textDecoder = new TextDecoder('utf-8');
        this.textEncoder = new TextEncoder();
        this.heapPtr = 4096; // Start heap after reserved space
    }

    /**
     * Load a WASM module from URL
     * @param {string} wasmPath - Path to the .wasm file
     * @returns {Promise<object>} - Exported functions
     */
    async load(wasmPath) {
        // Create shared memory
        this.memory = new WebAssembly.Memory({ initial: 256, maximum: 256 });

        // Import object with WASI and console functions
        const importObject = {
            env: {
                memory: this.memory,
            },
            wasi_snapshot_preview1: {
                fd_write: (fd, iovs, iovsLen, nwritten) => {
                    return this._fd_write(fd, iovs, iovsLen, nwritten);
                },
                fd_read: () => 0,
                fd_close: () => 0,
                fd_seek: () => 0,
                fd_prestat_get: () => 8, // EBADF
                fd_prestat_dir_name: () => 8,
                environ_sizes_get: (environCount, environBufSize) => {
                    const view = new DataView(this.memory.buffer);
                    view.setUint32(environCount, 0, true);
                    view.setUint32(environBufSize, 0, true);
                    return 0;
                },
                environ_get: () => 0,
                args_sizes_get: (argc, argvBufSize) => {
                    const view = new DataView(this.memory.buffer);
                    view.setUint32(argc, 0, true);
                    view.setUint32(argvBufSize, 0, true);
                    return 0;
                },
                args_get: () => 0,
                proc_exit: (code) => {
                    console.log(`Process exited with code: ${code}`);
                },
                clock_time_get: (clockId, precision, time) => {
                    const view = new DataView(this.memory.buffer);
                    const now = BigInt(Date.now()) * 1000000n; // Convert to nanoseconds
                    view.setBigUint64(time, now, true);
                    return 0;
                },
            },
            console: {
                log: (ptr, len) => {
                    const message = this.readString(ptr, len);
                    console.log(message);
                },
            },
        };

        try {
            const response = await fetch(wasmPath);
            const bytes = await response.arrayBuffer();
            const result = await WebAssembly.instantiate(bytes, importObject);

            this.instance = result.instance;
            this.exports = result.instance.exports;

            // Use module's memory if exported
            if (this.exports.memory) {
                this.memory = this.exports.memory;
            }

            return this.exports;
        } catch (error) {
            console.error('Failed to load WASM:', error);
            throw error;
        }
    }

    /**
     * WASI fd_write implementation
     */
    _fd_write(fd, iovs, iovsLen, nwritten) {
        const view = new DataView(this.memory.buffer);
        let totalWritten = 0;

        for (let i = 0; i < iovsLen; i++) {
            const ptr = view.getUint32(iovs + i * 8, true);
            const len = view.getUint32(iovs + i * 8 + 4, true);

            const bytes = new Uint8Array(this.memory.buffer, ptr, len);
            const str = this.textDecoder.decode(bytes);

            if (fd === 1) {
                // stdout
                process?.stdout?.write?.(str) ?? console.log(str.trimEnd());
            } else if (fd === 2) {
                // stderr
                console.error(str.trimEnd());
            }

            totalWritten += len;
        }

        view.setUint32(nwritten, totalWritten, true);
        return 0;
    }

    /**
     * Read a string from WASM memory
     * @param {number} ptr - Pointer to string
     * @param {number} len - String length
     * @returns {string}
     */
    readString(ptr, len) {
        const bytes = new Uint8Array(this.memory.buffer, ptr, len);
        return this.textDecoder.decode(bytes);
    }

    /**
     * Write a string to WASM memory
     * @param {string} str - String to write
     * @returns {object} - { ptr, len }
     */
    writeString(str) {
        const bytes = this.textEncoder.encode(str);
        const ptr = this.alloc(bytes.length + 1);

        const dest = new Uint8Array(this.memory.buffer, ptr, bytes.length + 1);
        dest.set(bytes);
        dest[bytes.length] = 0; // Null terminator

        return { ptr, len: bytes.length };
    }

    /**
     * Simple bump allocator
     * @param {number} size - Bytes to allocate
     * @returns {number} - Pointer to allocated memory
     */
    alloc(size) {
        const ptr = this.heapPtr;
        this.heapPtr += size;
        // Align to 8 bytes
        this.heapPtr = (this.heapPtr + 7) & ~7;
        return ptr;
    }

    /**
     * Call a function with automatic string conversion
     * @param {string} name - Function name
     * @param {...any} args - Arguments
     * @returns {any} - Return value
     */
    call(name, ...args) {
        if (!this.exports || !this.exports[name]) {
            throw new Error(`Function '${name}' not found in WASM exports`);
        }

        // Convert string arguments
        const wasmArgs = args.map(arg => {
            if (typeof arg === 'string') {
                const { ptr, len } = this.writeString(arg);
                return [ptr, len];
            }
            return arg;
        }).flat();

        return this.exports[name](...wasmArgs);
    }

    /**
     * Run the main/_start function
     */
    run() {
        if (this.exports._start) {
            return this.exports._start();
        } else if (this.exports.main) {
            return this.exports.main();
        } else {
            throw new Error('No entry point found (_start or main)');
        }
    }
}

/**
 * Helper function to load Cryo WASM module
 * @param {string} wasmPath - Path to WASM file
 * @returns {Promise<object>} - Exported functions
 */
async function loadCryoModule(wasmPath) {
    const cryo = new CryoWASM();
    return await cryo.load(wasmPath);
}

// Export for Node.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { CryoWASM, loadCryoModule };
}
