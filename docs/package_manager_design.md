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

[dependencies]
http-utils = "1.0.0"
json-parser = { git = "https://github.com/user/json-parser", tag = "v1.0.0" }
local-lib = { path = "../local-lib" }

[dev-dependencies]
test-framework = "0.1.0"
```

## Commands

| Command | Description |
|---------|-------------|
| `argon init` | Create new project with argon.toml |
| `argon install` | Install all dependencies |
| `argon add <pkg>` | Add a dependency |
| `argon remove <pkg>` | Remove a dependency |
| `argon update` | Update dependencies |
| `argon build` | Build the project |
| `argon run` | Run the project |
| `argon publish` | Publish to registry |

## Dependency Resolution

1. Parse `argon.toml`
2. Resolve transitive dependencies
3. Check version compatibility
4. Download to `deps/` folder
5. Generate `argon.lock`

## Registry

For v1, we'll use a simple file-based registry:
- GitHub repositories with `argon.toml`
- Version tags for releases

## Implementation Plan

### Phase 1: Core (v2.8.0)
- [x] argon.toml parser
- [x] argon init command
- [x] argon build command (with deps)
- [x] Local path dependencies

### Phase 2: Remote (v2.9.0)
- [ ] Git dependencies  
- [ ] argon install command
- [ ] argon.lock generation

### Phase 3: Registry (v3.0.0)
- [ ] Central registry
- [ ] argon publish command
- [ ] Version resolution algorithm
