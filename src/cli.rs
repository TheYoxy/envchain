use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "envchain")]
#[command(version, about = "A CLI used to set/get credentials from secure storage", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  /// Generate shell completions
  Completions {
    /// the shell to generate completions for
    #[arg(value_enum)]
    shell: clap_complete_command::Shell,
  },

  /// Get a secret
  #[command(arg_required_else_help = true)]
  Get {
    /// Name of the secret to get
    name: String,
  },

  /// Set a secret
  #[command(arg_required_else_help = true)]
  Set {
    /// Name of the secret to set
    name: String,
    /// Value of the secret to store
    value: String,
    /// Should override the current value
    #[arg(long, short = 'f', default_value = "false")]
    force: bool,
  },

  /// Delete a secret from the keyring
  #[command(arg_required_else_help = true)]
  Delete {
    /// Name of the secret to delete
    name: String,
  },
}
