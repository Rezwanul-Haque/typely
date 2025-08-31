# Getting Started with Typely

Welcome to Typely! This guide will help you get up and running with text expansion in minutes.

## 📥 Installation

### Quick Install (Recommended)

```bash
curl -fsSL https://typely.sh/install | sh
```

### Build from Source

```bash
git clone https://github.com/typely/typely.git
cd typely
make install
```

### Platform-Specific

#### Linux
```bash
# Install dependencies
sudo apt-get install build-essential libssl-dev libsqlite3-dev

# Build and install
make install
```

#### macOS
```bash
# Install dependencies
brew install sqlite

# Build and install
make install
```

## 🚀 First Steps

### 1. Verify Installation

```bash
typely-cli --version
```

### 2. Add Your First Snippet

```bash
typely-cli add "::hello" "Hello, World!"
```

### 3. Test the Snippet

```bash
typely-cli expand "::hello"
# Output: Hello, World!
```

### 4. List Your Snippets

```bash
typely-cli list
```

## 📝 Quick Examples

### Import Sample Data

```bash
# Import basic examples
typely-cli import examples/basic-snippets.json

# Import work templates
typely-cli import examples/work-snippets.json
```

### Common Use Cases

```bash
# Email signature
typely-cli add "::sig" "Best regards,\n{user}\nSoftware Engineer"

# Current date
typely-cli add "::date" "Today is {date}"

# Address
typely-cli add "::addr" "123 Main St, Anytown, USA"

# Phone number
typely-cli add "::phone" "555-123-4567"
```

## 🖥️ Choose Your Mode

### CLI Mode (Management)
Perfect for:
- Adding/editing snippets
- Bulk operations (import/export)
- Automation and scripting
- Testing expansions

```bash
typely-cli add "::example" "This is an example"
typely-cli list --active
typely-cli export backup.json
```

### Desktop Mode (Live Expansion)
Perfect for:
- Real-time text expansion in any app
- System-wide snippets
- Seamless workflow integration

```bash
# Start desktop app
typely

# Or run in background
typely --tray
```

## 🎯 Your First Workflow

1. **Import examples** to see what's possible:
   ```bash
   typely-cli import examples/basic-snippets.json
   ```

2. **Add personal snippets**:
   ```bash
   typely-cli add "::myemail" "your.email@example.com"
   typely-cli add "::myaddr" "Your Address Here"
   ```

3. **Test them**:
   ```bash
   typely-cli expand "::myemail"
   typely-cli expand "::myaddr"
   ```

4. **Start desktop app** for live expansion:
   ```bash
   typely
   ```

5. **Type in any application**: `::myemail` → expands to your email!

## 📋 Essential Commands

### Adding Snippets
```bash
# Simple snippet
typely-cli add "::trigger" "replacement text"

# With tags for organization
typely-cli add "::work-email" "work@company.com" --tags "work,email"

# Multi-line snippet
typely-cli add "::signature" "Best regards,\n{user}\nTitle\nCompany"
```

### Managing Snippets
```bash
# List all
typely-cli list

# Search
typely-cli search "email"

# Update
typely-cli update "::trigger" --replacement "new text"

# Remove
typely-cli remove "::old-trigger"
```

### Backup & Sync
```bash
# Export all snippets
typely-cli export my-snippets.json

# Import snippets
typely-cli import shared-snippets.json

# Export specific tags
typely-cli export work-snippets.json --tags "work"
```

## 🔧 Configuration

### Database Location
By default, Typely stores data in:
- **Linux**: `~/.local/share/typely/snippets.db`
- **macOS**: `~/Library/Application Support/typely/snippets.db`
- **Windows**: `%APPDATA%/typely/snippets.db`

### Custom Database
```bash
typely-cli --database /path/to/custom.db list
```

## 💡 Pro Tips

### Naming Conventions
```bash
# Use consistent prefixes
::email-work
::email-personal
::addr-home
::addr-work

# Be descriptive
::meeting-template  # Better than ::mt
::bug-report       # Better than ::br
```

### Use Placeholders
```bash
# Dynamic content
typely-cli add "::date" "Today is {date}"
typely-cli add "::time" "Current time: {time}"
typely-cli add "::sig" "Best regards,\n{user}"
```

### Organize with Tags
```bash
# Add with tags
typely-cli add "::bug-template" "..." --tags "work,github,template"

# Filter by tags
typely-cli list --tags "work"
typely-cli export work-backup.json --tags "work"
```

## 🆘 Need Help?

- **Examples**: Check the [`examples/`](../examples/) directory
- **CLI Reference**: See [cli-reference.md](cli-reference.md)
- **Issues**: Visit [GitHub Issues](https://github.com/typely/typely/issues)
- **Troubleshooting**: See [troubleshooting.md](troubleshooting.md)

## 🎉 What's Next?

- Explore advanced features in [CLI Reference](cli-reference.md)
- Set up the desktop app with [Desktop Guide](desktop-guide.md)
- Learn about customization in [Configuration](configuration.md)
- Check out more examples in [`examples/`](../examples/)

Happy expanding! 🚀