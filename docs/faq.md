# Frequently Asked Questions (FAQ)

Common questions and answers about Typely.

## 🚀 Getting Started

### Q: What is Typely?
**A:** Typely is a cross-platform text expansion tool that automatically replaces short trigger phrases (like `::email`) with longer text snippets (like your full email address) as you type. It works system-wide in any application.

### Q: How is Typely different from other text expanders?
**A:** Typely focuses on:
- **Local-first**: All data stays on your device, no cloud dependency
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Developer-friendly**: CLI-first design with powerful import/export
- **Open source**: Free and transparent
- **Clean architecture**: Well-organized codebase for contributors

### Q: Do I need to use both the CLI and desktop app?
**A:** No, choose based on your workflow:
- **CLI only**: Perfect for managing snippets, automation, and testing
- **Desktop app**: Needed for real-time text expansion while typing
- **Both**: Use CLI for management and desktop app for expansion

## 💾 Installation & Setup

### Q: What are the system requirements?
**A:** 
- **OS**: Linux, macOS 10.12+, or Windows 10+
- **RAM**: 10MB typical usage
- **Disk**: ~1MB for typical snippet collections
- **Dependencies**: SQLite (included), system libraries for keyboard integration

### Q: How do I install Typely?
**A:** Three main methods:
```bash
# Quick install (recommended)
curl -fsSL https://typely.sh/install | sh

# Build from source
git clone https://github.com/typely/typely.git
cd typely && make install

# Manual binary installation
# Download from releases page and add to PATH
```

### Q: Where is my data stored?
**A:** Locally in SQLite database:
- **Linux**: `~/.local/share/typely/snippets.db`
- **macOS**: `~/Library/Application Support/typely/snippets.db`  
- **Windows**: `%APPDATA%/typely/snippets.db`

### Q: Can I use a custom database location?
**A:** Yes, several ways:
```bash
# Per-command basis
typely-cli --database /path/to/custom.db list

# Environment variable
export TYPELY_DATABASE="/path/to/custom.db"

# Configuration file (planned feature)
```

## 📝 Using Snippets

### Q: What makes a good trigger?
**A:** Follow these best practices:
- **Unique prefix**: Start with `::` to avoid conflicts
- **Descriptive**: `::work-email` better than `::we`
- **Consistent**: Use patterns like `::email-personal`, `::email-work`
- **Not too short**: Avoid `::e` which might trigger accidentally
- **Memorable**: Use logical abbreviations

### Q: What can I put in replacement text?
**A:** Anything! Common examples:
- **Email addresses**: `john@example.com`
- **Postal addresses**: `123 Main St, City, State 12345`
- **Phone numbers**: `+1 (555) 123-4567`
- **Code templates**: Function signatures, boilerplate
- **Multi-line text**: Meeting notes, email signatures
- **Dynamic content**: Using placeholders like `{date}`, `{user}`

### Q: How do placeholders work?
**A:** Typely supports dynamic placeholders:
```bash
# Built-in placeholders
::date     → {date}        # 2024-01-15
::time     → {time}        # 14:30:25
::user     → {user}        # current username
::year     → {year}        # 2024

# Examples
typely-cli add "::today" "Today is {date}"
typely-cli add "::sig" "Best regards,\n{user}"
typely-cli add "::copyright" "© {year} Company Name"
```

### Q: Can I use multi-line snippets?
**A:** Yes! Use `\n` for line breaks:
```bash
typely-cli add "::meeting" "# Meeting Notes - {date}\n\n**Attendees:**\n- \n\n**Discussion:**\n- "
```

### Q: How do I organize my snippets?
**A:** Use tags for organization:
```bash
# Add with tags
typely-cli add "::work-email" "work@company.com" --tags "work,email"
typely-cli add "::home-addr" "123 Main St..." --tags "personal,address"

# Filter by tags
typely-cli list --tags "work"
typely-cli list --tags "email,personal"

# Export by category
typely-cli export work-snippets.json --tags "work"
```

## 🖥️ Desktop Application

