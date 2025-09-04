// Backend library for Typely text expansion application

pub mod app;
pub mod domain;
pub mod infra;

// Re-export commonly used types
pub use app::*;
pub use domain::entities::*;
pub use domain::repositories as domain_repositories;
pub use infra::repositories as infra_repositories;
pub use infra::*;
