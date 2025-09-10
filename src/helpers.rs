use std::{
	fs,
	io::{Cursor, Read},
};

use flate2::read::ZlibDecoder;

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
	let path = format!(".git/objects/{dir}/{file}");
	let Ok(compressed) =
		fs::read(&path).map_err(|e| eprintln!("Error while reading {path}: {e}\n"))
	else {
		return;
	};
	let mut decoder = ZlibDecoder::new(Cursor::new(compressed));
	let mut content = String::new();
	let Ok(_) = decoder
		.read_to_string(&mut content)
		.map_err(|e| eprintln!("Error while writing content to string: {e}"))
	else {
		return;
	};
	let Some(null_pos) = content.find('\0') else {
		eprintln!("Failed to find null_pos: `content.find('\\0')` returned `None`");
		return;
	};
	print!("{}", content.split_off(null_pos + 1))
}
