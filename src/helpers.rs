use crate::{
	commands::{CatFileArgs, HashObjectArgs, ListTreeArgs},
	types::ObjectId,
	utils::{get_path_from_hash, parse_content_raw_bytes, strip_header, zlib_decode},
};
use anyhow::{Context, Result};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::Write, path::PathBuf};

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
	let path = get_path_from_hash(&args.object)?;
	let content = zlib_decode(&path).with_context(|| "decode {path} with zlib")?;

	let content = String::from_utf8(content).with_context(|| {
		format!("convert decoded content to string using utf8 at {}", path.display())
	})?;
	print!("{}", strip_header(&content).context("strip header from decoded content")?); // split off everything before the content (including the null byte)

	Ok(())
}

/// Computes the SHA-1 hash for given object. Optionally, writes the file to `.git/objects`
/// directory if used with `-w` flag.
pub fn hash_object(args: HashObjectArgs) -> Result<()> {
	let path = args.file;
	let buf = fs::read(&path).with_context(|| format!("read file at {}", path.display()))?;
	let header = format!("blob {}\0", buf.len());
	let contents = [Vec::from(header.as_bytes()), buf].concat();
	let sha1_digest = Sha1::digest(&contents);

	let hash = format!("{:x}", sha1_digest);

	if args.write {
		let file_path = get_path_from_hash(&ObjectId::from(hash.clone()))?;
		let dir_path = file_path
			.parent()
			.with_context(|| format!("get parent directory at {}", file_path.display()))?;
		fs::create_dir_all(dir_path)
			.with_context(|| format!("create directory at {}", dir_path.display()))?;
		let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
		encoder.write_all(contents.as_slice()).context("write buffer to encoder")?;
		let contents = encoder.finish().context("retrieve encoded content")?;
		fs::write(&file_path, contents)
			.with_context(|| format!("write to {}", file_path.display()))?;
	}

	println!("{hash}");
	Ok(())
}

/// Lists the contents of a tree object
pub fn ls_tree(args: ListTreeArgs) -> Result<()> {
	let path = get_path_from_hash(&args.tree_sha)?;
	let content = zlib_decode(&path)?;
	let (_, bytes) = parse_content_raw_bytes(&content)?;

	let mut entries_raw: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> = vec![];

	let mut i = 0usize;
	while i < bytes.len() {
		let rel_space = bytes[i..].iter().position(|&b| b == b' ').context("get rel_space")?;
		let mode_end = i + rel_space;
		let mode = bytes[i..mode_end].to_vec();

		let rel_nul = bytes[mode_end + 1..].iter().position(|&b| b == 0).context("get rel_nul")?;
		let name_end = mode_end + 1 + rel_nul;
		let name = bytes[mode_end + 1..name_end].to_vec();

		let sha_start = name_end + 1;
		let sha_end = name_end + 1 + 20;

		let sha = bytes[sha_start..sha_end].to_vec();

		entries_raw.push((mode, name, sha));

		i = sha_end;
	}

	for (mode, name, sha) in entries_raw {
		let name = String::from_utf8(name).context("read entry name from raw bytes")?;
		if args.name_only {
			println!("{}", name);
		} else {
			let mode = String::from_utf8(mode).context("read entry mode from raw bytes")?;
			println!("{} {} {}", mode, name, hex::encode(&sha))
		}
	}
	Ok(())
}
