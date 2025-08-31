# Troubleshooting

Common issues and solutions when using Typely.

## 🚨 Installation Issues

### Build Failures

#### Missing Dependencies (Linux)
**Problem**: Build fails with missing system libraries
```
error: linking with `cc` failed: exit status: 1
/usr/bin/ld: cannot find -lsqlite3
```

**Solution**: Install required development packages
```bash
# Ubuntu/Debian
sudo apt-get install build-essential libssl-dev libsqlite3-dev pkg-config

# CentOS/RHEL/Fedora
sudo dnf install gcc openssl-devel sqlite-devel pkg-config
# or
sudo yum install gcc openssl-devel sqlite-devel pkg-config

# Arch Linux
sudo pacman -S base-devel openssl sqlite pkg-config
```

#### Rust Version Compatibility
**Problem**: Compilation fails with older Rust versions
```
error[E0658]: `async fn` is not permitted in Rust 2015
```

**Solution**: Update Rust to version 1.70 or later
```bash
# Update Rust
rustup update

# Check version
rustc --version

# If still old, install latest stable
rustup install stable
rustup default stable
```

#### macOS Build Issues
**Problem**: Build fails on macOS with linking errors
```
error: linking with `cc` failed
ld: library not found for -lsqlite3
```

**Solution**: Install dependencies via Homebrew
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install sqlite pkg-config

# Set environment variables if needed
export PKG_CONFIG_PATH="$(brew --prefix)/lib/pkgconfig"
```

#### Windows Build Issues
**Problem**: Build fails on Windows with MSVC errors

**Solution**: Install Visual Studio Build Tools
1. Download Visual Studio Installer
2. Install "C++ build tools" workload
3. Ensure Windows SDK is included
4. Restart terminal and try again

Or use the GNU toolchain:
```powershell
# Install via rustup
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### Permission Issues

#### Linux/macOS Installation
**Problem**: Permission denied during installation
```bash
make install
# Permission denied
```

**Solution**: Run with appropriate permissions
```bash
# Option 1: Use sudo (system-wide installation)
sudo make install

# Option 2: Install to user directory
make install PREFIX=$HOME/.local

# Option 3: Manual binary copy
cp target/release/typely ~/.local/bin/
cp target/release/typely-cli ~/.local/bin/
```

#### Database Creation Issues
**Problem**: Cannot create database file
```
Error: Permission denied (os error 13)
Database path: /home/user/.local/share/typely/snippets.db
```

**Solution**: Check directory permissions and create if needed
```bash
# Create directory with proper permissions
mkdir -p ~/.local/share/typely
chmod 755 ~/.local/share/typely

# Or use custom database location
typely-cli --database ~/my-snippets.db list
```

## 🖥️ Desktop Application Issues

### Application Won't Start

#### Missing Dependencies
**Problem**: Desktop app fails to start with library errors
```
error while loading shared libraries: libwebkit2gtk-4.0.so.37
```

**Solution**: Install WebKit dependencies
```bash
# Ubuntu/Debian
sudo apt-get install webkit2gtk-4.0-dev

# Fedora
sudo dnf install webkit2gtk4.0-devel

# Arch Linux
sudo pacman -S webkit2gtk
```

#### Wayland Issues (Linux)
**Problem**: GUI doesn't appear or is corrupted on Wayland
```
Warning: Failed to connect to display
```

**Solution**: Force X11 mode or install Wayland support
```bash
# Force X11 (temporary fix)
export GDK_BACKEND=x11
typely

# Or install Wayland support
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev
```

### System Tray Issues

#### Tray Icon Not Appearing
**Problem**: System tray icon doesn't show

**Solutions**:
1. **Check tray support**: Ensure your desktop environment supports system tray
2. **Enable tray**: Some DEs require manual tray activation
   ```bash
   # GNOME with extensions
   gnome-extensions enable appindicatorsupport@rgcjonas.gmail.com
   ```
3. **Manual tray start**:
   ```bash
   typely --tray
   ```

#### Tray Menu Not Working
**Problem**: Right-click menu doesn't appear

**Solution**: Check for compositor issues
```bash
# Restart compositor (KDE)
kquitapp5 kwin_x11 && kwin_x11 &

# Or try without hardware acceleration
typely --disable-gpu
```

