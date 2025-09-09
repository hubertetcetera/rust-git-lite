#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "A minimal git cli implemented in Rust")]
enum Command {
    #[command(about = "Create an empty Git repository or reinitialize an existing one")]
	Init,
    #[command(about = "Provide content or type and size information for repository objects")]
	CatFile,
}

fn main() {
    let cmd = Command::parse();

    match cmd {
        Command::Init => init(),
        Command::CatFile => cat_file(),
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
