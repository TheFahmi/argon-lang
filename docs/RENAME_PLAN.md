# Project Rename Plan: Argon → Cryo

## Overview

**Current Name:** Argon  
**New Name:** Cryo  
**Date:** 2026-01-01  
**Reason:** "Argon" sudah digunakan oleh bahasa pemrograman lain (arlang.io)

---

## 1. Branding Changes

### 1.1 Core Identity

| Item | Old | New |
|------|-----|-----|
| Language Name | Argon | Cryo |
| File Extension | `.ar` | `.cryo` or `.cry` |
| CLI Binary | `argon` / `argon.exe` | `cryo` / `cryo.exe` |
| Version Banner | `Argon Native v3.2.1` | `Cryo v3.2.1` |
| Package Name (Cargo) | `argon` | `cryo` |
| GitHub Repo | `TheFahmi/argon-lang` | `TheFahmi/cryo-lang` |

### 1.2 Web Framework

| Item | Old | New |
|------|-----|-----|
| Framework Name | ArgonWeb | CryoWeb |
| CLI Script | `argonweb-cli.sh` | `cryoweb-cli.sh` |

---

## 2. Files to Update

### 2.1 Core Files

| File | Changes |
|------|---------|
| `Cargo.toml` | Change `name = "argon"` to `name = "cryo"` |
| `README.md` | Replace all "Argon" with "Cryo" |
| `AGENTS.md` | Update all references |
| `ROADMAP.md` | Update all references |
| `LICENSE` | Update if project name is mentioned |

### 2.2 Source Code (`src/`)

| File | Changes |
|------|---------|
| `src/main.rs` | Update version banner, help text |
| `src/interpreter.rs` | Update `argon_*` builtins to `cryo_*` |
| `src/vm.rs` | Update comments/strings |
| `src/compiler.rs` | Update comments/strings |
| `src/lib.rs` | Update module documentation |
| All `src/*.rs` | Search & replace "Argon" → "Cryo" |

### 2.3 Standard Library (`stdlib/`)

| File | Changes |
|------|---------|
| `stdlib/argonweb.ar` | Rename to `stdlib/cryoweb.cryo` |
| All `stdlib/*.ar` | Rename to `*.cryo` |
| All stdlib files | Update internal references |

### 2.4 Self-Hosted Compiler (`self-host/`)

| File | Changes |
|------|---------|
| `self-host/compiler.ar` | Rename to `compiler.cryo`, update references |
| `self-host/wasm_codegen.ar` | Rename to `wasm_codegen.cryo` |

### 2.5 Examples (`examples/`)

| Action | Details |
|--------|---------|
| Rename all `.ar` files | To `.cryo` extension |
| Update content | Replace "Argon" with "Cryo" in comments |
| Update `argonweb_demo.ar` | Rename to `cryoweb_demo.cryo` |

### 2.6 Tools (`tools/`)

| File | Changes |
|------|---------|
| `tools/argondoc.ar` | Rename to `cryodoc.cryo` |
| `tools/argonfmt.ar` | Rename to `cryofmt.cryo` |
| `tools/repl.ar` | Rename to `repl.cryo` |

### 2.7 Documentation (`docs/`)

| Action | Details |
|--------|---------|
| All `*.md` files | Search & replace "Argon" → "Cryo" |
| All `*.md` files | Update `.ar` references to `.cryo` |

### 2.8 Build & Config Files

| File | Changes |
|------|---------|
| `Dockerfile` | Update binary names, paths |
| `build.sh` | Update binary references |
| `apm.sh` | Update package manager name |
| `argonweb-cli.sh` | Rename to `cryoweb-cli.sh` |
| `.gitignore` | Update paths if needed |

### 2.9 VSCode Extension (`lsp/vscode-extension/`)

| File | Changes |
|------|---------|
| `package.json` | Update extension name, language ID |
| `language-configuration.json` | Update file extensions |
| `syntaxes/*.tmLanguage.json` | Update file extensions, scope names |

---

## 3. Built-in Function Renames

### 3.1 Networking Functions

| Old Name | New Name |
|----------|----------|
| `argon_listen` | `cryoListen` |
| `argon_accept` | `cryoAccept` |
| `argon_socket_read` | `cryoSocketRead` |
| `argon_socket_write` | `cryoSocketWrite` |
| `argon_socket_close` | `cryoSocketClose` |

### 3.2 Async Functions

| Old Name | New Name |
|----------|----------|
| `argon_spawn` | `cryoSpawn` |
| `argon_await` | `cryoAwait` |
| `argon_sleep` | `cryoSleep` |

### 3.3 Backward Compatibility

Untuk backward compatibility, interpreter akan tetap mendukung prefix `argon_*` sebagai alias untuk `cryo*` selama periode transisi.

---

## 4. Execution Plan

