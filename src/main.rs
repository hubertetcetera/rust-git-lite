mod commands;
mod errors;
mod helpers;
mod types;
mod utils;

use anyhow::Ok;
use clap::Parser;
use commands::Command;

fn main() -> Result<(), anyhow::Error> {
	let cmd = Command::parse();

	match cmd {
		Command::Init => helpers::init(),
		Command::CatFile(args) => helpers::cat_file(args),
		Command::HashObject(args) => {
			let _ = helpers::hash_object(args)?;
			Ok(())
		},
		Command::ListTree(args) => helpers::ls_tree(args),
		Command::WriteTree(args) => helpers::write_tree(args.path),
	}
}
