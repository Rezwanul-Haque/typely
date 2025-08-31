#!/bin/bash
set -e

# Typely Installation Script
# Usage: curl -fsSL https://typely.sh/install | sh

REPO="typely/typely"
BINARY_NAME="typely"
CLI_BINARY_NAME="typely-cli"
INSTALL_DIR="/usr/local/bin"
VERSION="latest"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        warn "Running as root. Typely will be installed system-wide."
        INSTALL_DIR="/usr/local/bin"
    else
        log "Running as regular user. You may need sudo privileges for installation."
    fi
}

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s)
    local arch=$(uname -m)
    
    case "$os" in
        Linux*)
            OS="linux"
            ;;
        Darwin*)
            OS="macos"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            OS="windows"
            ;;
        *)
            error "Unsupported operating system: $os"
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        arm64|aarch64)
            ARCH="arm64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac
    
    log "Detected platform: $OS-$ARCH"
}

# Check dependencies
check_dependencies() {
    log "Checking dependencies..."
    
    # Check for required tools
    for cmd in curl tar; do
        if ! command -v $cmd &> /dev/null; then
            error "$cmd is required but not installed. Please install $cmd and try again."
        fi
    done
    
    # Check for SQLite
    if ! command -v sqlite3 &> /dev/null; then
        warn "SQLite3 not found. Installing..."
        install_sqlite
    fi
    
    # Check for system libraries on Linux
    if [[ "$OS" == "linux" ]]; then
        check_linux_dependencies
    fi
}

install_sqlite() {
    case "$OS" in
        linux)
            if command -v apt-get &> /dev/null; then
                sudo apt-get update && sudo apt-get install -y sqlite3 libsqlite3-dev
            elif command -v yum &> /dev/null; then
                sudo yum install -y sqlite sqlite-devel
            elif command -v dnf &> /dev/null; then
                sudo dnf install -y sqlite sqlite-devel
            elif command -v pacman &> /dev/null; then
                sudo pacman -S sqlite
            else
                error "Unable to install SQLite automatically. Please install sqlite3 manually."
            fi
            ;;
        macos)
            if command -v brew &> /dev/null; then
                brew install sqlite
            else
                error "Homebrew not found. Please install SQLite manually or install Homebrew."
            fi
            ;;
    esac
}