### Phase 1: Preparation (Day 1)
- [ ] Create backup branch
- [ ] Create this plan document ✅

### Phase 2: Core Rename (Day 1)
- [ ] Update `Cargo.toml`
- [ ] Update `src/main.rs` version banner
- [ ] Update `src/interpreter.rs` built-in names
- [ ] Build and test

### Phase 3: File Renames (Day 1-2)
- [ ] Create script to rename all `.ar` → `.cryo`
- [ ] Execute file renames
- [ ] Update all internal `import` statements

### Phase 4: Documentation (Day 2)
- [ ] Update `README.md`
- [ ] Update `AGENTS.md`
- [ ] Update `ROADMAP.md`
- [ ] Update all `docs/*.md`

### Phase 5: Infrastructure (Day 2)
- [ ] Rename shell scripts
- [ ] Update `Dockerfile`
- [ ] Update VSCode extension

### Phase 6: Testing (Day 3)
- [ ] Build release binary
- [ ] Run all tests with new extension
- [ ] Verify VSCode extension works
- [ ] Test Docker build

### Phase 7: Repository (Day 3)
- [ ] Rename GitHub repository
- [ ] Update all documentation links
- [ ] Create redirect from old repo

---

## 5. Script untuk Automasi

### 5.1 Rename Files Script (Python)

```python
#!/usr/bin/env python3
"""Rename all .ar files to .cryo"""
import os
import re

def rename_files(root_dir):
    for dirpath, dirnames, filenames in os.walk(root_dir):
        # Skip hidden dirs and target
        dirnames[:] = [d for d in dirnames if not d.startswith('.') and d != 'target']
        
        for filename in filenames:
            if filename.endswith('.ar'):
                old_path = os.path.join(dirpath, filename)
                new_path = os.path.join(dirpath, filename[:-3] + '.cryo')
                os.rename(old_path, new_path)
                print(f"Renamed: {old_path} → {new_path}")

if __name__ == '__main__':
    rename_files('.')
```

### 5.2 Content Replace Script (Python)

```python
#!/usr/bin/env python3
"""Replace Argon with Cryo in all files"""
import os
import re

REPLACEMENTS = [
    ('Argon', 'Cryo'),
    ('argon', 'cryo'),
    ('ARGON', 'CRYO'),
    ('.ar"', '.cryo"'),
    (".ar'", ".cryo'"),
    ('.ar)', '.cryo)'),
    ('.ar,', '.cryo,'),
]

def process_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
    except:
        return False
    
    original = content
    for old, new in REPLACEMENTS:
        content = content.replace(old, new)
    
    if content != original:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False

def main():
    extensions = ['.rs', '.md', '.sh', '.py', '.json', '.toml', '.cryo']
    skip_dirs = ['.git', 'target', 'node_modules']
    
    for dirpath, dirnames, filenames in os.walk('.'):
        dirnames[:] = [d for d in dirnames if d not in skip_dirs]
        
        for filename in filenames:
            if any(filename.endswith(ext) for ext in extensions):
                filepath = os.path.join(dirpath, filename)
                if process_file(filepath):
                    print(f"Updated: {filepath}")

if __name__ == '__main__':
    main()
```

---

## 6. New Taglines & Branding

### Taglines
- **Primary:** "Cryo - High-Performance Systems Language"
- **Alternative:** "Keep your code frozen solid"
- **Technical:** "Fast, Safe, Self-Hosted"

### Logo Concept
- Theme: Snowflake / Ice crystal
- Colors: Blue/White gradient
- Symbol: Stylized "C" with crystalline structure

### Domain Ideas
- `cryo-lang.org`
- `cryolang.io`
- `cryo.dev`

---

## 7. Version Strategy

| Version | Milestone |
|---------|-----------|
| v3.2.1 | Last Argon release |
| v4.0.0 | First Cryo release (breaking: file extension change) |
| v4.0.x | Transition period (both `.ar` and `.cryo` supported) |
| v5.0.0 | Remove `.ar` support |

---

## 8. Rollback Plan

Jika ada masalah besar:
1. Revert to backup branch
2. Keep "Argon" name with disclaimer in README
3. Add note: "Not affiliated with arlang.io Argon"

---

## 9. Estimated Effort

| Phase | Time Estimate |
|-------|---------------|
| Planning | 1 hour ✅ |
| Core Rename | 2-3 hours |
| File Renames | 1 hour |
| Documentation | 2 hours |
| Infrastructure | 1 hour |
| Testing | 2 hours |
| Repository | 30 minutes |
| **Total** | **8-10 hours** |

---

## 10. Sign-off

- [ ] Plan reviewed
- [ ] Ready to execute
- [ ] Backup created

**Author:** AI Assistant  
**Date:** 2026-01-01  
**Status:** DRAFT
