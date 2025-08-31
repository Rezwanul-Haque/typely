# Typely Examples

This directory contains examples and sample files to help you get started with Typely.

## 📁 Files

- **`snippets.json`** - Sample snippets demonstrating the JSON format
- **`basic-snippets.json`** - Basic text expansion examples
- **`advanced-snippets.json`** - Advanced snippets with placeholders
- **`work-snippets.json`** - Professional/work-related snippets
- **`programming-snippets.json`** - Code and development snippets

## 🚀 Quick Start Guide

### 1. Import Sample Snippets

```bash
# Import basic examples
typely-cli import examples/snippets.json

# Import work-related snippets
typely-cli import examples/work-snippets.json

# Import with overwrite (replaces existing)
typely-cli import examples/basic-snippets.json --overwrite
```

### 2. List Your Snippets

```bash
# List all snippets
typely-cli list

# List only active snippets
typely-cli list --active

# Search for specific snippets
typely-cli search "email"
```

### 3. Test Snippet Expansion

```bash
# Test a snippet expansion
typely-cli expand "::email"

# Show snippet details
typely-cli show "::hello"
```

## 📝 Usage Examples

### Basic Text Expansion

```bash
# Add a simple snippet
typely-cli add "::addr" "123 Main St, Anytown, USA"

# Add with tags
typely-cli add "::phone" "555-123-4567" --tags "contact,personal"

# Use the snippet (when system integration is enabled)
# Type: ::addr
# Result: 123 Main St, Anytown, USA
```

### Dynamic Content with Placeholders

```bash
# Add snippet with date placeholder
typely-cli add "::date" "Today is {date}"

# Add snippet with time
typely-cli add "::timestamp" "Generated at {datetime}"

# Add snippet with user info
typely-cli add "::sig" "Best regards,\n{user}"
```

### Professional Examples

```bash
# Email signatures
typely-cli add "::sig-work" "Best regards,\n{user}\nSoftware Engineer\nCompany Name\nphone@company.com"

# Meeting templates
typely-cli add "::meeting" "Hi team,\n\nMeeting scheduled for {date} at {time}.\n\nAgenda:\n- \n\nBest regards,\n{user}"

# Code snippets
typely-cli add "::fn-js" "function {cursor}() {\n    // TODO: Implementation\n    return null;\n}"
```

### Bulk Operations

```bash
# Export all snippets
typely-cli export my-backup.json

# Export only work-related snippets
typely-cli export work-backup.json --tags "work,business"

# Export only active snippets
typely-cli export active-backup.json --active

# Import and merge
typely-cli import shared-snippets.json

# Import and replace duplicates
typely-cli import new-snippets.json --overwrite
```

### Organization with Tags

```bash
# Add snippets with multiple tags
typely-cli add "::bug-template" "**Bug Report**\n\n**Description:**\n\n**Steps to Reproduce:**\n1. \n\n**Expected Result:**\n\n**Actual Result:**" --tags "work,templates,github"

# List snippets by tag
typely-cli list --tags "work"

# Export by tag
typely-cli export github-templates.json --tags "github,templates"
```

## 🔧 Management Commands

```bash
# Update a snippet
typely-cli update "::email" --replacement "new-email@example.com"

# Activate/deactivate snippets
typely-cli update "::old-addr" --deactivate
typely-cli update "::addr" --activate

# Remove a snippet
typely-cli remove "::unused"

# View statistics
typely-cli stats
```

## 📊 JSON Format Reference

The JSON format for import/export follows this structure:

```json
[
  {
    "trigger": "::example",
    "replacement": "This is the expanded text",
    "tags": ["category", "type"]
  }
]
```

### Field Descriptions:

- **`trigger`** (required): The text that triggers expansion (e.g., "::hello")
- **`replacement`** (required): The text that replaces the trigger
- **`tags`** (optional): Array of tags for organization, can be `null`

### Placeholder Support:

You can use these placeholders in your replacement text:
- `{date}` - Current date (YYYY-MM-DD)
- `{time}` - Current time (HH:MM:SS)
- `{datetime}` - Current date and time
- `{timestamp}` - Unix timestamp
- `{user}` - Current username

## 🎯 Best Practices

### Trigger Naming

```bash
# Use consistent prefixes
::email-work    # Work email
::email-personal # Personal email
::addr-home     # Home address
::addr-work     # Work address

# Use descriptive names
::meeting-template  # Better than ::mt
::bug-report       # Better than ::br
```

### Organization

```bash
# Use meaningful tags
--tags "work,email,templates"
--tags "personal,contact"
--tags "code,javascript,functions"
```

### Backup Strategy

```bash
# Regular backups
typely-cli export backup-$(date +%Y%m%d).json

# Category-specific backups
typely-cli export work-backup.json --tags "work"
typely-cli export personal-backup.json --tags "personal"
```

## 🔗 Desktop vs CLI Usage

- **CLI Mode**: Perfect for managing snippets, bulk operations, and automation
- **Desktop Mode**: Provides system-wide text expansion with real-time triggering
- **Both**: Use CLI for management and desktop app for expansion in any application

Try out the example files and customize them for your workflow!