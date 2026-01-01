# Cryo Package Manager Design

## Overview

The Cryo Package Manager (APM) provides dependency management for Cryo projects.

## Project Structure

```
my-project/
├── cryo.toml          # Project manifest
├── cryo.lock          # Lock file (generated)
├── src/
│   └── main.cryo         # Entry point
├── lib/
│   └── mylib.cryo        # Library code
└── deps/               # Downloaded dependencies
    └── http-utils/
        └── lib.cryo
```

## cryo.toml Format

```toml
[package]
name = "my-project"
version = "1.0.0"
description = "My awesome Cryo project"
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
entry = "src/main.cryo"
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
- [x] cryo.toml parser
- [x] cryo init command
- [x] cryo build command
- [x] Local path dependencies

### Phase 2: Remote (v2.9.0) ✅
- [x] Git dependencies  
- [x] cryo install command
- [x] cryo.lock generation
- [x] cryo list command
- [x] cryo update command

### Phase 3: Publishing (v2.9.0) ✅
- [x] cryo publish command (git tag based)
- [x] cryo remove command
- [x] cryo search command

### Phase 4: Central Registry (v2.10.0) ✅
- [x] registry/index.json format
- [x] apm search - list all packages
- [x] apm search <query> - filter packages
- [x] apm info <pkg> - show package info
- [x] Version resolution from registry
- [x] Fallback to GitHub if registry unavailable

