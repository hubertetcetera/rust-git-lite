use crate::types::ObjectId;
use clap::{Args, Parser};
use std::path::PathBuf;

/// A minimal git cli implemented in Rust
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub enum Command {
	/// Create an empty Git repository or reinitialize an existing one
	Init,
	/// Provide content or type and size information for repository objects
	CatFile(CatFileArgs),
	/// Compute the `ObjectId` for a given file
	HashObject(HashObjectArgs),
	/// Inspect a tree object
	#[command(name = "ls-tree")]
	ListTree,
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

/// Command-line arguments for the `hash-object` command
#[derive(Debug, Args)]
pub struct HashObjectArgs {
	/// Write `object` to the `.git/objects` directory
	#[arg(short = 'w')]
	pub write: bool,
	/// Path to the input file
	#[arg(required = true)]
	pub file: PathBuf,
}
