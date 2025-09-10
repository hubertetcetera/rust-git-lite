use std::fs;

use crate::commands::CatFileArgs;

pub fn init() {
	fs::create_dir(".git").unwrap();
	fs::create_dir(".git/objects").unwrap();
	fs::create_dir(".git/refs").unwrap();
	fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
	println!("Initialized git directory")
}

pub fn cat_file(args: CatFileArgs) {
	eprintln!("\nrunning cat-file with args: {:#?}\n", args);
	let (dir, file) = args.object.split_at(2);
	let path = ".git/objects/".to_string() + dir + "/" + file;
	let Ok(buf) = fs::read(&path).map_err(|e| eprintln!("Error while reading {path}: {e}\n"))
	else {
		return;
	};
	println!("{:?}", buf)
}
