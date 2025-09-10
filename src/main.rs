use clap::Parser;

mod helpers;

/// A minimal git cli implemented in Rust
#[derive(Debug, Parser)]
enum Command {
    /// Create an empty Git repository or reinitialize an existing one
	Init,
	/// Provide content or type and size information for repository objects
	CatFile,
}

fn main() {
	let cmd = Command::parse();

	match cmd {
		Command::Init => helpers::init(),
		Command::CatFile => helpers::cat_file(),
	}
}
