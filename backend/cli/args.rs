use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "Typely - Text expansion made easy", long_about = None)]
pub struct TypelyArgs {
    #[command(subcommand)]
    pub command: TypelyCommand,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Database file path (optional)
    #[arg(short, long)]
    pub database: Option<String>,
}

#[derive(Subcommand)]
pub enum TypelyCommand {
    /// Add a new snippet
    Add {
        /// Trigger text (e.g., "::hello")
        trigger: String,
        /// Replacement text
        replacement: String,
        /// Tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
    },

    /// Remove a snippet by trigger
    Remove {
        /// Trigger text to remove
        trigger: String,
    },

    /// List all snippets
    List {
        /// Search term to filter snippets
        #[arg(short, long)]
        search: Option<String>,
        /// Show only active snippets
        #[arg(short, long)]
        active: bool,
        /// Show only inactive snippets
        #[arg(short = 'i', long)]
        inactive: bool,
        /// Filter by tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Sort by (trigger, created, updated, usage)
        #[arg(long, default_value = "updated")]
        sort: String,
        /// Sort order (asc, desc)
        #[arg(long, default_value = "desc")]
        order: String,
    },

    /// Show snippet details
    Show {
        /// Trigger text to show
        trigger: String,
    },

    /// Update a snippet
    Update {
        /// Trigger text to update
        trigger: String,
        /// New replacement text
        #[arg(short, long)]
        replacement: Option<String>,
        /// New trigger text
        #[arg(short = 'T', long)]
        new_trigger: Option<String>,
        /// New tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        /// Activate the snippet
        #[arg(long)]
        activate: bool,
        /// Deactivate the snippet
        #[arg(long)]
        deactivate: bool,
    },

    /// Import snippets from a file
    Import {
        /// JSON file to import from
        file: String,
        /// Overwrite existing snippets
        #[arg(short, long)]
        overwrite: bool,
    },

    /// Export snippets to a file
    Export {
        /// JSON file to export to
        file: String,
        /// Include inactive snippets
        #[arg(short, long)]
        inactive: bool,
        /// Export only specific tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
    },

    /// Test snippet expansion
    Expand {
        /// Trigger text to expand
        trigger: String,
    },

    /// Search for snippets
    Search {
        /// Search query
        query: String,
        /// Limit number of results
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },

    /// Show statistics
    Stats,
}

impl TypelyArgs {
    pub fn parse_tags(tags_str: &str) -> Vec<String> {
        tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}