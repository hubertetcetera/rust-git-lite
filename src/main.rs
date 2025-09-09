#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use clap::Parser;

mod consts;

#[derive(Debug, Parser)]
#[command(about = consts::ABOUT_CLI)]
enum Command {
    #[command(about = consts::ABOUT_INIT)]
	Init,
    #[command(about = consts::ABOUT_CATFILE)]
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
