#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Default, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum Command {
	Init,
	CatFile,
	#[default]
	Help,
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		return help()
	}

	let Ok(command) = Command::from_str(args[1].as_str()) else {
		eprintln!("Unknown command: {}", args[1]);
		return help()
	};

	match command {
		Command::Init => init(),
		Command::CatFile => cat_file(),
		Command::Help => help(),
	}
}

fn init() {
	fs::create_dir(".git").unwrap();
	fs::create_dir(".git/objects").unwrap();
	fs::create_dir(".git/refs").unwrap();
	fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
	println!("Initialized git directory")
}

fn cat_file() {
	println!("running cat-file")
}

fn help() {
	println!("Usage: rust-git-lite COMMAND <flags>")
}