### Q: How do I start the desktop app?
**A:** Multiple ways:
```bash
# Normal startup
typely

# Start in system tray
typely --tray

# Check if it's running
ps aux | grep typely
```

### Q: Why doesn't text expansion work in some applications?
**A:** Some applications block input injection for security:
- **Password managers**: Deliberately block automated input
- **Secure terminals**: May require additional permissions
- **Some games**: Block external input for anti-cheat
- **Virtual machines**: May not pass through simulated keystrokes

Try in a simple text editor first to verify Typely is working.

### Q: Do I need special permissions?
**A:** Platform-specific requirements:
- **Linux**: May need to add user to `input` group
- **macOS**: Accessibility permissions in System Preferences
- **Windows**: May need to run as administrator initially

### Q: Can I disable expansion temporarily?
**A:** Yes, several options:
```bash
# Pause via tray menu (GUI)
# Right-click tray icon → Pause

# Stop desktop app temporarily
pkill typely

# Deactivate specific snippets
typely-cli update "::trigger" --active false
```

## 📁 Import & Export

### Q: What file formats are supported?
**A:** JSON format for maximum compatibility:
```json
[
  {
    "trigger": "::example",
    "replacement": "Example text",
    "tags": ["demo", "test"],
    "description": "Optional description",
    "active": true
  }
]
```

### Q: How do I backup my snippets?
**A:** Regular exports recommended:
```bash
# Full backup
typely-cli export backup-$(date +%Y%m%d).json

# Category-specific backups
typely-cli export work-backup.json --tags "work"

# Automated backup (add to cron)
0 2 * * * /usr/local/bin/typely-cli export ~/backups/typely-$(date +\%Y\%m\%d).json
```

### Q: Can I share snippets with others?
**A:** Yes! Export and share JSON files:
```bash
# Create shareable collection
typely-cli export team-snippets.json --tags "work,shared"

# Others can import
typely-cli import team-snippets.json

# Merge without replacing
typely-cli import team-snippets.json --skip-duplicates
```

### Q: How do I migrate from another text expander?
**A:** Manual process currently:
1. Export from your current tool (if supported)
2. Convert to Typely JSON format
3. Import using `typely-cli import`

Common conversions needed:
- Trigger format (adapt to `::prefix` style)
- Placeholder syntax (convert to `{variable}` format)
- Tag structure (adapt to array format)

## 🔧 Troubleshooting

### Q: The command isn't found after installation
**A:** PATH issues, try:
```bash
# Check if installed
which typely-cli
find /usr -name "typely-cli" 2>/dev/null

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$PATH:/usr/local/bin"

# Use full path temporarily
./target/release/typely-cli list
```

### Q: I get "database is locked" errors
**A:** Multiple instances or permissions:
```bash
# Kill all typely processes
pkill typely

# Check for lock files
ls ~/.local/share/typely/
rm ~/.local/share/typely/*.db-wal
rm ~/.local/share/typely/*.db-shm

# Check permissions
chmod 644 ~/.local/share/typely/snippets.db
```

### Q: Text expansion is too slow
**A:** Performance optimization:
```bash
# Vacuum database
sqlite3 ~/.local/share/typely/snippets.db "VACUUM;"

# Remove unused snippets
typely-cli list --inactive
typely-cli remove "::unused-trigger"

# Check system resources
top -p $(pgrep typely)
```

### Q: How do I completely uninstall Typely?
**A:** Remove binaries and data:
```bash
# Stop all processes
pkill typely

# Remove binaries
sudo rm /usr/local/bin/typely*
# or
rm ~/.local/bin/typely*

# Remove data (backup first!)
typely-cli export final-backup.json
rm -rf ~/.local/share/typely/

# Remove configuration
rm -rf ~/.config/typely/
```

## 🔐 Security & Privacy

### Q: Is my data sent to the cloud?
**A:** No. Typely is completely local:
- All snippets stored in local SQLite database
- No network connections for normal operation
- No telemetry or analytics
- No account registration required

### Q: Can other applications access my snippets?
**A:** Only applications with database file access:
- Database file uses standard filesystem permissions
- Consider encrypting home directory for additional security
- No special protection beyond OS file permissions

