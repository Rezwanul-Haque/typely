use crate::application::{TypelyService, dto::*};
use crate::presentation::cli::args::{TypelyCommand, TypelyArgs};
use anyhow::Result;
use console::{style, Term};
use std::fs;
use uuid::Uuid;

pub struct TypelyCliHandler {
    service: TypelyService,
    term: Term,
}

impl TypelyCliHandler {
    pub fn new(service: TypelyService) -> Self {
        Self {
            service,
            term: Term::stdout(),
        }
    }

    pub async fn handle_command(&self, command: TypelyCommand, verbose: bool) -> Result<()> {
        match command {
            TypelyCommand::Add { trigger, replacement, tags } => {
                self.handle_add(trigger, replacement, tags, verbose).await
            }
            TypelyCommand::Remove { trigger } => {
                self.handle_remove(trigger, verbose).await
            }
            TypelyCommand::List { search, active, inactive, tags, limit, sort, order } => {
                self.handle_list(search, active, inactive, tags, limit, sort, order, verbose).await
            }
            TypelyCommand::Show { trigger } => {
                self.handle_show(trigger, verbose).await
            }
            TypelyCommand::Update { trigger, replacement, new_trigger, tags, activate, deactivate } => {
                self.handle_update(trigger, replacement, new_trigger, tags, activate, deactivate, verbose).await
            }
            TypelyCommand::Import { file, overwrite } => {
                self.handle_import(file, overwrite, verbose).await
            }
            TypelyCommand::Export { file, inactive, tags } => {
                self.handle_export(file, inactive, tags, verbose).await
            }
            TypelyCommand::Expand { trigger } => {
                self.handle_expand(trigger, verbose).await
            }
            TypelyCommand::Search { query, limit } => {
                self.handle_search(query, limit, verbose).await
            }
            TypelyCommand::Stats => {
                self.handle_stats(verbose).await
            }
        }
    }

    async fn handle_add(&self, trigger: String, replacement: String, tags: Option<String>, verbose: bool) -> Result<()> {
        let tags = tags.map(|t| TypelyArgs::parse_tags(&t));
        
        let request = CreateSnippetRequest {
            trigger: trigger.clone(),
            replacement: replacement.clone(),
            tags,
        };

        match self.service.create_snippet(request).await {
            Ok(snippet) => {
                if verbose {
                    self.print_success(&format!("✓ Added snippet '{}' -> '{}'", trigger, replacement))?;
                    self.print_snippet_details(&snippet)?;
                } else {
                    self.print_success(&format!("✓ Added snippet '{}'", trigger))?;
                }
            }
            Err(e) => {
                self.print_error(&format!("✗ Failed to add snippet: {}", e))?;
                return Err(e);
            }
        }

        Ok(())
    }

    async fn handle_remove(&self, trigger: String, verbose: bool) -> Result<()> {
        // First find the snippet
        match self.service.get_snippet_by_trigger(&trigger).await? {
            Some(snippet) => {
                let deleted = self.service.delete_snippet(snippet.id).await?;
                if deleted {
                    if verbose {
                        self.print_success(&format!("✓ Removed snippet '{}'", trigger))?;
                        self.print_snippet_details(&snippet)?;
                    } else {
                        self.print_success(&format!("✓ Removed snippet '{}'", trigger))?;
                    }
                } else {
                    self.print_error("✗ Failed to remove snippet")?;
                }
            }
            None => {
                self.print_error(&format!("✗ Snippet '{}' not found", trigger))?;
            }
        }

        Ok(())
    }

