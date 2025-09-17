use crate::{
	commands::{CatFileArgs, HashObjectArgs},
	utils::is_valid_sha1,
};
use flate2::read::ZlibDecoder;
use std::{
	fs,
	io::{Cursor, Read},
};

/// Initializes a Git repository if one doesn't exist already. Panics on failure.
pub fn init() {
	fs::create_dir(".git").unwrap();
	fs::create_dir(".git/objects").unwrap();
	fs::create_dir(".git/refs").unwrap();
	fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
	println!("Initialized git directory")
}

/// Print the contents of a Git object by its hash.
///
/// Looks up the object in `.git/objects` using the standard Git layout,
/// decompresses it, strips the header, and prints the body (file contents,
/// commit text, etc.) to stdout.
///
/// Errors are reported to stderr if the file is missing, corrupt, or malformed.
///
/// TODO: handle errors properly with enums
pub fn cat_file(args: CatFileArgs) {
	if !is_valid_sha1(&args.object.to_string()) {
		return eprintln!("<object> must be a valid SHA-1 hash");
	}

	// Git derives the path to an object from its hash.
	//
	// For example, the path for the object with the hash `e88f7a929cd70b0274c4ea33b209c97fa845fdbc`
	// would be: `.git/objects/e8/8f7a929cd70b0274c4ea33b209c97fa845fdbc`
	let (dir, file) = args.object.split_at(2);
	let path = format!(".git/objects/{dir}/{file}"); // Derive the object path using its hash

	if let Ok(compressed) =
		fs::read(&path).map_err(|e| eprintln!("Error while reading {path}: {e}\n"))
	{
		let mut decoder = ZlibDecoder::new(Cursor::new(compressed));
		let mut content = String::new();
		if let Ok(size) = decoder.read_to_string(&mut content).map_err(|e| {
			eprintln!("Error while reading content from `ZlibDecorder` to `String`: {e}")
		}) {
			log::info!("Successfully read {size} bytes from decoder into content")
		}

		// The format of a blob object file looks like this (after Zlib decompression):
		// ```
		//  blob <size>\0<content>
		// ```
		// `<size>` is the size of the content (in bytes)
		// `\0` is a null byte
		// `<content>` is the actual content of the file
		let Some(null_pos) = content.find('\0') else {
			return eprintln!("Failed to find null_pos: `content.find('\\0')` returned `None`");
		}; // Get the position of the null byte (`\0`)
		print!("{}", content.split_off(null_pos + 1)) // then split off everything before the content
		                                        // (including the null byte)
	}
}

/// TODO: Add documentation
pub fn hash_object(_args: HashObjectArgs) {
	todo!()
}
