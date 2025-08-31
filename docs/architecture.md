# Architecture

Comprehensive overview of Typely's system design and architecture.

## 🏗️ Architectural Overview

Typely follows **Domain-Driven Design (DDD)** principles with clean architecture, ensuring separation of concerns and maintainability.

```
┌─────────────────────────────────────────────────────────┐
│                      Presentation Layer                 │
│  ┌─────────────────┐  ┌─────────────────┐             │
│  │   CLI Client    │  │   GUI Client    │             │
│  │   (clap-based)  │  │  (Tauri-based)  │             │
│  └─────────────────┘  └─────────────────┘             │
└─────────────────────────────────────────────────────────┘
                            │
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                    │
│  ┌─────────────────────────────────────────────────────┐  │
│  │              Service Orchestration                  │  │
│  │  • SnippetService  • ImportExportService           │  │
│  │  • ExpansionService • ValidationService            │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                            │
┌─────────────────────────────────────────────────────────┐
│                      Domain Layer                       │
│  ┌─────────────────┐  ┌─────────────────┐             │
│  │    Entities     │  │   Interfaces    │             │
│  │  • Snippet      │  │  • Repository   │             │
│  │  • User         │  │  • Engine       │             │
│  │  • Events       │  │  • Services     │             │
│  └─────────────────┘  └─────────────────┘             │
└─────────────────────────────────────────────────────────┘
                            │
┌─────────────────────────────────────────────────────────┐
│                  Infrastructure Layer                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐    │
│  │  Database   │  │   Engine    │  │   System    │    │
│  │  (SQLite)   │  │ (Expansion) │  │(Integration)│    │
│  │• Repository │  │• Detection  │  │• Keyboard   │    │
│  │• Migrations │  │• Replacement│  │• Clipboard  │    │
│  └─────────────┘  └─────────────┘  └─────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## 📁 Project Structure

```
typely/
├── backend/                 # 🦀 Core Rust backend
│   ├── lib.rs              # Library definition & module exports
│   ├── main.rs             # Desktop app entry point
│   │
│   ├── domain/             # 🎯 Pure business entities & interfaces
│   │   ├── entities/       # Core domain entities
│   │   │   ├── snippet.rs  # Snippet aggregate root
│   │   │   ├── user.rs     # User entity
│   │   │   ├── query.rs    # Query objects
│   │   │   └── events.rs   # Domain events
│   │   │
│   │   └── repositories/   # Repository trait definitions
│   │       ├── snippet_repository.rs
│   │       └── user_repository.rs
│   │
│   ├── app/               # 🔧 Application services & use cases
│   │   ├── services/      # Business services
│   │   │   ├── snippet_service.rs
│   │   │   ├── expansion_service.rs
│   │   │   ├── import_export_service.rs
│   │   │   ├── validation_service.rs
│   │   │   └── typely_service.rs  # Main orchestrator
│   │   │
│   │   └── dto/           # Data transfer objects
│   │       ├── snippet_dto.rs
│   │       └── export_dto.rs
│   │
│   ├── infra/            # 🗄️ Infrastructure implementations
│   │   ├── database/     # Database connection & migrations
│   │   │   ├── connection.rs
│   │   │   └── migrations.rs
│   │   │
│   │   ├── repositories/ # Concrete repository implementations
│   │   │   ├── sqlite_snippet_repository.rs
│   │   │   └── sqlite_user_repository.rs
│   │   │
│   │   ├── engine/       # Text expansion engine
│   │   │   ├── trigger_detection.rs
│   │   │   ├── text_replacement.rs
│   │   │   └── placeholder_resolver.rs
│   │   │
│   │   └── system.rs     # System integration (keyboard/clipboard)
│   │
│   └── cli/              # 💻 Command-line interface
│       ├── commands/     # CLI command implementations
│       ├── args.rs       # Argument parsing
│       └── output.rs     # Output formatting
│
├── clients/
│   ├── gui/              # 🖥️ Desktop GUI (Tauri)
│   │   ├── src/          # Frontend TypeScript/HTML/CSS
│   │   ├── src-tauri/    # Tauri Rust backend bridge
│   │   └── tauri.conf.json
│   │
│   └── frontend/         # 🌐 Landing page & documentation
│       ├── index.html
│       ├── styles.css
│       └── assets/
│
├── examples/             # 📝 Usage examples & sample data
├── docs/                # 📚 Documentation
└── Makefile            # 🔨 Build system
```

## 🎯 Domain Layer

### Core Principles
- **Pure Business Logic**: No external dependencies
- **Entity-Centric**: Domain entities are the source of truth
- **Interface Segregation**: Clear contracts for external dependencies

### Key Components

#### Entities
- **Snippet**: Aggregate root containing trigger, replacement, metadata
- **User**: User preferences and settings
- **Events**: Domain events for business logic tracking

#### Value Objects
- **SnippetQuery**: Query parameters and filtering
- **SortBy/SortOrder**: Sorting specifications
- **Tags**: Tag management and categorization

#### Interfaces
```rust
pub trait SnippetRepository {
    async fn save(&self, snippet: Snippet) -> Result<()>;
    async fn find_by_trigger(&self, trigger: &str) -> Result<Option<Snippet>>;
    async fn find_all(&self, query: SnippetQuery) -> Result<Vec<Snippet>>;
    async fn delete(&self, trigger: &str) -> Result<()>;
}