## 💻 CLI Issues

### Command Not Found

#### PATH Issues
**Problem**: `typely-cli` command not found
```bash
typely-cli: command not found
```

**Solutions**:
1. **Check installation location**:
   ```bash
   which typely-cli
   find /usr -name "typely-cli" 2>/dev/null
   ```

2. **Add to PATH**:
   ```bash
   # Add to ~/.bashrc or ~/.zshrc
   export PATH="$PATH:/usr/local/bin"
   
   # Or create symlink
   sudo ln -s /path/to/typely-cli /usr/local/bin/
   ```

3. **Use full path**:
   ```bash
   ./target/release/typely-cli list
   ```

### Database Connection Issues

#### Database Locked
**Problem**: Database operations fail with lock error
```
Error: database is locked (code 5)
```

**Solutions**:
1. **Close other instances**:
   ```bash
   # Kill all typely processes
   pkill typely
   
   # Check for remaining processes
   ps aux | grep typely
   ```

2. **Remove lock file**:
   ```bash
   # Remove SQLite lock files
   rm ~/.local/share/typely/*.db-wal
   rm ~/.local/share/typely/*.db-shm
   ```

3. **Check file permissions**:
   ```bash
   ls -la ~/.local/share/typely/
   chmod 644 ~/.local/share/typely/snippets.db
   ```

#### Corrupted Database
**Problem**: Database file is corrupted
```
Error: database disk image is malformed
```

**Solutions**:
1. **Backup and recreate**:
   ```bash
   # Backup existing data
   cp ~/.local/share/typely/snippets.db ~/.local/share/typely/snippets.db.backup
   
   # Try to export before recreation
   typely-cli export backup.json 2>/dev/null
   
   # Remove corrupted database
   rm ~/.local/share/typely/snippets.db
   
   # Recreate from backup if export worked
   typely-cli import backup.json
   ```

2. **SQLite recovery**:
   ```bash
   # Try to recover using SQLite
   sqlite3 ~/.local/share/typely/snippets.db.backup ".recover" | sqlite3 ~/.local/share/typely/snippets.db
   ```

### Import/Export Issues

#### JSON Format Errors
**Problem**: Import fails with JSON parsing errors
```
Error: invalid JSON format at line 15
```

**Solutions**:
1. **Validate JSON**:
   ```bash
   # Check JSON validity
   python -m json.tool snippets.json > /dev/null
   
   # Or use jq
   jq empty snippets.json
   ```

2. **Common fixes**:
   - Remove trailing commas
   - Escape special characters in strings
   - Use double quotes for strings
   - Check for missing brackets

#### Large File Imports
**Problem**: Import fails with large files
```
Error: memory allocation failed
```

**Solutions**:
1. **Split large files**:
   ```bash
   # Split JSON array into smaller files
   jq -c '.[]' large-file.json | split -l 1000 - snippets-part-
   ```

2. **Import in batches**:
   ```bash
   for file in snippets-part-*; do
     echo "[$(<$file)]" | typely-cli import /dev/stdin
   done
   ```

## 🔧 Text Expansion Issues

### Expansion Not Working

#### System Integration
**Problem**: Typing triggers doesn't expand text

**Solutions**:
1. **Check system integration build**:
   ```bash
   # Verify build features
   typely-cli --version
   # Should show "with system integration"
   ```

2. **Permission issues (Linux)**:
   ```bash
   # Add user to input group
   sudo usermod -a -G input $USER
   # Logout and login again
   ```

3. **Accessibility permissions (macOS)**:
   - System Preferences → Security & Privacy → Privacy → Accessibility
   - Add Typely to allowed applications

4. **Windows UAC issues**:
   - Run as administrator initially
   - Or disable UAC for Typely in Windows settings

#### Specific Application Issues
**Problem**: Expansion works in some apps but not others

**Solutions**:
1. **Check application compatibility**:
   - Some apps (like password managers) block input injection
   - Try in simple text editor first

2. **Timing adjustments**:
   ```bash
   # Slower expansion for problematic apps
   typely --expansion-delay 100ms
   ```

### Trigger Detection Issues

#### False Positives
**Problem**: Unwanted expansions occur

