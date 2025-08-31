# CLI Reference

Complete reference for the Typely command-line interface.

## Overview

Typely provides two main CLI commands:
- **`typely`** - Desktop application launcher
- **`typely-cli`** - Snippet management interface

## Desktop App (`typely`)

Start the desktop application with GUI and system tray integration.

### Usage
```bash
typely [OPTIONS]
```

### Options
- `--tray` - Start minimized to system tray
- `--version` - Show version information
- `--help` - Show help information

### Examples
```bash
# Start desktop app normally
typely

# Start minimized to tray
typely --tray

# Check version
typely --version
```

## CLI Management (`typely-cli`)

Manage snippets from the command line.

### Global Options
- `--database <PATH>` - Custom database path
- `--verbose` - Enable verbose output
- `--help` - Show help information
- `--version` - Show version information

## Commands

### `add` - Add New Snippet

Add a new text expansion snippet.

```bash
typely-cli add <TRIGGER> <REPLACEMENT> [OPTIONS]
```

#### Arguments
- `<TRIGGER>` - The trigger text (e.g., "::email")
- `<REPLACEMENT>` - The replacement text

#### Options
- `--tags <TAGS>` - Comma-separated tags for organization
- `--description <DESC>` - Optional description
- `--active` - Mark as active (default: true)

#### Examples
```bash
# Simple snippet
typely-cli add "::email" "john@example.com"

# With tags
typely-cli add "::work-email" "work@company.com" --tags "work,email"

# Multi-line with description
typely-cli add "::signature" "Best regards,\n{user}" --description "Email signature"

# With placeholders
typely-cli add "::date-time" "Today is {date} at {time}" --tags "date,time"
```

### `list` - List Snippets

Display existing snippets with filtering options.

```bash
typely-cli list [OPTIONS]
```

#### Options
- `--tags <TAGS>` - Filter by tags (comma-separated)
- `--active` - Show only active snippets
- `--inactive` - Show only inactive snippets
- `--search <TERM>` - Search in triggers and replacements
- `--limit <N>` - Limit number of results
- `--format <FORMAT>` - Output format: table, json, yaml

#### Examples
```bash
# List all snippets
typely-cli list

# Filter by tags
typely-cli list --tags "work,email"

# Search for specific terms
typely-cli list --search "signature"

# JSON output
typely-cli list --format json

# Only active snippets with limit
typely-cli list --active --limit 10
```

### `search` - Search Snippets

Search snippets by trigger, replacement, or tags.

```bash
typely-cli search <QUERY> [OPTIONS]
```

#### Arguments
- `<QUERY>` - Search query

#### Options
- `--tags <TAGS>` - Also search in tags
- `--case-sensitive` - Case-sensitive search
- `--regex` - Use regular expressions

#### Examples
```bash
# Basic search
typely-cli search "email"

# Case-sensitive search
typely-cli search "Email" --case-sensitive

# Regex search
typely-cli search "::.*email.*" --regex
```

### `update` - Update Snippet

Update an existing snippet.

```bash
typely-cli update <TRIGGER> [OPTIONS]
```

#### Arguments
- `<TRIGGER>` - The trigger to update

#### Options
- `--replacement <TEXT>` - New replacement text
- `--tags <TAGS>` - New tags (comma-separated)
- `--description <DESC>` - New description
- `--active <BOOL>` - Set active status (true/false)

#### Examples
```bash
# Update replacement text
typely-cli update "::email" --replacement "new@email.com"

# Update tags
typely-cli update "::work-email" --tags "work,corporate,email"

# Deactivate snippet
typely-cli update "::old-snippet" --active false
```

### `remove` - Remove Snippet

Delete snippets from the database.

```bash
typely-cli remove <TRIGGER> [OPTIONS]
```

#### Arguments
- `<TRIGGER>` - The trigger to remove

#### Options
- `--force` - Skip confirmation prompt
- `--backup` - Create backup before removal

#### Examples
```bash
# Remove with confirmation
typely-cli remove "::old-trigger"

# Force removal without confirmation
typely-cli remove "::old-trigger" --force

# Remove with backup
typely-cli remove "::deprecated" --backup
```

### `expand` - Test Expansion

Test snippet expansion without system integration.

```bash
typely-cli expand <TRIGGER> [OPTIONS]
```

#### Arguments
- `<TRIGGER>` - The trigger to expand

#### Options
- `--context <TEXT>` - Additional context for expansion
- `--format <FORMAT>` - Output format: text, json

#### Examples
```bash
# Simple expansion test
typely-cli expand "::email"

# Test with context
typely-cli expand "::signature" --context "formal"

# JSON output
typely-cli expand "::date" --format json
```

### `import` - Import Snippets

Import snippets from JSON files.

```bash
typely-cli import <FILE> [OPTIONS]
```

#### Arguments
- `<FILE>` - Path to JSON file

#### Options
- `--merge` - Merge with existing (default: replace duplicates)
- `--skip-duplicates` - Skip duplicate triggers
- `--backup` - Create backup before import
- `--dry-run` - Show what would be imported without changing database