pub trait ExpansionEngine {
    fn detect_trigger(&self, input: &str) -> Option<String>;
    fn expand(&self, trigger: &str, replacement: &str) -> Result<String>;
}
```

## 🔧 Application Layer

### Service Architecture
The application layer orchestrates domain operations without containing business logic itself.

#### Key Services

**SnippetService**
- CRUD operations for snippets
- Validation and business rule enforcement
- Event emission for domain changes

**ExpansionService**
- Trigger detection coordination
- Text replacement orchestration
- Placeholder resolution

**ImportExportService**
- JSON serialization/deserialization
- Bulk operations
- Data validation

**ValidationService**
- Snippet validation rules
- Database integrity checks
- Import validation

**TypelyService** (Main Orchestrator)
- Unified API for all operations
- Cross-service coordination
- Transaction management

### Data Transfer Objects (DTOs)
```rust
pub struct SnippetDto {
    pub trigger: String,
    pub replacement: String,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub active: bool,
}

pub struct ExportDto {
    pub snippets: Vec<SnippetDto>,
    pub metadata: ExportMetadata,
}
```

## 🗄️ Infrastructure Layer

### Database Architecture
- **SQLite**: Local-first database for reliability and performance
- **SQLX**: Compile-time checked SQL queries
- **Migrations**: Version-controlled schema changes

#### Schema Design
```sql
-- Snippets table
CREATE TABLE snippets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trigger TEXT UNIQUE NOT NULL,
    replacement TEXT NOT NULL,
    tags TEXT, -- JSON array
    description TEXT,
    active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    usage_count INTEGER DEFAULT 0
);

-- Users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    preferences TEXT, -- JSON
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Text Expansion Engine

#### Trigger Detection
- **Pattern Matching**: Configurable trigger patterns
- **Context Awareness**: Understanding of cursor position and text context
- **Performance Optimization**: Efficient string matching algorithms

#### Text Replacement
- **Placeholder Resolution**: Dynamic content injection
- **Formatting Preservation**: Maintaining original text formatting
- **Undo Support**: Reversible expansions

#### System Integration
```rust
pub trait KeyboardMonitor {
    fn start_monitoring(&self) -> Result<()>;
    fn stop_monitoring(&self) -> Result<()>;
}

pub trait InputSimulator {
    fn simulate_backspace(&self, count: usize) -> Result<()>;
    fn simulate_text_input(&self, text: &str) -> Result<()>;
}

pub trait ClipboardManager {
    fn get_text(&self) -> Result<String>;
    fn set_text(&self, text: &str) -> Result<()>;
}
```

## 🖥️ Client Architecture

### Desktop GUI (Tauri)
- **Frontend**: TypeScript + HTML + CSS
- **Backend Bridge**: Rust commands exposed to frontend
- **System Integration**: Native OS capabilities

#### Tauri Commands
```rust
#[tauri::command]
async fn get_snippets() -> Result<Vec<SnippetDto>, String> {
    // Bridge to backend services
}

#[tauri::command]
async fn add_snippet(snippet: SnippetDto) -> Result<(), String> {
    // Bridge to backend services
}
```

### CLI Interface
- **Clap**: Argument parsing and command structure
- **Structured Output**: JSON, YAML, table formats
- **Error Handling**: User-friendly error messages

## 🔄 Data Flow

