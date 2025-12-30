# Argon Package Manager Design

## Overview

The Argon Package Manager (APM) provides dependency management for Argon projects.

## Project Structure

```
my-project/
├── argon.toml          # Project manifest
├── argon.lock          # Lock file (generated)
├── src/
│   └── main.ar         # Entry point
├── lib/
│   └── mylib.ar        # Library code
└── deps/               # Downloaded dependencies
    └── http-utils/
        └── lib.ar
```

## argon.toml Format

```toml
[package]
name = "my-project"
version = "1.0.0"
description = "My awesome Argon project"
author = "Your Name"
license = "MIT"
repository = "https://github.com/user/my-project"

[dependencies]
# Registry dependency
http-utils = "1.0.0"

# Git dependency
json-parser = { git = "https://github.com/user/json-parser", tag = "v1.0.0" }

# Local path dependency  
local-lib = { path = "../local-lib" }

[dev-dependencies]
test-framework = "0.1.0"

[build]
entry = "src/main.ar"
output = "build"
```

## Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `apm init [name]` | | Create new project |
| `apm build [file]` | `b` | Build the project |
| `apm run [file]` | `r` | Build and run |
| `apm install` | `i` | Install all dependencies |
| `apm add <pkg>` | `a` | Add a dependency |
| `apm remove <pkg>` | `rm` | Remove a dependency |
| `apm update` | `up` | Update all dependencies |
| `apm list` | `ls` | List installed dependencies |
| `apm search <query>` | `s` | Search for packages |
| `apm publish` | | Publish to registry |
| `apm clean` | | Remove build artifacts |

## Dependency Types

```bash
# Local path
apm add ../my-lib --path

# GitHub repository
apm add user/repo --git

# GitHub with specific version
apm add user/repo@v1.0.0 --git

# Registry (future)
apm add package-name
```

## Implementation Status

### Phase 1: Core (v2.8.0) ✅
- [x] argon.toml parser
- [x] argon init command
- [x] argon build command
- [x] Local path dependencies

### Phase 2: Remote (v2.9.0) ✅
- [x] Git dependencies  
- [x] argon install command
- [x] argon.lock generation
- [x] argon list command
- [x] argon update command

### Phase 3: Publishing (v2.9.0) ✅
- [x] argon publish command (git tag based)
- [x] argon remove command
- [x] argon search command

### Future: Central Registry
- [ ] Central package registry server
- [ ] Version resolution algorithm
- [ ] Semantic versioning support