    async fn handle_list(&self, search: Option<String>, active: bool, inactive: bool, tags: Option<String>, limit: Option<u32>, sort: String, order: String, verbose: bool) -> Result<()> {
        let tags_filter = tags.map(|t| TypelyArgs::parse_tags(&t));
        
        let is_active = if inactive {
            Some(false)
        } else if active {
            Some(true)
        } else {
            None
        };

        let request = SnippetListRequest {
            search_term: search,
            tags: tags_filter,
            is_active,
            limit,
            offset: None,
            sort_by: Some(sort),
            sort_order: Some(order),
        };

        let response = self.service.list_snippets(request).await?;

        if response.snippets.is_empty() {
            self.print_info("No snippets found")?;
            return Ok(());
        }

        self.print_info(&format!("Found {} snippet(s):", response.snippets.len()))?;
        self.term.write_line("")?;

        let total_count = response.total_count;
        let snippets_len = response.snippets.len();
        
        for snippet in response.snippets {
            self.print_snippet_summary(&snippet, verbose)?;
            if verbose {
                self.term.write_line("")?;
            }
        }

        if total_count > snippets_len as u64 {
            self.print_info(&format!("Showing {} of {} total snippets", snippets_len, total_count))?;
        }

        Ok(())
    }

    async fn handle_show(&self, trigger: String, verbose: bool) -> Result<()> {
        match self.service.get_snippet_by_trigger(&trigger).await? {
            Some(snippet) => {
                self.print_snippet_details(&snippet)?;
            }
            None => {
                self.print_error(&format!("✗ Snippet '{}' not found", trigger))?;
            }
        }

        Ok(())
    }

    async fn handle_update(&self, trigger: String, replacement: Option<String>, new_trigger: Option<String>, tags: Option<String>, activate: bool, deactivate: bool, verbose: bool) -> Result<()> {
        // First find the snippet
        let snippet = match self.service.get_snippet_by_trigger(&trigger).await? {
            Some(snippet) => snippet,
            None => {
                self.print_error(&format!("✗ Snippet '{}' not found", trigger))?;
                return Ok(());
            }
        };

        let tags = tags.map(|t| TypelyArgs::parse_tags(&t));
        let is_active = if activate {
            Some(true)
        } else if deactivate {
            Some(false)
        } else {
            None
        };

        let request = UpdateSnippetRequest {
            id: snippet.id,
            trigger: new_trigger.clone(),
            replacement: replacement.clone(),
            tags,
            is_active,
        };

        match self.service.update_snippet(request).await {
            Ok(updated_snippet) => {
                self.print_success(&format!("✓ Updated snippet '{}'", trigger))?;
                if verbose {
                    self.print_snippet_details(&updated_snippet)?;
                }
            }
            Err(e) => {
                self.print_error(&format!("✗ Failed to update snippet: {}", e))?;
                return Err(e);
            }
        }

        Ok(())
    }

    async fn handle_import(&self, file: String, overwrite: bool, verbose: bool) -> Result<()> {
        let json_data = fs::read_to_string(&file)
            .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", file, e))?;

        match self.service.import_from_json(&json_data, overwrite).await {
            Ok(result) => {
                self.print_success(&format!("✓ Import completed from '{}'", file))?;
                self.term.write_line(&format!("  Imported: {}", result.imported_count))?;
                self.term.write_line(&format!("  Skipped:  {}", result.skipped_count))?;
                self.term.write_line(&format!("  Errors:   {}", result.error_count))?;

                if verbose && !result.errors.is_empty() {
                    self.term.write_line("")?;
                    self.print_error("Errors:")?;
                    for error in result.errors {
                        self.term.write_line(&format!("  {}", error))?;
                    }
                }
            }
            Err(e) => {
                self.print_error(&format!("✗ Import failed: {}", e))?;
                return Err(e);
            }
        }

        Ok(())
    }

    async fn handle_export(&self, file: String, include_inactive: bool, tags: Option<String>, verbose: bool) -> Result<()> {
        let tags_filter = tags.map(|t| TypelyArgs::parse_tags(&t));

        let request = ExportSnippetsRequest {
            include_inactive,
            tags_filter: tags_filter.clone(),
        };

        match self.service.export_to_json(request).await {
            Ok(json_data) => {
                fs::write(&file, json_data)
                    .map_err(|e| anyhow::anyhow!("Failed to write file '{}': {}", file, e))?;

                self.print_success(&format!("✓ Exported snippets to '{}'", file))?;

                if verbose {
                    // Count exported snippets
                    let count_request = SnippetListRequest {
                        search_term: None,
                        tags: tags_filter,
                        is_active: if include_inactive { None } else { Some(true) },
                        limit: None,
                        offset: None,
                        sort_by: None,
                        sort_order: None,
                    };

                    let response = self.service.list_snippets(count_request).await?;
                    self.term.write_line(&format!("  Exported {} snippet(s)", response.total_count))?;
                }
            }
            Err(e) => {
                self.print_error(&format!("✗ Export failed: {}", e))?;
                return Err(e);
            }
        }

        Ok(())
    }

