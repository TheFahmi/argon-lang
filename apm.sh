#!/bin/bash
# ============================================
# ARGON PACKAGE MANAGER (APM) v2.0.0
# Complete dependency management for Argon
# Supports: local, git, and registry deps
# ============================================

set -e

VERSION="2.1.0"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEPS_DIR="deps"
REGISTRY_URL="https://raw.githubusercontent.com/anthropics/argon-packages/main"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# Print functions
print_header() {
    echo -e "${CYAN}========================================"
    echo -e "  ARGON PACKAGE MANAGER v${VERSION}"
    echo -e "========================================${NC}"
    echo ""
}

print_success() { echo -e "${GREEN}✓ $1${NC}"; }
print_error() { echo -e "${RED}✗ $1${NC}"; }
print_info() { echo -e "${BLUE}→ $1${NC}"; }
print_warning() { echo -e "${YELLOW}! $1${NC}"; }
print_step() { echo -e "${MAGENTA}[$1] $2${NC}"; }

# Help message
show_help() {
    print_header
    echo "Usage: apm <command> [options]"
    echo ""
    echo "Commands:"
    echo "  init [name]       Create a new Argon project"
    echo "  build [file]      Build the project"
    echo "  run [file]        Build and run the project"
    echo "  install           Install all dependencies"
    echo "  add <pkg>         Add a dependency"
    echo "  remove <pkg>      Remove a dependency"
    echo "  update            Update all dependencies"
    echo "  list              List installed dependencies"
    echo "  search <query>    Search for packages"
    echo "  publish           Publish package to registry"
    echo "  clean             Remove build artifacts"
    echo "  version           Show version"
    echo "  help              Show this help"
    echo ""
    echo "Dependency types:"
    echo "  apm add pkg-name                    # From registry"
    echo "  apm add ../local-lib --path         # Local path"
    echo "  apm add user/repo --git             # From GitHub"
    echo "  apm add user/repo@v1.0.0 --git      # Specific version"
    echo ""
}

# ============================================
# PROJECT INITIALIZATION
# ============================================