check_linux_dependencies() {
    log "Checking Linux system dependencies..."
    
    # Check for X11 libraries
    local missing_libs=()
    
    if ! ldconfig -p | grep -q libX11; then
        missing_libs+="libx11-dev "
    fi
    
    if ! ldconfig -p | grep -q libXi; then
        missing_libs+="libxi-dev "
    fi
    
    if ! ldconfig -p | grep -q libXtst; then
        missing_libs+="libxtst-dev "
    fi
    
    if [[ ${#missing_libs[@]} -gt 0 ]]; then
        warn "Missing X11 libraries. Installing: ${missing_libs[*]}"
        
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y ${missing_libs[*]}
        elif command -v yum &> /dev/null; then
            sudo yum install -y libX11-devel libXi-devel libXtst-devel
        elif command -v dnf &> /dev/null; then
            sudo dnf install -y libX11-devel libXi-devel libXtst-devel
        else
            warn "Could not install X11 libraries automatically. Please install manually:"
            warn "  Ubuntu/Debian: sudo apt-get install libx11-dev libxi-dev libxtst-dev"
            warn "  CentOS/RHEL: sudo yum install libX11-devel libXi-devel libXtst-devel"
            warn "  Fedora: sudo dnf install libX11-devel libXi-devel libXtst-devel"
        fi
    fi
}

# Get the latest release information
get_release_info() {
    log "Fetching release information..."
    
    if [[ "$VERSION" == "latest" ]]; then
        RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"
    else
        RELEASE_URL="https://api.github.com/repos/$REPO/releases/tags/$VERSION"
    fi
    
    # Since we don't have actual releases yet, we'll build from source
    log "No pre-built releases available yet. Building from source..."
    BUILD_FROM_SOURCE=true
}

# Build from source
build_from_source() {
    log "Building Typely from source..."
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        log "Rust not found. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    cd "$TEMP_DIR"
    
    log "Cloning repository..."
    git clone https://github.com/$REPO.git
    cd typely
    
    log "Building release version..."
    cargo build --release --features system-integration
    
    log "Installing binaries..."
    if [[ $EUID -eq 0 ]]; then
        cp target/release/typely "$INSTALL_DIR/"
        cp target/release/typely-cli "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/typely"
        chmod +x "$INSTALL_DIR/typely-cli"
    else
        sudo cp target/release/typely "$INSTALL_DIR/"
        sudo cp target/release/typely-cli "$INSTALL_DIR/"
        sudo chmod +x "$INSTALL_DIR/typely"
        sudo chmod +x "$INSTALL_DIR/typely-cli"
    fi
    
    # Clean up
    cd /
    rm -rf "$TEMP_DIR"
}

# Create desktop entry (Linux)
create_desktop_entry() {
    if [[ "$OS" == "linux" ]] && [[ -d "$HOME/.local/share/applications" ]]; then
        log "Creating desktop entry..."
        cat > "$HOME/.local/share/applications/typely.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Typely
Comment=Text expansion made easy
Exec=$INSTALL_DIR/typely
Icon=typely
Terminal=false
Categories=Utility;Productivity;
StartupNotify=false
NoDisplay=true
X-GNOME-Autostart-enabled=true
EOF
        
        # Make it executable
        chmod +x "$HOME/.local/share/applications/typely.desktop"
        
        # Add to autostart
        mkdir -p "$HOME/.config/autostart"
        cp "$HOME/.local/share/applications/typely.desktop" "$HOME/.config/autostart/"
        
        success "Desktop entry created and autostart enabled"
    fi
}

# Setup shell completions
setup_completions() {
    log "Setting up shell completions..."
    
    # Create completions directory
    local comp_dir="$HOME/.local/share/typely/completions"
    mkdir -p "$comp_dir"
    
    # Generate completions for different shells
    if command -v typely-cli &> /dev/null; then
        typely-cli completions bash > "$comp_dir/typely.bash" 2>/dev/null || true
        typely-cli completions zsh > "$comp_dir/_typely" 2>/dev/null || true
        typely-cli completions fish > "$comp_dir/typely.fish" 2>/dev/null || true
    fi
    
    # Add to shell configuration
    local shell_name=$(basename "$SHELL")
    case "$shell_name" in
        bash)
            if [[ -f "$HOME/.bashrc" ]] && ! grep -q "typely.bash" "$HOME/.bashrc"; then
                echo "source '$comp_dir/typely.bash'" >> "$HOME/.bashrc"
            fi
            ;;
        zsh)
            if [[ -f "$HOME/.zshrc" ]] && ! grep -q "_typely" "$HOME/.zshrc"; then
                echo "fpath=('$comp_dir' \$fpath)" >> "$HOME/.zshrc"
                echo "autoload -U compinit && compinit" >> "$HOME/.zshrc"
            fi
            ;;
        fish)
            local fish_comp_dir="$HOME/.config/fish/completions"
            mkdir -p "$fish_comp_dir"
            cp "$comp_dir/typely.fish" "$fish_comp_dir/" 2>/dev/null || true
            ;;
    esac
}

# Verify installation
verify_installation() {
    log "Verifying installation..."
    
    if ! command -v typely &> /dev/null; then
        error "Installation failed: typely command not found"
    fi
    
    if ! command -v typely-cli &> /dev/null; then
        error "Installation failed: typely-cli command not found"
    fi
    
    # Test the CLI
    if ! typely-cli --version &> /dev/null; then
        warn "CLI installation may have issues, but binaries are installed"
    fi
    
    success "Installation completed successfully!"
    
    # Show version info
    log "Installed versions:"
    echo "  typely: $(typely --version 2>/dev/null || echo 'unknown')"
    echo "  typely-cli: $(typely-cli --version 2>/dev/null || echo 'unknown')"
}

# Main installation function
main() {
    log "Starting Typely installation..."
    
    check_root
    detect_platform
    check_dependencies
    get_release_info
    
    if [[ "$BUILD_FROM_SOURCE" == true ]]; then
        build_from_source
    else
        error "Pre-built binaries not available yet. Please build from source."
    fi
    
    create_desktop_entry
    setup_completions
    verify_installation
    
    success "🎉 Typely has been successfully installed!"
    echo ""
    echo "Next steps:"
    echo "  1. Run 'typely' to start the background service"
    echo "  2. Use 'typely-cli add \"::hello\" \"Hello, World!\"' to create your first snippet"
    echo "  3. Type '::hello' in any application to test text expansion"
    echo ""
    echo "For help and documentation, visit: https://github.com/typely/typely"
    echo "Report issues at: https://github.com/typely/typely/issues"
}

# Run main function
main "$@"