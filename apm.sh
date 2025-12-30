#!/bin/bash
# ============================================
# ARGON PACKAGE MANAGER (APM) v1.0.0
# Dependency management for Argon projects
# ============================================

set -e

VERSION="1.0.0"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Print functions
print_header() {
    echo -e "${CYAN}========================================"
    echo -e "  ARGON PACKAGE MANAGER v${VERSION}"
    echo -e "========================================${NC}"
    echo ""
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${BLUE}→ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}! $1${NC}"
}

# Help message
show_help() {
    print_header
    echo "Usage: apm <command> [options]"
    echo ""
    echo "Commands:"
    echo "  init [name]     Create a new Argon project"
    echo "  build           Build the current project"
    echo "  run             Build and run the project"
    echo "  add <pkg>       Add a dependency"
    echo "  remove <pkg>    Remove a dependency"
    echo "  install         Install all dependencies"
    echo "  clean           Remove build artifacts"
    echo "  version         Show version"
    echo "  help            Show this help"
    echo ""
    echo "Examples:"
    echo "  apm init my-project"
    echo "  apm build"
    echo "  apm run"
    echo "  apm add ../my-lib --path"
    echo ""
}

# Initialize new project
cmd_init() {
    local name="${1:-my-project}"
    
    print_header
    print_info "Creating new Argon project: ${name}"
    
    # Create directory structure
    mkdir -p "${name}/src"
    mkdir -p "${name}/lib"
    mkdir -p "${name}/tests"
    
    # Create argon.toml
    cat > "${name}/argon.toml" << EOF
[package]
name = "${name}"
version = "0.1.0"
description = "An Argon project"
author = ""
license = "MIT"

[dependencies]
# Add dependencies here
# example = "1.0.0"
# local-lib = { path = "../local-lib" }

[dev-dependencies]
# test-framework = "0.1.0"
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
        return true;
    }
    print("  [FAIL] test_greet");
    return false;
}

