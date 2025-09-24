mod commands;
mod errors;
mod helpers;
mod types;
mod utils;

use clap::Parser;
use commands::Command;

fn main() -> Result<(), anyhow::Error> {
	let cmd = Command::parse();

	match cmd {
		Command::Init => helpers::init(),
		Command::CatFile(args) => helpers::cat_file(args),
		Command::HashObject(args) => helpers::hash_object(args),
		Command::ListTree(args) => helpers::ls_tree(args),
	}
}
