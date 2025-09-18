use crate::{
	commands::{CatFileArgs, HashObjectArgs},
	utils::ensure_valid_sha1,
};
use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;
use std::{fs, io::Read, path::PathBuf};

/// Initializes a Git repository if one doesn't exist already.
pub fn init() -> Result<()> {
	let git = PathBuf::from(".git");

	fs::create_dir_all(git.join("objects")).context("create .git/objects")?;
	fs::create_dir_all(git.join("refs")).context("create .git/refs")?;
	fs::write(git.join("HEAD"), "ref: refs/heads/main\n").context("write .git/HEAD")?;
	println!("Initialized git directory");

	Ok(())
}

/// Print the contents of a Git object by its hash.
///
/// Looks up the object in `.git/objects` using the standard Git layout,
/// decompresses it, strips the header, and prints the body (file contents,
/// commit text, etc.) to stdout.
///
/// Errors are reported to stderr if the file is missing, corrupt, or malformed.
pub fn cat_file(args: CatFileArgs) -> Result<()> {
	ensure_valid_sha1(&args.object)?;

	// Git derives the path to an object from its hash.
	//
	// For example, the path for the object with the hash `e88f7a929cd70b0274c4ea33b209c97fa845fdbc`
	// would be: `.git/objects/e8/8f7a929cd70b0274c4ea33b209c97fa845fdbc`
	let (dir, file) = args.object.split_at(2);
	let path = PathBuf::from(".git").join("objects").join(dir).join(file); // Derive the object path using its hash

	let compressed =
		fs::read(&path).with_context(|| format!("read object file at '{}'", path.display()))?;
	let mut decoder = ZlibDecoder::new(&compressed[..]);
	let mut content = String::new();
	decoder
		.read_to_string(&mut content)
		.with_context(|| format!("decompress object at '{}'", path.display()))?;

	// The format of a blob object file looks like this (after Zlib decompression):
	// ```
	//  blob <size>\0<content>
	// ```
	// `<size>` is the size of the content (in bytes)
	// `\0` is a null byte
	// `<content>` is the actual content of the file
	let null_pos = content.find('\0').with_context(|| "find NUL after header:")?; // Get the position of the null byte (`\0`)
	print!("{}", content.split_off(null_pos + 1)); // then split off everything before the content (including the null byte)

	Ok(())
}

/// TODO: Add documentation
pub fn hash_object(_args: HashObjectArgs) -> Result<()> {
	todo!()
}
