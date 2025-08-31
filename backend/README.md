# Typely

**Text expansion made easy**

Typely is a cross-platform productivity tool designed to streamline text expansion. It allows users to define and manage custom text snippets that automatically expand when triggered (e.g., typing `::asap` expands to "As soon as possible").

## Features

### ✨ Core Functionality
- **Text Expansion Engine**: System-wide text expansion that works in any application
- **Custom Triggers**: Define your own trigger patterns (e.g., `::email`, `::addr`)
- **Dynamic Placeholders**: Support for date/time and user info (`{date}`, `{time}`, `{user}`)
- **Smart Formatting**: Preserve formatting and context

### 🖥️ Desktop Application
- **Tray Integration**: Runs quietly in the system tray
- **Modal UI**: Quick access to snippet management
- **Cross-Platform**: Works on macOS, Linux, and Windows

### 🔧 Command Line Interface
- **Full CLI Support**: Complete snippet management from the terminal
- **Import/Export**: JSON-based snippet backup and sharing
- **Search & Filter**: Find snippets quickly with powerful search
- **Statistics**: Track usage and performance

### 💾 Database & Storage
- **Local SQLite Database**: Fast, reliable local storage
- **Auto-backup**: Automatic snippet backup and recovery
- **Migration System**: Safe database schema updates

## Installation

### Quick Install (Recommended)

```bash
curl -fsSL https://typely.sh/install | sh
```

### Manual Installation

#### Prerequisites
- Rust 1.70+ (for building from source)
- SQLite development libraries

On Ubuntu/Debian:
```bash
sudo apt-get update
sudo apt-get install -y libx11-dev libxi-dev libxtst-dev libsqlite3-dev
```

On macOS:
```bash
brew install sqlite
```

#### Build from Source

```bash
# Clone the repository
git clone https://github.com/typely/typely.git
cd typely

# Build with system integration (full functionality)
cargo build --release --features system-integration

# Or build CLI-only version (for servers/headless environments)
cargo build --release --no-default-features --features cli-only

# Install binaries
sudo cp target/release/typely /usr/local/bin/
sudo cp target/release/typely-cli /usr/local/bin/
```

## Usage

### Desktop Application

Start Typely in the background:
```bash
typely
```

The application will:
- Run in the system tray
- Monitor for trigger patterns
- Expand text automatically
- Provide a modal UI for snippet management (click the tray icon)

### Command Line Interface

#### Basic Commands

```bash
# Add a new snippet
typely-cli add "::email" "john@example.com"

# Add a snippet with tags
typely-cli add "::sig" "Best regards,\nJohn" --tags "signature,email"

# List all snippets
typely-cli list

# Search snippets
typely-cli search "email"

# Show snippet details
typely-cli show "::email"

# Test expansion
typely-cli expand "::email"

# Update a snippet
typely-cli update "::email" --replacement "john.doe@example.com"

# Remove a snippet
typely-cli remove "::email"
```

#### Import/Export

```bash
# Export all snippets to JSON
typely-cli export snippets.json

# Export only active snippets
typely-cli export active-snippets.json

# Export snippets with specific tags
typely-cli export work-snippets.json --tags "work,business"

# Import snippets from JSON
typely-cli import snippets.json

# Import with overwrite
typely-cli import snippets.json --overwrite
```

#### Statistics

```bash
# Show usage statistics
typely-cli stats

# Verbose statistics with tags
typely-cli --verbose stats
```

### Advanced Features

#### Dynamic Placeholders

Typely supports dynamic placeholders that are expanded at runtime:

- `{date}` - Current date (YYYY-MM-DD)
- `{time}` - Current time (HH:MM:SS)
- `{datetime}` - Current date and time
- `{timestamp}` - Unix timestamp
- `{user}` - Current username

Example:
```bash
typely-cli add "::today" "Today is {date}"
typely-cli add "::meeting" "Meeting scheduled for {datetime}"
```

#### Tagging System

Organize snippets with tags:

