use clap::{Args, Parser};

use crate::types::ObjectId;

/// A minimal git cli implemented in Rust
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub enum Command {
	/// Create an empty Git repository or reinitialize an existing one
	Init,
	/// Provide content or type and size information for repository objects
	CatFile(CatFileArgs),
}

/// Command-line arguments for the `cat-file` subcommand.
#[derive(Debug, Args)]
pub struct CatFileArgs {
	/// Pretty-print the contents of <object> based on its type
	#[arg(short = 'p')]
	pretty: bool,
	/// The name of the object to show
	#[arg(required = true)]
	pub object: ObjectId,
}
