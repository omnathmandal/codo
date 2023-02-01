use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new todo
    #[command(arg_required_else_help = true)]
    Add { text: Option<String> },

    /// Remove an existing todo
    #[command(arg_required_else_help = true)]
    Remove { index: Option<u32> },

    /// Check off an existing todo
    #[command(arg_required_else_help = true)]
    Done { index: Option<u32> },

    /// Modify an existing todo
    #[command(arg_required_else_help = true)]
    Change {
        /// The index of the todo to be modified
        index: Option<u32>,
        /// New todo text
        new: Option<String>,
    },

    /// Clears all todos
    Clear,

    /// Display all todos
    Show,

    /// Display incomplete todos
    Inc,

    /// Display completed todos
    Comp,
}