fn main() {
    print("Running tests...");
    test_greet();
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

# Dependencies
deps/

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

## Build

\`\`\`bash
apm build
\`\`\`

## Run

\`\`\`bash
apm run
\`\`\`

## Test

\`\`\`bash
apm build tests/test_main.ar
./tests/test_main.ar.out
\`\`\`
EOF

    print_success "Created project: ${name}"
    echo ""
    echo "Next steps:"
    echo "  cd ${name}"
    echo "  apm build"
    echo "  apm run"
}

# Parse argon.toml
parse_toml() {
    local file="$1"
    local key="$2"
    
    if [[ ! -f "$file" ]]; then
        return 1
    fi
    
    grep "^${key}" "$file" | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/'
}

# Parse dependencies from argon.toml
parse_dependencies() {
    local file="$1"
    local in_deps=false
    local deps=""
    
    while IFS= read -r line; do
        # Check for [dependencies] section
        if [[ "$line" =~ ^\[dependencies\] ]]; then
            in_deps=true
            continue
        fi
        
        # Check for new section (end of dependencies)
        if [[ "$line" =~ ^\[.*\] ]]; then
            in_deps=false
            continue
        fi
        
        # Parse dependency line
        if $in_deps && [[ "$line" =~ ^[a-zA-Z] ]]; then
            # Skip comments
            [[ "$line" =~ ^# ]] && continue
            
            # Parse path dependency: name = { path = "..." }
            if [[ "$line" =~ path\ *=\ *\"([^\"]+)\" ]]; then
                local dep_path="${BASH_REMATCH[1]}"
                deps="${deps}${dep_path}\n"
            fi
        fi
    done < "$file"
    
    echo -e "$deps"
}

# Build project
cmd_build() {
    local target="${1:-src/main.ar}"
    
    print_header
    
    # Check for argon.toml
    if [[ ! -f "argon.toml" ]]; then
        print_error "No argon.toml found. Run 'apm init' first."
        exit 1
    fi
    
    local name=$(parse_toml "argon.toml" "name")
    local version=$(parse_toml "argon.toml" "version")
    
    print_info "Building ${name} v${version}"
    echo ""
    
    # Parse dependencies
    local deps=$(parse_dependencies "argon.toml")
    
    # Build import list
    local imports=""
    if [[ -n "$deps" ]]; then
        print_info "Resolving dependencies..."
        while IFS= read -r dep; do
            [[ -z "$dep" ]] && continue
            if [[ -d "$dep" ]]; then
                # Find .ar files in dependency
                for ar_file in "$dep"/*.ar; do
                    if [[ -f "$ar_file" ]]; then
                        imports="${imports}import \"${ar_file}\";\n"
                        print_success "Found: ${ar_file}"
                    fi
                done
            elif [[ -f "$dep" ]]; then
                imports="${imports}import \"${dep}\";\n"
                print_success "Found: ${dep}"
            else
                print_warning "Dependency not found: ${dep}"
            fi
        done <<< "$deps"
        echo ""
    fi
    
    # Compile using Docker
    print_info "Compiling ${target}..."
    
    if command -v docker &> /dev/null; then
        # Get proper path for Docker mount (Windows compatibility)
        local mount_path
        if [[ "$(uname -s)" == *"MINGW"* ]] || [[ "$(uname -s)" == *"MSYS"* ]]; then
            # Git Bash on Windows - use pwd -W
            mount_path="$(pwd -W)"
        else
            mount_path="$(pwd)"
        fi
        
        # Use Docker
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

# Run project
cmd_run() {
    local target="${1:-src/main.ar}"
    local output="${target%.ar}.out"
    
    # Build first
    cmd_build "$target"
    
    echo ""
    print_info "Running ${output}..."
    echo "----------------------------------------"
    
    # Get proper path for Docker mount (Windows compatibility)
    local mount_path
    if [[ "$(uname -s)" == *"MINGW"* ]] || [[ "$(uname -s)" == *"MSYS"* ]]; then
        # Git Bash on Windows - use pwd -W and run in Docker
        mount_path="$(pwd -W)"
        docker run --rm -v "${mount_path}:/src" -w //src argon-toolchain ./${output}
    else
        # Linux/Mac - can run directly
        "./${output}"
    fi
}

# Add dependency
cmd_add() {
    local pkg="$1"
    local type="${2:---path}"
    
    if [[ -z "$pkg" ]]; then
        print_error "Usage: apm add <package> [--path|--git]"
        exit 1
    fi
    
    print_header
    
    if [[ ! -f "argon.toml" ]]; then
        print_error "No argon.toml found. Run 'apm init' first."
        exit 1
    fi
    
    local dep_name=$(basename "$pkg")
    
    if [[ "$type" == "--path" ]]; then
        # Add path dependency
        print_info "Adding path dependency: ${dep_name}"
        
        # Check if dependency section exists
        if grep -q "\[dependencies\]" argon.toml; then
            # Add after [dependencies]
            sed -i "/\[dependencies\]/a ${dep_name} = { path = \"${pkg}\" }" argon.toml
        else
            # Add section
            echo "" >> argon.toml
            echo "[dependencies]" >> argon.toml
            echo "${dep_name} = { path = \"${pkg}\" }" >> argon.toml
        fi
        
        print_success "Added: ${dep_name} = { path = \"${pkg}\" }"
    else
        print_warning "Only --path dependencies supported in v1.0"
    fi
}

# Remove dependency
cmd_remove() {
    local pkg="$1"
    
    if [[ -z "$pkg" ]]; then
        print_error "Usage: apm remove <package>"
        exit 1
    fi
    
    print_header
    
    if [[ ! -f "argon.toml" ]]; then
        print_error "No argon.toml found."
        exit 1
    fi
    
    print_info "Removing dependency: ${pkg}"
    
    # Remove line containing the package
    sed -i "/${pkg}/d" argon.toml
    
    print_success "Removed: ${pkg}"
}

# Install dependencies
cmd_install() {
    print_header
    
    if [[ ! -f "argon.toml" ]]; then
        print_error "No argon.toml found. Run 'apm init' first."
        exit 1
    fi
    
    print_info "Installing dependencies..."
    
    local deps=$(parse_dependencies "argon.toml")
    local count=0
    
    while IFS= read -r dep; do
        [[ -z "$dep" ]] && continue
        
        if [[ -d "$dep" ]] || [[ -f "$dep" ]]; then
            print_success "Found: ${dep}"
            ((count++))
        else
            print_warning "Not found: ${dep}"
        fi
    done <<< "$deps"
    
    echo ""
    print_success "Installed ${count} dependencies"
}

# Clean build artifacts
cmd_clean() {
    print_header
    print_info "Cleaning build artifacts..."
    
    local count=0
    
    # Remove .ll files
    for f in $(find . -name "*.ll" -type f 2>/dev/null); do
        rm -f "$f"
        ((count++))
    done
    
    # Remove .out files
    for f in $(find . -name "*.out" -type f 2>/dev/null); do
        rm -f "$f"
        ((count++))
    done
    
    # Remove .o files
    for f in $(find . -name "*.o" -type f 2>/dev/null); do
        rm -f "$f"
        ((count++))
    done
    
    print_success "Removed ${count} files"
}

# Show version
cmd_version() {
    echo "Argon Package Manager v${VERSION}"
}

# Main
main() {
    local cmd="${1:-help}"
    shift || true
    
    case "$cmd" in
        init)
            cmd_init "$@"
            ;;
        build)
            cmd_build "$@"
            ;;
        run)
            cmd_run "$@"
            ;;
        add)
            cmd_add "$@"
            ;;
        remove)
            cmd_remove "$@"
            ;;
        install)
            cmd_install "$@"
            ;;
        clean)
            cmd_clean "$@"
            ;;
        version|--version|-v)
            cmd_version
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "Unknown command: ${cmd}"
            echo "Run 'apm help' for usage."
            exit 1
            ;;
    esac
}

main "$@"