### Q: Is it safe to store passwords in snippets?
**A:** Not recommended:
- Snippets are stored in plain text in database
- Consider using a dedicated password manager instead
- If necessary, use partial passwords or hints only

## 🛠️ Development & Contributing

### Q: How can I contribute to Typely?
**A:** Several ways:
- **Bug reports**: Submit issues on GitHub
- **Feature requests**: Propose new functionality
- **Code contributions**: Submit pull requests
- **Documentation**: Improve guides and examples
- **Testing**: Try on different platforms

### Q: What's the development setup?
**A:** Standard Rust development:
```bash
# Clone repository
git clone https://github.com/typely/typely.git

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and test
cd typely
cargo build
cargo test

# Use Makefile for convenience
make all
make test
```

### Q: How is the codebase organized?
**A:** Clean architecture with Domain-Driven Design:
- **`domain/`**: Pure business logic and entities
- **`app/`**: Application services and use cases  
- **`infra/`**: Database, system integration, external concerns
- **`cli/`**: Command-line interface
- **`clients/`**: Desktop GUI and web frontend

### Q: Can I add custom expansion engines?
**A:** Currently internal, but planned:
- Plugin architecture in future versions
- Custom trigger patterns
- External data source integration
- Third-party service connections

## 📈 Performance & Limits

### Q: How many snippets can I have?
**A:** No hard limits, but practical considerations:
- SQLite handles millions of records efficiently
- Memory usage scales with active snippet count
- Trigger detection speed may slow with very large collections
- Recommended: <10,000 snippets for optimal performance

### Q: Does Typely slow down my system?
**A:** Minimal impact:
- **Memory**: ~10MB typical usage
- **CPU**: <1% during normal operation
- **Startup time**: ~100ms initialization
- **Expansion latency**: <1ms for trigger detection

### Q: Can I use Typely on multiple computers?
**A:** Yes, with manual synchronization:
```bash
# Computer A: Export
typely-cli export shared-snippets.json

# Transfer file to Computer B

# Computer B: Import  
typely-cli import shared-snippets.json --skip-duplicates
```

Automatic sync is a planned feature for future versions.

## 🔮 Future Plans

### Q: What features are planned?
**A:** Roadmap includes:
- **Cloud synchronization**: Optional sync across devices
- **Plugin system**: Custom expansion engines and integrations
- **Advanced placeholders**: More dynamic content options
- **GUI snippet editor**: Visual interface for complex snippets
- **Statistics dashboard**: Usage analytics and insights
- **Team sharing**: Shared snippet collections
- **Mobile apps**: iOS and Android support

### Q: When will feature X be available?
**A:** Check the GitHub project roadmap for current status. Features are prioritized based on:
- Community demand
- Technical complexity  
- Maintainer availability
- Platform requirements

### Q: Can I request a feature?
**A:** Yes! Submit feature requests:
- **GitHub Issues**: Detailed feature proposals
- **Community discussion**: Join conversations about priorities
- **Pull requests**: Implement features yourself
- **Use cases**: Explain your specific needs

## 📞 Getting More Help

### Q: Where can I get support?
**A:** Multiple channels:
- **Documentation**: Check all files in `docs/` directory
- **GitHub Issues**: Bug reports and feature requests
- **Troubleshooting**: See [troubleshooting.md](troubleshooting.md)
- **Examples**: Review `examples/` directory

### Q: How do I report a bug?
**A:** Include this information:
- **Operating system and version**
- **Typely version**: `typely --version`
- **Steps to reproduce the issue**
- **Expected vs actual behavior**
- **Error messages or logs**
- **Configuration details**

### Q: Is there a community forum?
**A:** Currently GitHub Issues serves as the main community hub. Consider:
- **Searching existing issues** before posting
- **Using appropriate labels** (bug, feature, question)
- **Providing detailed information** for faster resolution
- **Following up** on your issues

Remember: The best way to get help is to provide clear, detailed information about your specific situation and what you've already tried!