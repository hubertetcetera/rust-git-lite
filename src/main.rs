mod commands;
mod helpers;

use clap::Parser;
use commands::Command;

fn main() {
	let cmd = Command::parse();

	match cmd {
		Command::Init => helpers::init(),
		Command::CatFile(args) => helpers::cat_file(args),
	}
}