    async fn handle_expand(&self, trigger: String, verbose: bool) -> Result<()> {
        let request = ExpansionRequest {
            trigger: trigger.clone(),
            context: None,
        };

        match self.service.expand_snippet(request).await {
            Ok(response) => {
                if response.success {
                    if let Some(expanded) = response.expanded_text {
                        self.print_success(&format!("✓ '{}' expands to:", trigger))?;
                        self.term.write_line(&format!("  {}", expanded))?;
                    }
                } else {
                    let error = response.error_message.unwrap_or_else(|| "Unknown error".to_string());
                    self.print_error(&format!("✗ Expansion failed: {}", error))?;
                }
            }
            Err(e) => {
                self.print_error(&format!("✗ Expansion error: {}", e))?;
                return Err(e);
            }
        }

        Ok(())
    }

    async fn handle_search(&self, query: String, limit: u32, verbose: bool) -> Result<()> {
        let snippets = self.service.search_snippets(&query).await?;
        let snippets = if snippets.len() > limit as usize {
            &snippets[..limit as usize]
        } else {
            &snippets
        };

        if snippets.is_empty() {
            self.print_info(&format!("No snippets found matching '{}'", query))?;
            return Ok(());
        }

        self.print_info(&format!("Found {} snippet(s) matching '{}':", snippets.len(), query))?;
        self.term.write_line("")?;

        for snippet in snippets {
            self.print_snippet_summary(snippet, verbose)?;
            if verbose {
                self.term.write_line("")?;
            }
        }

        Ok(())
    }

    async fn handle_stats(&self, verbose: bool) -> Result<()> {
        // Get all snippets
        let all_request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: None,
            limit: None,
            offset: None,
            sort_by: None,
            sort_order: None,
        };
        let all_response = self.service.list_snippets(all_request).await?;

