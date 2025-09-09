use clap::Parser;

mod consts;
mod helpers;

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
		Command::Init => helpers::init(),
		Command::CatFile => helpers::cat_file(),
	}
}
