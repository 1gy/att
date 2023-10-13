mod commands;
mod config;
mod contest;
mod workspace;

pub use crate::commands::*;
pub use crate::config::*;
pub use crate::contest::*;
pub use crate::workspace::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