cmd_init() {
    local name="${1:-my-project}"
    
    print_header
    print_info "Creating new Argon project: ${name}"
    
    mkdir -p "${name}/src" "${name}/lib" "${name}/tests" "${name}/deps"
    
    # Create argon.toml
    cat > "${name}/argon.toml" << EOF
[package]
name = "${name}"
version = "0.1.0"
description = "An Argon project"
author = ""
license = "MIT"
repository = ""
keywords = []

[dependencies]
# Registry: package = "version"
# Local:    package = { path = "../path" }
# Git:      package = { git = "https://github.com/user/repo", tag = "v1.0.0" }

[dev-dependencies]

[build]
entry = "src/main.ar"
output = "build"
EOF

    # Create main.ar
    cat > "${name}/src/main.ar" << 'EOF'
// ============================================
// Main entry point
// ============================================

fn main() {
    print("Hello from Argon!");
    return 0;
}
EOF

    # Create lib.ar
    cat > "${name}/lib/lib.ar" << 'EOF'
// ============================================
// Library code
// ============================================

fn greet(name) {
    return "Hello, " + name + "!";
}

fn add(a, b) {
    return a + b;
}
EOF

    # Create test file
    cat > "${name}/tests/test_main.ar" << 'EOF'
// ============================================
// Test suite
// ============================================

import "../lib/lib.ar";

fn test_greet() {
    let result = greet("World");
    if (result == "Hello, World!") {
        print("  [PASS] test_greet");
        return 1;
    }
    print("  [FAIL] test_greet");
    return 0;
}

fn test_add() {
    if (add(2, 3) == 5) {
        print("  [PASS] test_add");
        return 1;
    }
    print("  [FAIL] test_add");
    return 0;
}

fn main() {
    print("Running tests...");
    let passed = 0;
    passed = passed + test_greet();
    passed = passed + test_add();
    print("Passed: " + passed + "/2");
    return 0;
}
EOF

    # Create .gitignore
    cat > "${name}/.gitignore" << 'EOF'
# Build artifacts
*.ll
*.out
*.o
*.exe
build/

# Dependencies (downloaded)
deps/

# Lock file can be committed
# argon.lock

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db
EOF

    # Create README
    cat > "${name}/README.md" << EOF
# ${name}

An Argon project.

## Quick Start

\`\`\`bash
# Install dependencies
apm install

# Build
apm build

# Run
apm run

# Test
apm run tests/test_main.ar
\`\`\`

## Project Structure

\`\`\`
${name}/
├── argon.toml      # Project manifest
├── argon.lock      # Dependency lock file
├── src/
│   └── main.ar     # Entry point
├── lib/
│   └── lib.ar      # Library code
├── tests/
│   └── test_main.ar
└── deps/           # Downloaded dependencies
\`\`\`

## License

MIT
EOF

    print_success "Created project: ${name}"
    echo ""
    echo "Next steps:"
    echo "  cd ${name}"
    echo "  apm install"
    echo "  apm run"
}

# ============================================
# TOML PARSING
# ============================================

parse_toml_value() {
    local file="$1"
    local key="$2"
    grep "^${key}" "$file" 2>/dev/null | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/'
}

# Parse all dependencies with type info
# Output: name|type|value per line
parse_all_dependencies() {
    local file="$1"
    local in_deps=false
    
    while IFS= read -r line || [[ -n "$line" ]]; do
        if [[ "$line" =~ ^\[dependencies\] ]]; then
            in_deps=true
            continue
        fi
        
        if [[ "$line" =~ ^\[.*\] ]]; then
            in_deps=false
            continue
        fi
        
        if $in_deps; then
            [[ "$line" =~ ^[[:space:]]*# ]] && continue
            [[ -z "$line" ]] && continue
            
            # Extract package name
            local pkg_name=$(echo "$line" | sed 's/[[:space:]]*=.*//' | tr -d ' ')
            [[ -z "$pkg_name" ]] && continue
            
            # Path dependency
            if [[ "$line" =~ path[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
                echo "${pkg_name}|path|${BASH_REMATCH[1]}"
            # Git dependency
            elif [[ "$line" =~ git[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
                local git_url="${BASH_REMATCH[1]}"
                local git_tag=""
                if [[ "$line" =~ tag[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
                    git_tag="${BASH_REMATCH[1]}"
                fi
                echo "${pkg_name}|git|${git_url}|${git_tag}"
            # Registry dependency (version string)
            elif [[ "$line" =~ =[[:space:]]*\"([^\"]+)\" ]]; then
                echo "${pkg_name}|registry|${BASH_REMATCH[1]}"
            fi
        fi
    done < "$file"
}

# ============================================
# DEPENDENCY INSTALLATION
# ============================================

install_path_dep() {
    local name="$1"
    local path="$2"
    
    if [[ -d "$path" ]] || [[ -f "$path" ]]; then
        # Create symlink in deps/
        mkdir -p "${DEPS_DIR}"
        local target="${DEPS_DIR}/${name}"
        
        if [[ -L "$target" ]] || [[ -d "$target" ]]; then
            rm -rf "$target"
        fi
        
        # Use relative or absolute path
        if [[ "$path" == /* ]]; then
            ln -sf "$path" "$target" 2>/dev/null || cp -r "$path" "$target"
        else
            ln -sf "../$path" "$target" 2>/dev/null || cp -r "$path" "$target"
        fi
        
        print_success "Linked: ${name} -> ${path}"
        return 0
    else
        print_error "Path not found: ${path}"
        return 1
    fi
}

install_git_dep() {
    local name="$1"
    local url="$2"
    local tag="$3"
    
    mkdir -p "${DEPS_DIR}"
    local target="${DEPS_DIR}/${name}"
    
    # Remove existing
    rm -rf "$target"
    
    print_info "Cloning ${url}..."
    
    if [[ -n "$tag" ]]; then
        git clone --depth 1 --branch "$tag" "$url" "$target" 2>/dev/null
    else
        git clone --depth 1 "$url" "$target" 2>/dev/null
    fi
    
    if [[ $? -eq 0 ]]; then
        # Get commit hash for lock file
        local commit=$(cd "$target" && git rev-parse HEAD 2>/dev/null)
        print_success "Installed: ${name} (${commit:0:8})"
        echo "$commit"
        return 0
    else
        print_error "Failed to clone: ${url}"
        return 1
    fi
}

install_registry_dep() {
    local name="$1"
    local version="$2"
    
    mkdir -p "${DEPS_DIR}"
    local target="${DEPS_DIR}/${name}"
    
    print_info "Fetching ${name}@${version} from registry..."
    
    # Try to fetch registry index
    local registry_index=""
    local registry_file="${SCRIPT_DIR}/registry/index.json"
    
    if [[ -f "$registry_file" ]]; then
        # Use local registry
        registry_index=$(cat "$registry_file")
    else
        # Try to fetch from remote
        registry_index=$(curl -sL "${REGISTRY_URL}/index.json" 2>/dev/null || echo "")
    fi
    
    if [[ -z "$registry_index" ]]; then
        print_warning "Could not fetch registry index"
        # Fallback to GitHub convention
        local url="https://github.com/argon-lang/pkg-${name}"
        return $(install_git_dep_fallback "$name" "$url" "$version")
    fi
    
    # Parse package info from registry (simple JSON parsing)
    local pkg_repo=$(echo "$registry_index" | grep -A20 "\"${name}\"" | grep '"repository"' | head -1 | sed 's/.*: *"\([^"]*\)".*/\1/')
    local pkg_latest=$(echo "$registry_index" | grep -A20 "\"${name}\"" | grep '"latest"' | head -1 | sed 's/.*: *"\([^"]*\)".*/\1/')
    
    if [[ -z "$pkg_repo" ]]; then
        print_warning "Package not found in registry: ${name}"
        print_info "Try: apm search ${name}"
        return 1
    fi
    
    # Resolve version
    if [[ "$version" == "latest" ]] || [[ -z "$version" ]]; then
        version="$pkg_latest"
    fi
    
    print_info "Resolved: ${name}@${version}"
    
    # Clone from repository
    rm -rf "$target"
    
    if git clone --depth 1 --branch "v${version}" "$pkg_repo" "$target" 2>/dev/null; then
        local commit=$(cd "$target" && git rev-parse HEAD 2>/dev/null)
        print_success "Installed: ${name}@${version} (${commit:0:8})"
        return 0
    elif git clone --depth 1 --branch "${version}" "$pkg_repo" "$target" 2>/dev/null; then
        local commit=$(cd "$target" && git rev-parse HEAD 2>/dev/null)
        print_success "Installed: ${name}@${version} (${commit:0:8})"
        return 0
    else
        # Try latest/main branch
        if git clone --depth 1 "$pkg_repo" "$target" 2>/dev/null; then
            local commit=$(cd "$target" && git rev-parse HEAD 2>/dev/null)
            print_success "Installed: ${name}@latest (${commit:0:8})"
            return 0
        fi
        
        print_error "Failed to install: ${name}@${version}"
        return 1
    fi
}

install_git_dep_fallback() {
    local name="$1"
    local url="$2"
    local version="$3"
    
    local target="${DEPS_DIR}/${name}"
    rm -rf "$target"
    
    if [[ -n "$version" ]] && [[ "$version" != "latest" ]]; then
        git clone --depth 1 --branch "v${version}" "$url" "$target" 2>/dev/null || \
        git clone --depth 1 --branch "${version}" "$url" "$target" 2>/dev/null || \
        git clone --depth 1 "$url" "$target" 2>/dev/null
    else
        git clone --depth 1 "$url" "$target" 2>/dev/null
    fi
    
    if [[ -d "$target" ]]; then
        print_success "Installed: ${name}"
        return 0
    fi
    return 1
}

# Fetch and display package info
get_package_info() {
    local name="$1"
    local registry_file="${SCRIPT_DIR}/registry/index.json"
    
    if [[ ! -f "$registry_file" ]]; then
        return 1
    fi
    
    local in_pkg=false
    local pkg_desc=""
    local pkg_repo=""
    local pkg_latest=""
    local pkg_versions=""
    
    while IFS= read -r line; do
        if [[ "$line" =~ \"${name}\"[[:space:]]*: ]]; then
            in_pkg=true
        elif $in_pkg; then
            if [[ "$line" =~ \"description\"[[:space:]]*:[[:space:]]*\"([^\"]+)\" ]]; then
                pkg_desc="${BASH_REMATCH[1]}"
            elif [[ "$line" =~ \"repository\"[[:space:]]*:[[:space:]]*\"([^\"]+)\" ]]; then
                pkg_repo="${BASH_REMATCH[1]}"
            elif [[ "$line" =~ \"latest\"[[:space:]]*:[[:space:]]*\"([^\"]+)\" ]]; then
                pkg_latest="${BASH_REMATCH[1]}"
            elif [[ "$line" =~ ^[[:space:]]*\} ]]; then
                break
            fi
        fi
    done < "$registry_file"
    
    if [[ -n "$pkg_desc" ]]; then
        echo "name:${name}"
        echo "description:${pkg_desc}"
        echo "repository:${pkg_repo}"
        echo "latest:${pkg_latest}"
        return 0
    fi
    return 1
}

# ============================================
# LOCK FILE MANAGEMENT
# ============================================

generate_lockfile() {
    local lockfile="argon.lock"
    
    print_info "Generating ${lockfile}..."
    
    cat > "$lockfile" << EOF
# This file is auto-generated by APM
# Do not edit manually
# Generated: $(date -u +"%Y-%m-%dT%H:%M:%SZ")

[metadata]
apm_version = "${VERSION}"

EOF
    
    # Add each installed dependency
    if [[ -d "${DEPS_DIR}" ]]; then
        for dep_dir in "${DEPS_DIR}"/*; do
            if [[ -d "$dep_dir" ]]; then
                local dep_name=$(basename "$dep_dir")
                local dep_version=""
                local dep_source=""
                local dep_commit=""
                
                # Check if it's a git repo
                if [[ -d "${dep_dir}/.git" ]]; then
                    dep_commit=$(cd "$dep_dir" && git rev-parse HEAD 2>/dev/null)
                    dep_source=$(cd "$dep_dir" && git remote get-url origin 2>/dev/null)
                    dep_version=$(cd "$dep_dir" && git describe --tags --always 2>/dev/null)
                else
                    dep_source="local"
                    dep_version="0.0.0"
                fi
                
                cat >> "$lockfile" << EOF
[[package]]
name = "${dep_name}"
version = "${dep_version}"
source = "${dep_source}"
commit = "${dep_commit}"

EOF
            fi
        done
    fi
    
    print_success "Generated: ${lockfile}"
}

read_lockfile() {
    local lockfile="argon.lock"
    
    if [[ ! -f "$lockfile" ]]; then
        return 1
    fi
    
    # Parse lock file and output name|commit pairs
    local current_name=""
    local current_commit=""
    
    while IFS= read -r line; do
        if [[ "$line" =~ ^name[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
            current_name="${BASH_REMATCH[1]}"
        elif [[ "$line" =~ ^commit[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
            current_commit="${BASH_REMATCH[1]}"
            if [[ -n "$current_name" ]]; then
                echo "${current_name}|${current_commit}"
            fi
            current_name=""
            current_commit=""
        fi
    done < "$lockfile"
}

# ============================================
# COMMANDS
# ============================================

cmd_install() {
    print_header
    
    if [[ ! -f "argon.toml" ]]; then
        print_error "No argon.toml found. Run 'apm init' first."
        exit 1
    fi
    
    local name=$(parse_toml_value "argon.toml" "name")
    print_info "Installing dependencies for ${name}..."
    echo ""
    
    local count=0
    local failed=0
    
    # Read all dependencies
    while IFS='|' read -r pkg_name dep_type dep_value dep_extra; do
        [[ -z "$pkg_name" ]] && continue
        
        print_step "$((count+1))" "Installing ${pkg_name}..."
        
        case "$dep_type" in
            path)
                if install_path_dep "$pkg_name" "$dep_value"; then
                    count=$((count + 1))
                else
                    failed=$((failed + 1))
                fi
                ;;
            git)
                if install_git_dep "$pkg_name" "$dep_value" "$dep_extra"; then
                    count=$((count + 1))
                else
                    failed=$((failed + 1))
                fi
                ;;
            registry)
                if install_registry_dep "$pkg_name" "$dep_value"; then
                    count=$((count + 1))
                else
                    failed=$((failed + 1))
                fi
                ;;
        esac
    done < <(parse_all_dependencies "argon.toml")
    
    echo ""
    
    if [[ $count -gt 0 ]]; then
        generate_lockfile
    fi
    
    echo ""
    if [[ $failed -eq 0 ]]; then
        print_success "Installed ${count} dependencies"
    else
        print_warning "Installed ${count} dependencies, ${failed} failed"
    fi
}

cmd_add() {
    local pkg="$1"
    local type_flag="${2:---registry}"
    
    if [[ -z "$pkg" ]]; then
        print_error "Usage: apm add <package> [--path|--git|--registry]"
        exit 1
    fi
    
    print_header
    
    if [[ ! -f "argon.toml" ]]; then
        print_error "No argon.toml found. Run 'apm init' first."
        exit 1
    fi
    
    local dep_line=""
    local dep_name=""
    
    case "$type_flag" in
        --path)
            dep_name=$(basename "$pkg")
            dep_line="${dep_name} = { path = \"${pkg}\" }"
            ;;
        --git)
            # Parse user/repo or full URL
            if [[ "$pkg" =~ ^https?:// ]]; then
                dep_name=$(basename "$pkg" .git)
                dep_line="${dep_name} = { git = \"${pkg}\" }"
            elif [[ "$pkg" =~ @ ]]; then
                # user/repo@version
                local repo_part="${pkg%@*}"
                local version_part="${pkg#*@}"
                dep_name=$(basename "$repo_part")
                dep_line="${dep_name} = { git = \"https://github.com/${repo_part}\", tag = \"${version_part}\" }"
            else
                dep_name=$(basename "$pkg")
                dep_line="${dep_name} = { git = \"https://github.com/${pkg}\" }"
            fi
            ;;
        --registry|*)
            # Package name with optional version
            if [[ "$pkg" =~ @ ]]; then
                dep_name="${pkg%@*}"
                local version="${pkg#*@}"
                dep_line="${dep_name} = \"${version}\""
            else
                dep_name="$pkg"
                dep_line="${dep_name} = \"latest\""
            fi
            ;;
    esac
    
    print_info "Adding ${dep_name}..."
    
    # Check if already exists
    if grep -q "^${dep_name}[[:space:]]*=" argon.toml 2>/dev/null; then
        print_warning "Dependency already exists: ${dep_name}"
        print_info "Use 'apm update' to update or remove first"
        exit 1
    fi
    
    # Add to argon.toml
    if grep -q "\[dependencies\]" argon.toml; then
        # Add after [dependencies] section
        sed -i "/\[dependencies\]/a ${dep_line}" argon.toml
    else
        # Create section
        echo "" >> argon.toml
        echo "[dependencies]" >> argon.toml
        echo "$dep_line" >> argon.toml
    fi
    
    print_success "Added: ${dep_line}"
    echo ""
    print_info "Run 'apm install' to download"
}

cmd_remove() {
    local pkg="$1"
    
    if [[ -z "$pkg" ]]; then
        print_error "Usage: apm remove <package>"
        exit 1
    fi
    
    print_header
    print_info "Removing ${pkg}..."
    
    # Remove from argon.toml
    sed -i "/^${pkg}[[:space:]]*=/d" argon.toml 2>/dev/null
    
    # Remove from deps/
    rm -rf "${DEPS_DIR}/${pkg}"
    
    # Regenerate lock file
    if [[ -f "argon.lock" ]]; then
        generate_lockfile
    fi
    
    print_success "Removed: ${pkg}"
}

cmd_update() {
    print_header
    print_info "Updating all dependencies..."
    
    # Remove deps and reinstall
    rm -rf "${DEPS_DIR}"
    cmd_install
}

cmd_list() {
    print_header
    print_info "Installed dependencies:"
    echo ""
    
    if [[ ! -d "${DEPS_DIR}" ]]; then
        echo "  (none)"
        return
    fi
    
    for dep_dir in "${DEPS_DIR}"/*; do
        if [[ -d "$dep_dir" ]]; then
            local dep_name=$(basename "$dep_dir")
            local dep_info=""
            
            if [[ -d "${dep_dir}/.git" ]]; then
                dep_info=$(cd "$dep_dir" && git describe --tags --always 2>/dev/null)
            else
                dep_info="local"
            fi
            
            echo "  ${dep_name} (${dep_info})"
        fi
    done
}

cmd_search() {
    local query="$1"
    
    print_header
    
    if [[ -z "$query" ]]; then
        print_info "Available packages in registry:"
        echo ""
    else
        print_info "Searching for: ${query}"
        echo ""
    fi
    
    local registry_file="${SCRIPT_DIR}/registry/index.json"
    
    if [[ -f "$registry_file" ]]; then
        local found=0
        
        # Simple approach: find package names by looking for pattern
        # "package-name": {
        #   "description": "...",
        # (after the "packages": { line)
        
        # Get all package definitions
        local packages=$(grep -E '^\s*"[a-z][a-z0-9-]*"\s*:\s*\{' "$registry_file" | \
                        grep -v '"packages"' | grep -v '"versions"' | grep -v '"dependencies"' | \
                        sed 's/.*"\([^"]*\)".*/\1/')
        
        for pkg in $packages; do
            # Get description and latest for this package
            local pkg_section=$(sed -n "/\"${pkg}\"/,/\"latest\"/p" "$registry_file" | head -20)
            local desc=$(echo "$pkg_section" | grep '"description"' | head -1 | sed 's/.*"\([^"]*\)"[^"]*$/\1/')
            local latest=$(echo "$pkg_section" | grep '"latest"' | head -1 | sed 's/.*"\([^"]*\)"[^"]*$/\1/')
            
            # Filter by query if provided
            if [[ -z "$query" ]] || [[ "$pkg" == *"$query"* ]] || [[ "$desc" == *"$query"* ]]; then
                echo -e "  ${GREEN}${pkg}${NC} (${latest})"
                echo "    ${desc}"
                echo ""
                found=$((found + 1))
            fi
        done
        
        if [[ $found -eq 0 ]]; then
            if [[ -n "$query" ]]; then
                print_warning "No packages found matching: ${query}"
            else
                print_warning "No packages in registry"
            fi
        else
            echo "Found ${found} package(s)"
        fi
        
        echo ""
        echo "Install a package:"
        echo "  apm add <package-name>"
    else
        print_warning "Local registry not found"
        echo ""
        echo "Search on GitHub:"
        echo "  https://github.com/search?q=argon+${query}&type=repositories"
        echo ""
        echo "To add a GitHub package:"
        echo "  apm add user/repo --git"
    fi
}

# Show package info
cmd_info() {
    local pkg="$1"
    
    print_header
    
    if [[ -z "$pkg" ]]; then
        print_error "Usage: apm info <package>"
        exit 1
    fi
    
    print_info "Package: ${pkg}"
    echo ""
    
    local info=$(get_package_info "$pkg")
    
    if [[ -n "$info" ]]; then
        while IFS=':' read -r key value; do
            case "$key" in
                name) echo "  Name:        ${value}" ;;
                description) echo "  Description: ${value}" ;;
                repository) echo "  Repository:  ${value}" ;;
                latest) echo "  Latest:      ${value}" ;;
            esac
        done <<< "$info"
        
        echo ""
        echo "Install:"
        echo "  apm add ${pkg}"
    else
        print_warning "Package not found: ${pkg}"
        echo ""
        echo "Try searching:"
        echo "  apm search ${pkg}"
    fi
}

cmd_publish() {
    print_header
    
    if [[ ! -f "argon.toml" ]]; then
        print_error "No argon.toml found."
        exit 1
    fi
    
    local name=$(parse_toml_value "argon.toml" "name")
    local version=$(parse_toml_value "argon.toml" "version")
    
    print_info "Publishing ${name}@${version}..."
    echo ""
    
    # Check required fields
    local repo=$(parse_toml_value "argon.toml" "repository")
    if [[ -z "$repo" ]]; then
        print_error "Missing 'repository' in argon.toml"
        echo "Add: repository = \"https://github.com/user/repo\""
        exit 1
    fi
    
    # Create a git tag
    print_step "1" "Creating tag v${version}..."
    
    if git tag "v${version}" 2>/dev/null; then
        print_success "Created tag: v${version}"
    else
        print_warning "Tag already exists or git error"
    fi
    
    print_step "2" "Pushing to repository..."
    
    git push origin "v${version}" 2>/dev/null && \
        print_success "Pushed tag to origin" || \
        print_warning "Push failed - check git remote"
    
    echo ""
    print_success "Published ${name}@${version}"
    echo ""
    echo "Others can install with:"
    echo "  apm add ${repo#https://github.com/}@v${version} --git"
}

cmd_build() {
    local target="${1:-src/main.ar}"
    
    print_header
    
    if [[ ! -f "argon.toml" ]]; then
        print_error "No argon.toml found. Run 'apm init' first."
        exit 1
    fi
    
    local name=$(parse_toml_value "argon.toml" "name")
    local version=$(parse_toml_value "argon.toml" "version")
    
    print_info "Building ${name} v${version}"
    
    # Check if deps need installing
    local dep_count=$(parse_all_dependencies "argon.toml" | wc -l)
    if [[ $dep_count -gt 0 ]] && [[ ! -d "${DEPS_DIR}" ]]; then
        print_warning "Dependencies not installed. Running 'apm install'..."
        cmd_install
        echo ""
    fi
    
    print_info "Compiling ${target}..."
    
    if command -v docker &> /dev/null; then
        local mount_path
        if [[ "$(uname -s)" == *"MINGW"* ]] || [[ "$(uname -s)" == *"MSYS"* ]]; then
            mount_path="$(pwd -W)"
        else
            mount_path="$(pwd)"
        fi
        
        docker run --rm -v "${mount_path}:/src" -w //src argon-toolchain bash -c "
            argonc ${target} && \
            clang++ -O2 -Wno-override-module ${target}.ll /usr/lib/libruntime_argon.a -o ${target%.ar}.out -lpthread -ldl
        "
        
        if [[ -f "${target%.ar}.out" ]]; then
            print_success "Built: ${target%.ar}.out"
        else
            print_error "Build failed"
            exit 1
        fi
    else
        print_error "Docker not found. Please install Docker."
        exit 1
    fi
}

cmd_run() {
    local target="${1:-src/main.ar}"
    local output="${target%.ar}.out"
    
    cmd_build "$target"
    
    echo ""
    print_info "Running ${output}..."
    echo "----------------------------------------"
    
    local mount_path
    if [[ "$(uname -s)" == *"MINGW"* ]] || [[ "$(uname -s)" == *"MSYS"* ]]; then
        mount_path="$(pwd -W)"
        docker run --rm -v "${mount_path}:/src" -w //src argon-toolchain ./${output}
    else
        "./${output}"
    fi
}

cmd_clean() {
    print_header
    print_info "Cleaning build artifacts..."
    
    local count=0
    
    for ext in ll out o; do
        for f in $(find . -name "*.${ext}" -type f 2>/dev/null); do
            rm -f "$f"
            ((count++))
        done
    done
    
    print_success "Removed ${count} files"
}

cmd_version() {
    echo "Argon Package Manager v${VERSION}"
}

# ============================================
# MAIN
# ============================================

main() {
    local cmd="${1:-help}"
    shift || true
    
    case "$cmd" in
        init)       cmd_init "$@" ;;
        build|b)    cmd_build "$@" ;;
        run|r)      cmd_run "$@" ;;
        install|i)  cmd_install "$@" ;;
        add|a)      cmd_add "$@" ;;
        remove|rm)  cmd_remove "$@" ;;
        update|up)  cmd_update "$@" ;;
        list|ls)    cmd_list "$@" ;;
        search|s)   cmd_search "$@" ;;
        info)       cmd_info "$@" ;;
        publish)    cmd_publish "$@" ;;
        clean)      cmd_clean "$@" ;;
        version|--version|-v) cmd_version ;;
        help|--help|-h) show_help ;;
        *)
            print_error "Unknown command: ${cmd}"
            echo "Run 'apm help' for usage."
            exit 1
            ;;
    esac
}

main "$@"
