#!/bin/bash
set -e

# Typely Uninstallation Script

BINARY_NAME="typely"
CLI_BINARY_NAME="typely-cli"
INSTALL_DIR="/usr/local/bin"

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
    if [[ $EUID -ne 0 ]] && [[ -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
        warn "You may need sudo privileges to remove system-wide installation."
    fi
}

# Stop running processes
stop_typely() {
    log "Stopping Typely processes..."
    
    # Find and kill typely processes
    if pgrep -f "typely" > /dev/null; then
        log "Found running Typely processes. Stopping..."
        pkill -f "typely" || true
        sleep 2
        
        # Force kill if still running
        if pgrep -f "typely" > /dev/null; then
            warn "Force stopping Typely processes..."
            pkill -9 -f "typely" || true
        fi
    fi
}

# Remove binaries
remove_binaries() {
    log "Removing binaries..."
    
    local removed=false
    
    # Remove main binary
    if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
        if [[ $EUID -eq 0 ]]; then
            rm -f "$INSTALL_DIR/$BINARY_NAME"
        else
            sudo rm -f "$INSTALL_DIR/$BINARY_NAME"
        fi
        success "Removed $INSTALL_DIR/$BINARY_NAME"
        removed=true
    fi
    
    # Remove CLI binary
    if [[ -f "$INSTALL_DIR/$CLI_BINARY_NAME" ]]; then
        if [[ $EUID -eq 0 ]]; then
            rm -f "$INSTALL_DIR/$CLI_BINARY_NAME"
        else
            sudo rm -f "$INSTALL_DIR/$CLI_BINARY_NAME"
        fi
        success "Removed $INSTALL_DIR/$CLI_BINARY_NAME"
        removed=true
    fi
    
    if [[ "$removed" == false ]]; then
        warn "No Typely binaries found in $INSTALL_DIR"
    fi
}

# Remove user data
remove_user_data() {
    local remove_data=false
    
    echo ""
    read -p "Do you want to remove user data and snippets? [y/N]: " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        remove_data=true
    fi
    
    if [[ "$remove_data" == true ]]; then
        log "Removing user data..."
        
        # Remove data directories
        local data_dirs=(
            "$HOME/.local/share/typely"
            "$HOME/Library/Application Support/typely"
            "$HOME/AppData/Roaming/typely"
        )
        
        for dir in "${data_dirs[@]}"; do
            if [[ -d "$dir" ]]; then
                rm -rf "$dir"
                success "Removed $dir"
            fi
        done
        
        # Remove configuration
        local config_dirs=(
            "$HOME/.config/typely"
        )
        
        for dir in "${config_dirs[@]}"; do
            if [[ -d "$dir" ]]; then
                rm -rf "$dir"
                success "Removed $dir"
            fi
        done
    else
        log "User data preserved. You can manually remove:"
        log "  - ~/.local/share/typely (Linux)"
        log "  - ~/Library/Application Support/typely (macOS)"
        log "  - ~/.config/typely (configuration)"
    fi
}

# Remove desktop integration
remove_desktop_integration() {
    log "Removing desktop integration..."
    
    # Remove desktop entry
    local desktop_file="$HOME/.local/share/applications/typely.desktop"
    if [[ -f "$desktop_file" ]]; then
        rm -f "$desktop_file"
        success "Removed desktop entry"
    fi
    
    # Remove autostart entry
    local autostart_file="$HOME/.config/autostart/typely.desktop"
    if [[ -f "$autostart_file" ]]; then
        rm -f "$autostart_file"
        success "Removed autostart entry"
    fi
    
    # Update desktop database
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
    fi
}

# Remove shell completions
remove_completions() {
    log "Removing shell completions..."
    
    # Remove completion files
    local comp_dir="$HOME/.local/share/typely/completions"
    if [[ -d "$comp_dir" ]]; then
        rm -rf "$comp_dir"
        success "Removed completions"
    fi
    
    # Note: We don't automatically remove shell configuration additions
    # as they may have been manually customized
    warn "You may want to manually remove Typely completion lines from:"
    warn "  - ~/.bashrc (bash users)"
    warn "  - ~/.zshrc (zsh users)"
    warn "  - ~/.config/fish/completions/typely.fish (fish users)"
}

# Verify removal
verify_removal() {
    log "Verifying removal..."
    
    local issues=()
    
    # Check if binaries still exist
    if command -v typely &> /dev/null; then
        issues+=("typely command still available")
    fi
    
    if command -v typely-cli &> /dev/null; then
        issues+=("typely-cli command still available")
    fi
    
    # Check if processes are still running
    if pgrep -f "typely" > /dev/null; then
        issues+=("Typely processes still running")
    fi
    
    if [[ ${#issues[@]} -gt 0 ]]; then
        warn "Removal incomplete. Issues found:"
        for issue in "${issues[@]}"; do
            echo "  - $issue"
        done
        echo ""
        warn "You may need to:"
        warn "  - Restart your terminal/shell"
        warn "  - Log out and log back in"
        warn "  - Manually remove remaining files"
    else
        success "Typely has been completely removed!"
    fi
}

# Show uninstall summary
show_summary() {
    echo ""
    echo "==============================================="
    echo "        Typely Uninstallation Complete"
    echo "==============================================="
    echo ""
    echo "What was removed:"
    echo "  ✓ Typely binaries"
    echo "  ✓ Desktop integration"
    echo "  ✓ Shell completions"
    echo "  ✓ Running processes"
    echo ""
    echo "Thank you for using Typely!"
    echo "If you have feedback, please share it at:"
    echo "  https://github.com/typely/typely/issues"
    echo ""
}

# Main uninstallation function
main() {
    echo "==============================================="
    echo "          Typely Uninstaller"
    echo "==============================================="
    echo ""
    
    log "This will completely remove Typely from your system."
    echo ""
    read -p "Are you sure you want to continue? [y/N]: " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log "Uninstallation cancelled."
        exit 0
    fi
    
    echo ""
    log "Starting Typely uninstallation..."
    
    check_root
    stop_typely
    remove_binaries
    remove_desktop_integration
    remove_completions
    remove_user_data
    verify_removal
    show_summary
}

# Run main function
main "$@"