#### Examples
```bash
# Import from file
typely-cli import snippets.json

# Import with backup
typely-cli import work-snippets.json --backup

# Dry run to preview
typely-cli import new-snippets.json --dry-run

# Skip duplicates
typely-cli import shared-snippets.json --skip-duplicates
```

### `export` - Export Snippets

Export snippets to JSON files.

```bash
typely-cli export <FILE> [OPTIONS]
```

#### Arguments
- `<FILE>` - Output file path

#### Options
- `--tags <TAGS>` - Export only specific tags
- `--active-only` - Export only active snippets
- `--format <FORMAT>` - Output format: json, yaml
- `--pretty` - Pretty-print output

#### Examples
```bash
# Export all snippets
typely-cli export backup.json

# Export specific tags
typely-cli export work-backup.json --tags "work,business"

# Pretty-printed JSON
typely-cli export snippets.json --pretty

# Export only active snippets
typely-cli export active-snippets.json --active-only
```

### `stats` - Usage Statistics

Show database and usage statistics.

```bash
typely-cli stats [OPTIONS]
```

#### Options
- `--detailed` - Show detailed statistics
- `--tags` - Group statistics by tags
- `--format <FORMAT>` - Output format: table, json

#### Examples
```bash
# Basic statistics
typely-cli stats

# Detailed statistics
typely-cli stats --detailed

# Tag-based statistics
typely-cli stats --tags

# JSON output
typely-cli stats --format json
```

### `validate` - Validate Database

Check database integrity and snippet validity.

```bash
typely-cli validate [OPTIONS]
```

#### Options
- `--fix` - Attempt to fix issues automatically
- `--verbose` - Show detailed validation results

#### Examples
```bash
# Validate database
typely-cli validate

# Validate and fix issues
typely-cli validate --fix

# Verbose validation
typely-cli validate --verbose
```

### `backup` - Database Backup

Create database backups.

```bash
typely-cli backup [OPTIONS]
```

#### Options
- `--path <PATH>` - Backup file path (default: auto-generated)
- `--compress` - Compress backup file

#### Examples
```bash
# Create backup with auto-generated name
typely-cli backup

# Create backup at specific path
typely-cli backup --path "/backups/typely-backup-$(date +%Y%m%d).json"

# Compressed backup
typely-cli backup --compress
```

## JSON File Format

### Snippet Structure
```json
[
  {
    "trigger": "::example",
    "replacement": "This is an example",
    "tags": ["example", "demo"],
    "description": "Optional description",
    "active": true,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z",
    "usage_count": 0
  }
]
```

### Field Descriptions
- `trigger` (required) - The trigger text that activates expansion
- `replacement` (required) - The text that replaces the trigger
- `tags` (optional) - Array of strings for organization
- `description` (optional) - Human-readable description
- `active` (optional) - Boolean indicating if snippet is active (default: true)
- `created_at` (optional) - ISO 8601 timestamp of creation
- `updated_at` (optional) - ISO 8601 timestamp of last update
- `usage_count` (optional) - Number of times snippet has been used

## Placeholders

Typely supports dynamic placeholders in replacement text:

### Built-in Placeholders
- `{date}` - Current date (YYYY-MM-DD format)
- `{time}` - Current time (HH:MM:SS format)
- `{datetime}` - Current date and time
- `{user}` - Current username
- `{year}` - Current year
- `{month}` - Current month name
- `{day}` - Current day of month

### Examples with Placeholders
```bash
# Date-based snippets
typely-cli add "::today" "Today is {date}"
typely-cli add "::now" "Current time: {time}"

# User-based snippets
typely-cli add "::signature" "Best regards,\n{user}"
typely-cli add "::copyright" "© {year} {user}. All rights reserved."
```

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Invalid arguments
- `3` - Database error
- `4` - File not found
- `5` - Permission denied

## Configuration

### Database Location
- **Linux**: `~/.local/share/typely/snippets.db`
- **macOS**: `~/Library/Application Support/typely/snippets.db`
- **Windows**: `%APPDATA%/typely/snippets.db`

### Custom Database
Use `--database` flag or set `TYPELY_DATABASE` environment variable:

```bash
export TYPELY_DATABASE="/path/to/custom.db"
typely-cli list
```

## Examples Directory

The project includes several example files:
- `examples/basic-snippets.json` - Simple text replacements
- `examples/work-snippets.json` - Professional templates
- `examples/programming-snippets.json` - Code templates
- `examples/personal-snippets.json` - Personal use cases

Import any of these to get started quickly:

```bash
typely-cli import examples/basic-snippets.json
typely-cli import examples/work-snippets.json
```

## Best Practices

### Trigger Naming
- Use consistent prefixes: `::email-`, `::addr-`, `::work-`
- Be descriptive: `::meeting-template` instead of `::mt`
- Use lowercase for consistency
- Avoid conflicts with common typing patterns

### Organization
- Use tags consistently for filtering and organization
- Group related snippets with common tag prefixes
- Keep replacement text readable and properly formatted
- Use descriptions for complex snippets

### Backup Strategy
- Regular exports: `typely-cli export backup-$(date +%Y%m%d).json`
- Tag-specific backups for different contexts
- Version control for shared snippet collections
- Test imports with `--dry-run` before applying