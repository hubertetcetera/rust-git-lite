use clap::{Args, Parser};

/// A minimal git cli implemented in Rust
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub enum Command {
	/// Create an empty Git repository or reinitialize an existing one
	Init,
	/// Provide content or type and size information for repository objects
	CatFile(CatFileArgs),
}

#[derive(Debug, Args)]
pub struct CatFileArgs {
	/// Pretty-print the contents of <object> based on its type
	#[arg(short = 'p')]
	pretty: bool,
	/// The name of the object to show
	#[arg(required = true)]
	object: String,
}

impl CatFileArgs {
	pub fn get_object(&self) -> String {
		self.object.clone()
	}
}