```bash
# Add snippets with tags
typely-cli add "::work-email" "john@company.com" --tags "work,email"
typely-cli add "::home-email" "john@personal.com" --tags "personal,email"

# List snippets by tag
typely-cli list --tags "work"

# Export specific tags
typely-cli export work-snippets.json --tags "work"
```

## Configuration

### Database Location

By default, Typely stores snippets in:
- **Linux/Unix**: `~/.local/share/typely/snippets.db`
- **macOS**: `~/Library/Application Support/typely/snippets.db`  
- **Windows**: `%APPDATA%\typely\snippets.db`

You can specify a custom database location:
```bash
typely-cli --database /path/to/custom.db list
```

### Environment Variables

- `TYPELY_DB_PATH` - Custom database path
- `RUST_LOG` - Logging level (debug, info, warn, error)

## Development

### Architecture

Typely follows Domain-Driven Design (DDD) and Clean Architecture principles:

```
src/
├── domain/          # Business logic and entities
│   ├── entities/    # Core entities (Snippet)
│   ├── services/    # Domain services
│   └── repositories/ # Repository interfaces
├── application/     # Use cases and application services
│   ├── use_cases/   # Business use cases
│   └── dto/         # Data transfer objects
├── infrastructure/ # External concerns
│   ├── database/    # SQLite implementation
│   ├── repositories/ # Repository implementations
│   └── system/      # OS integration
└── presentation/   # User interfaces
    ├── cli/         # Command line interface
    ├── gui/         # Desktop GUI (Tauri)
    └── engine/      # Text expansion engine
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Build specific features
cargo build --features system-integration
cargo build --no-default-features --features cli-only

# Run tests
cargo test

# Check code
cargo check
cargo clippy
```

### Features

- `default` - CLI-only functionality
- `gui` - Desktop GUI with Tauri
- `system-integration` - Full system integration (keyboard monitoring, input simulation)
- `cli-only` - Command line only

## Troubleshooting

### Common Issues

#### Database Permission Errors
```bash
# Check database permissions
ls -la ~/.local/share/typely/

# Create directory if missing
mkdir -p ~/.local/share/typely/
```

#### System Integration Not Working
Make sure you built with system integration:
```bash
cargo build --release --features system-integration
```

On Linux, ensure X11 development libraries are installed:
```bash
sudo apt-get install libx11-dev libxi-dev libxtst-dev
```

#### Text Expansion Not Detected
- Verify Typely is running in the background
- Check that accessibility permissions are granted (macOS)
- Ensure the application has input monitoring permissions

### Debugging

Enable debug logging:
```bash
RUST_LOG=debug typely-cli list
```

Check database content:
```bash
sqlite3 ~/.local/share/typely/snippets.db ".tables"
sqlite3 ~/.local/share/typely/snippets.db "SELECT * FROM snippets;"
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Development Setup

```bash
# Clone and setup
git clone https://github.com/typely/typely.git
cd typely

# Install development dependencies
rustup component add clippy rustfmt

# Setup pre-commit hooks (optional)
cargo install pre-commit
pre-commit install
```

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure clippy passes (`cargo clippy`)
- Write tests for new features
- Update documentation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Cloud sync with encryption
- [ ] Multi-device synchronization
- [ ] Rich text formatting (Markdown support)
- [ ] AI-powered snippet suggestions
- [ ] Browser extension
- [ ] Mobile app support
- [ ] Team collaboration features
- [ ] Advanced scripting support

## Support

- 📖 [Documentation](https://typely.sh/docs)
- 🐛 [Issue Tracker](https://github.com/typely/typely/issues)
- 💬 [Discussions](https://github.com/typely/typely/discussions)
- 📧 [Email Support](mailto:support@typely.sh)

## Acknowledgments

- Built with [Rust](https://rust-lang.org) and [Tauri](https://tauri.app)
- Database powered by [SQLite](https://sqlite.org)
- CLI powered by [clap](https://clap.rs)
- System integration via [rdev](https://github.com/obv-mikhail/rdev) and [enigo](https://github.com/enigo-rs/enigo)

---

**Made with ❤️ by the Typely team**