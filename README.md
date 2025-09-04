<div align="center">
  <img src="clients/gui/icons/typely_logo_full.png" alt="Typely" height="80">
</div>

# Typely

**Text expansion made easy**

A cross-platform productivity tool designed to streamline text expansion. Define and manage custom text snippets that automatically expand when triggered (e.g., typing `::asap` expands to "As soon as possible").

## 🏗️ Project Structure

```
typely/
├── backend/                 # 🦀 Core Rust backend
│   ├── bin/                # 🚀 Main entry points
│   │   └── main.rs        # Desktop app entry point
│   ├── domain/            # 🎯 Pure business entities & interfaces
│   │   ├── entities/      # Core domain entities (Snippet, etc.)
│   │   └── repositories/  # Repository trait definitions
│   ├── app/              # 🔧 Application services (merged use cases)
│   │   ├── services/     # All business services & use cases
│   │   └── dto/          # Data transfer objects
│   ├── infra/            # 🗄️ Infrastructure implementations
│   │   ├── database/     # SQLite database connection
│   │   ├── repositories/ # Repository implementations
│   │   └── engine/       # Text expansion engine
│   ├── lib.rs            # Backend library definition
│   └── scripts/          # 📦 Installation scripts
├── clients/
│   ├── cli/              # 💻 Command-line interface
│   ├── gui/              # 🖥️ Desktop GUI (Tauri)
│   └── frontend/         # 🌐 Landing page & documentation
├── examples/             # 📝 Usage examples & sample data
├── Makefile              # 🔨 Build system
└── Cargo.toml           # 📋 Workspace configuration
```

## ✨ Features

### 🔧 Core Functionality
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

## 🚀 Quick Start

### Installation

```bash
# Quick install (recommended)
curl -fsSL https://typely.sh/install | sh

# Or build from source
git clone https://github.com/typely/typely.git
cd typely
make install
```

### Usage

```bash
# Start the desktop application
typely

# Or use the CLI
typely-cli add "::email" "john@example.com"
typely-cli list
typely-cli expand "::email"
```

## 📝 Examples & Getting Started

Check out the `examples/` directory for:
- **Sample snippet collections** in JSON format
- **Comprehensive usage guide** with real-world examples
- **Import/export workflows** for different scenarios
- **Best practices** for organizing snippets

```bash
# Quick start with sample data
typely-cli import examples/basic-snippets.json
typely-cli import examples/work-snippets.json

# See all available examples
ls examples/*.json
```

## 🔧 Development

### Prerequisites
- Rust 1.70+
- System dependencies (automatically installed via `make install-deps`):
  - SQLite development libraries
  - X11 libraries (Linux): `libx11-dev`, `libxi-dev`, `libxtst-dev`
  - Build tools: `build-essential`, `pkg-config`

### Building

```bash
# 🔨 Use the Makefile (recommended)
make install-deps     # Install system dependencies
make all              # Build everything
make backend          # Build backend only
make cli              # Build CLI only  
make gui              # Build GUI only
make executable       # Build executable binaries
make install          # Install to system

# Or manually:
cd backend && cargo build --release --features system-integration
cd clients/cli && cargo build --release
cd clients/gui && cargo tauri build
```

### Architecture

This project follows **Domain-Driven Design (DDD)** principles with clean architecture:

- **Domain Layer**: Core business entities and interfaces - no external dependencies
- **App Layer**: Services and use cases that coordinate domain operations
- **Infra Layer**: Database, external services, and repository implementations
- **Clients**: Separate applications for different interfaces:
  - **CLI**: Command-line interface in `clients/cli/`
  - **GUI**: Desktop application in `clients/gui/`
  - **Frontend**: Web interface in `clients/frontend/`

### Features

```bash
# Build options
cargo build --release --features system-integration  # Full functionality (backend)
cd clients/cli && cargo build --release              # CLI application
cd clients/gui && cargo tauri build                  # GUI application

# Feature flags for backend
cargo build --no-default-features                    # Minimal build
cargo build --features system-integration            # With X11/system integration
```

## 📁 Directory Details

### Backend Structure

- **`domain/`**: Pure business logic, no external dependencies
  - `entities/`: Core business entities (Snippet, User, etc.)
  - `repositories/`: Repository trait definitions
  
- **`app/`**: Application services and use cases
  - `services/`: High-level application services
  - `use_cases/`: Specific business use cases
  - `dto/`: Data transfer objects for API boundaries
  
- **`infra/`**: External concerns and implementations
  - `database/`: SQLite database connection and migrations
  - `repositories/`: Concrete repository implementations
  - `engine/`: Text expansion engine with system integration

### Client Structure

- **`clients/cli/`**: Command-line interface (separate Rust crate)
- **`clients/gui/`**: Tauri-based desktop application
- **`clients/frontend/`**: Web-based landing page and documentation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Run the test suite (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://rust-lang.org) and [Tauri](https://tauri.app)
- Database powered by [SQLite](https://sqlite.org)
- CLI powered by [clap](https://clap.rs)

---

**Made with ❤️ by the LittleGiants team**