### Snippet Creation Flow
```
User Input (CLI/GUI)
    ↓
Command Parsing/Validation
    ↓
Application Service (SnippetService)
    ↓
Domain Validation (Snippet entity)
    ↓
Repository Interface
    ↓
Infrastructure Repository
    ↓
Database Storage
    ↓
Domain Event Emission
    ↓
Event Handlers (logging, notifications)
```

### Text Expansion Flow
```
Keyboard Input
    ↓
System Monitor (KeyboardMonitor)
    ↓
Trigger Detection (ExpansionEngine)
    ↓
Snippet Lookup (SnippetRepository)
    ↓
Placeholder Resolution (PlaceholderResolver)
    ↓
Text Replacement (InputSimulator)
    ↓
Usage Statistics Update
```

## 🛠️ Build System

### Makefile Targets
```makefile
all:           # Build everything
backend:       # Build backend only
gui:           # Build GUI client
frontend:      # Build landing page
install:       # Install to system
test:          # Run all tests
clean:         # Clean build artifacts
```

### Feature Flags
```rust
// Cargo.toml features
[features]
default = ["database", "system-integration"]
cli-only = ["database"]
system-integration = ["dep:enigo", "dep:global-hotkey"]
gui = ["tauri"]
```

### Cross-Platform Building
- **Linux**: Native compilation with system libraries
- **macOS**: Bundle creation with app signing
- **Windows**: MSI installer generation

## 🔧 Configuration

### Environment Variables
- `TYPELY_DATABASE`: Custom database path
- `TYPELY_LOG_LEVEL`: Logging verbosity
- `TYPELY_CONFIG_DIR`: Configuration directory

### Configuration File
```toml
# ~/.config/typely/config.toml
[database]
path = "~/.local/share/typely/snippets.db"
backup_interval = "24h"

[expansion]
trigger_prefix = "::"
case_sensitive = false
enable_placeholders = true

[system]
enable_global_hotkeys = true
monitoring_interval = 10
```

## 🧪 Testing Strategy

### Unit Tests
- Domain entity behavior
- Service logic verification
- Repository implementation testing

### Integration Tests
- Database operations
- CLI command execution
- Service integration

### End-to-End Tests
- Complete workflow testing
- GUI interaction testing
- System integration validation

### Test Organization
```
tests/
├── unit/           # Unit tests per module
├── integration/    # Cross-module integration
└── e2e/           # End-to-end scenarios
```

## 🚀 Deployment

### Installation Methods
1. **Installer Script**: `curl -fsSL https://typely.sh/install | sh`
2. **Package Managers**: Homebrew, APT, Chocolatey
3. **Manual Build**: From source compilation
4. **Binary Releases**: Pre-built binaries for each platform

### System Integration
- **Linux**: Desktop file, systemd service
- **macOS**: Application bundle, launchd service
- **Windows**: Start menu, Windows Service

## 🔍 Monitoring & Observability

### Logging
- **Structured Logging**: JSON-formatted logs
- **Log Levels**: Error, Warn, Info, Debug, Trace
- **Rotation**: Automatic log file rotation

### Metrics
- Snippet usage statistics
- Performance metrics
- Error rate tracking

### Health Checks
- Database connectivity
- System integration status
- Resource usage monitoring

## 🔒 Security Considerations

### Data Protection
- Local-only storage (no cloud dependency)
- Database encryption options
- Secure clipboard handling

### System Integration
- Minimal privilege requirements
- Sandboxed operations where possible
- User consent for system access

### Input Validation
- SQL injection prevention
- Command injection protection
- File path sanitization

## 📈 Performance Characteristics

### Database Performance
- SQLite optimizations for read-heavy workloads
- Indexed queries for fast snippet lookup
- Connection pooling for concurrent access

### Text Expansion Performance
- Low-latency trigger detection (<1ms)
- Efficient string replacement algorithms
- Minimal memory footprint

### System Resource Usage
- Memory: ~10MB typical usage
- CPU: <1% during monitoring
- Disk: ~1MB for typical snippet collections

## 🔮 Extensibility

### Plugin Architecture (Future)
- Custom expansion engines
- External data sources
- Third-party integrations

### API Design
- Consistent REST-like interfaces
- Versioned DTOs for compatibility
- Clear separation of concerns

This architecture provides a solid foundation for Typely's current features while remaining flexible for future enhancements and integrations.