**Solutions**:
1. **Adjust trigger patterns**:
   ```bash
   # Use more specific triggers
   typely-cli update "::e" --replacement "more-specific-trigger"
   ```

2. **Context sensitivity**:
   - Use longer, more unique triggers
   - Avoid common letter combinations

#### Case Sensitivity
**Problem**: Triggers don't work with different cases

**Solution**: Configure case sensitivity
```bash
# Check current settings
typely --show-config

# Update settings (when configuration is implemented)
typely --case-sensitive false
```

## 🔍 Performance Issues

### Slow Trigger Detection
**Problem**: Noticeable delay before expansion

**Solutions**:
1. **Check system resources**:
   ```bash
   # Monitor CPU usage
   top -p $(pgrep typely)
   
   # Check memory usage
   ps -o pid,vsz,rss,comm -p $(pgrep typely)
   ```

2. **Optimize database**:
   ```bash
   # Vacuum database
   sqlite3 ~/.local/share/typely/snippets.db "VACUUM;"
   
   # Analyze query performance
   sqlite3 ~/.local/share/typely/snippets.db "PRAGMA optimize;"
   ```

3. **Reduce snippet count**:
   ```bash
   # Remove unused snippets
   typely-cli list --inactive
   typely-cli remove "::unused-trigger"
   ```

### High Memory Usage
**Problem**: Typely uses excessive memory

**Solutions**:
1. **Check for memory leaks**:
   ```bash
   # Monitor memory over time
   while true; do
     ps -o pid,vsz,rss,comm -p $(pgrep typely)
     sleep 60
   done
   ```

2. **Restart periodically**:
   ```bash
   # Add to crontab for daily restart
   0 3 * * * pkill typely && sleep 5 && typely --tray
   ```

## 🐛 Debug Mode

### Enabling Debug Logging
```bash
# Environment variable
RUST_LOG=debug typely

# Or for CLI operations
RUST_LOG=debug typely-cli list

# Verbose output
typely-cli --verbose list
```

### Log File Locations
- **Linux**: `~/.local/share/typely/logs/`
- **macOS**: `~/Library/Logs/typely/`
- **Windows**: `%APPDATA%/typely/logs/`

### Common Debug Information
```bash
# Check configuration
typely --show-config

# Database statistics
typely-cli stats --detailed

# Validate installation
typely-cli validate --verbose

# Test expansion without system integration
typely-cli expand "::test"
```

## 📞 Getting Help

### Before Reporting Issues
1. **Check this troubleshooting guide**
2. **Update to latest version**:
   ```bash
   typely --version
   # Compare with latest release
   ```
3. **Test with minimal configuration**:
   ```bash
   # Use temporary database
   typely-cli --database /tmp/test.db add "::test" "Hello World"
   typely-cli --database /tmp/test.db expand "::test"
   ```

### Collecting Debug Information
```bash
# System information
uname -a
rustc --version
typely --version

# Configuration and logs
typely --show-config
tail -n 100 ~/.local/share/typely/logs/typely.log

# Database information
typely-cli stats --detailed
sqlite3 ~/.local/share/typely/snippets.db ".schema"
```

### Reporting Bugs
Include the following information:
- **Operating system and version**
- **Typely version**: `typely --version`
- **Rust version**: `rustc --version`
- **Build configuration**: Features enabled during compilation
- **Error messages**: Full error text and stack traces
- **Steps to reproduce**: Minimal reproduction case
- **Expected vs actual behavior**

### Community Support
- **GitHub Issues**: https://github.com/typely/typely/issues
- **Documentation**: Check all documentation files in `docs/`
- **Examples**: Review `examples/` directory for usage patterns

### Emergency Recovery
If Typely is completely broken:
```bash
# 1. Stop all processes
pkill typely

# 2. Backup data
cp -r ~/.local/share/typely/ ~/typely-backup/

# 3. Clean installation
rm -rf ~/.local/share/typely/
rm ~/.local/bin/typely*

# 4. Reinstall
curl -fsSL https://typely.sh/install | sh

# 5. Restore data
typely-cli import ~/typely-backup/export.json
```

Remember: When in doubt, create a backup of your snippets with `typely-cli export` before attempting fixes!