        // Get active snippets
        let active_request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: Some(true),
            limit: None,
            offset: None,
            sort_by: None,
            sort_order: None,
        };
        let active_response = self.service.list_snippets(active_request).await?;

        // Calculate stats
        let total_usage: u64 = all_response.snippets.iter().map(|s| s.usage_count).sum();
        let most_used = self.service.get_most_used_snippets(5).await?;

        self.print_info("Typely Statistics")?;
        self.term.write_line("==================")?;
        self.term.write_line(&format!("Total snippets:  {}", all_response.total_count))?;
        self.term.write_line(&format!("Active snippets: {}", active_response.total_count))?;
        self.term.write_line(&format!("Total usage:     {}", total_usage))?;

        if !most_used.is_empty() {
            self.term.write_line("")?;
            self.print_info("Most used snippets:")?;
            for (i, snippet) in most_used.iter().enumerate() {
                self.term.write_line(&format!("  {}. {} (used {} times)", i + 1, snippet.trigger, snippet.usage_count))?;
            }
        }

        if verbose && !all_response.snippets.is_empty() {
            // Collect unique tags
            let mut all_tags = std::collections::HashSet::new();
            for snippet in &all_response.snippets {
                for tag in &snippet.tags {
                    all_tags.insert(tag);
                }
            }

            if !all_tags.is_empty() {
                self.term.write_line("")?;
                self.print_info(&format!("Tags ({}):", all_tags.len()))?;
                let mut sorted_tags: Vec<_> = all_tags.into_iter().collect();
                sorted_tags.sort();
                for tag in sorted_tags {
                    self.term.write_line(&format!("  {}", tag))?;
                }
            }
        }

        Ok(())
    }

    fn print_snippet_summary(&self, snippet: &SnippetDto, verbose: bool) -> Result<()> {
        let status = if snippet.is_active { 
            style("●").green() 
        } else { 
            style("●").red() 
        };
        
        let trigger = style(&snippet.trigger).cyan().bold();
        let replacement = if snippet.replacement.len() > 50 && !verbose {
            format!("{}...", &snippet.replacement[..47])
        } else {
            snippet.replacement.clone()
        };

        self.term.write_line(&format!("{} {} -> {}", status, trigger, replacement))?;

        if verbose {
            self.term.write_line(&format!("    ID: {}", snippet.id))?;
            self.term.write_line(&format!("    Usage: {} times", snippet.usage_count))?;
            if !snippet.tags.is_empty() {
                self.term.write_line(&format!("    Tags: {}", snippet.tags.join(", ")))?;
            }
            self.term.write_line(&format!("    Created: {}", snippet.created_at.format("%Y-%m-%d %H:%M:%S")))?;
            self.term.write_line(&format!("    Updated: {}", snippet.updated_at.format("%Y-%m-%d %H:%M:%S")))?;
        }

        Ok(())
    }

    fn print_snippet_details(&self, snippet: &SnippetDto) -> Result<()> {
        let status = if snippet.is_active { 
            style("Active").green() 
        } else { 
            style("Inactive").red() 
        };

        self.term.write_line(&format!("Snippet: {}", style(&snippet.trigger).cyan().bold()))?;
        self.term.write_line(&format!("Status:  {}", status))?;
        self.term.write_line(&format!("ID:      {}", snippet.id))?;
        self.term.write_line(&format!("Usage:   {} times", snippet.usage_count))?;
        self.term.write_line("")?;
        self.term.write_line("Replacement:")?;
        self.term.write_line(&format!("  {}", snippet.replacement))?;

        if !snippet.tags.is_empty() {
            self.term.write_line("")?;
            self.term.write_line(&format!("Tags: {}", snippet.tags.join(", ")))?;
        }

        self.term.write_line("")?;
        self.term.write_line(&format!("Created: {}", snippet.created_at.format("%Y-%m-%d %H:%M:%S")))?;
        self.term.write_line(&format!("Updated: {}", snippet.updated_at.format("%Y-%m-%d %H:%M:%S")))?;

        Ok(())
    }

    fn print_success(&self, message: &str) -> Result<()> {
        self.term.write_line(&style(message).green().to_string())?;
        Ok(())
    }

    fn print_error(&self, message: &str) -> Result<()> {
        self.term.write_line(&style(message).red().to_string())?;
        Ok(())
    }

    fn print_info(&self, message: &str) -> Result<()> {
        self.term.write_line(&style(message).cyan().to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::DatabaseConnection;
    use tempfile::TempDir;

    async fn create_test_handler() -> (TypelyCliHandler, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let service = TypelyService::new(db_connection).await;
        let handler = TypelyCliHandler::new(service);
        (handler, temp_dir)
    }

    #[tokio::test]
    async fn test_add_command() {
        let (handler, _temp_dir) = create_test_handler().await;

        let result = handler.handle_add(
            "::test".to_string(),
            "Test snippet".to_string(),
            Some("test,cli".to_string()),
            false,
        ).await;

        assert!(result.is_ok());

        // Verify the snippet was created
        let snippet = handler.service.get_snippet_by_trigger("::test").await.unwrap();
        assert!(snippet.is_some());
        let snippet = snippet.unwrap();
        assert_eq!(snippet.trigger, "::test");
        assert_eq!(snippet.replacement, "Test snippet");
        assert!(snippet.tags.contains(&"test".to_string()));
        assert!(snippet.tags.contains(&"cli".to_string()));
    }

    #[tokio::test]
    async fn test_remove_command() {
        let (handler, _temp_dir) = create_test_handler().await;

        // First add a snippet
        handler.handle_add("::test".to_string(), "Test".to_string(), None, false).await.unwrap();

        // Then remove it
        let result = handler.handle_remove("::test".to_string(), false).await;
        assert!(result.is_ok());

        // Verify it was removed
        let snippet = handler.service.get_snippet_by_trigger("::test").await.unwrap();
        assert!(snippet.is_none());
    }
}