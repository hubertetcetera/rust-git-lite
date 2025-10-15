use crate::{
	commands::{CatFileArgs, HashObjectArgs, ListTreeArgs, WriteTreeArgs},
	types::ObjectId,
	utils::{get_path_from_hash, parse_content_raw_bytes, strip_header, zlib_decode},
};
use anyhow::{Context, Ok, Result};
use flate2::{write::ZlibEncoder, Compression};
use gag::BufferRedirect;
use is_executable::IsExecutable;
use sha1::{Digest, Sha1};
use std::{
	env::current_dir,
	fs,
	io::{Read, Write},
	path::PathBuf,
};

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
pub fn hash_object(args: HashObjectArgs) -> Result<String> {
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

	if !args.quiet {
		println!("{hash}");
	}
	Ok(hash)
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

/// Create a tree object from the current state of the staging area*.
///
/// *Please note that for the purpose of this minimal implementation, we don't implement a staging
/// area, we'll just assume that all files in the working directory are staged.
///
/// TODO: Refactor function
pub fn write_tree(args: WriteTreeArgs) -> Result<()> {
	let current_dir = current_dir().context("get current directory")?;
	let path = args.path.unwrap_or(current_dir);
	let mut buf = Vec::new();
	// TODO: use walkdir for cleaner traversal
	for entry in fs::read_dir(path)? {
		let path = entry?.path();
		let is_git_dir = path.components().any(|p| p.as_os_str() == ".git");
		if is_git_dir {
			continue;
		}
		let mut mode = String::new();
		let mut ascii_hash = String::new();
		if path.is_dir() {
			let args = WriteTreeArgs { path: Some(path.clone()), quiet: false };
			mode = String::from("40000");
			let mut buffer_redirect =
				BufferRedirect::stdout().context("read tree hash to buffer redirect")?;
			write_tree(args).with_context(|| format!("writing tree: {}", path.display()))?;

			buffer_redirect
				.read_to_string(&mut ascii_hash)
				.context("read hash from redirected buffer to string")?;

			let hash_bytes = hex::decode(ascii_hash.trim()).context("decode hash string to hex")?;

			let file_name = path.file_name().context("read directory name")?.to_owned();
			buf.push((mode, file_name, hash_bytes));
		} else {
			if path.is_symlink() {
				mode = String::from("120000");
			} else if path.is_executable() {
				mode = String::from("100755");
			} else if path.is_file() {
				mode = String::from("100644");
			}

			let file_hash = hash_object(HashObjectArgs::new(true, path.clone(), true))?;
			let hash_bytes = hex::decode(file_hash).context("decode file_hash to hex bytes")?;
			let name = path
				.file_name()
				.with_context(|| format!("reading file_name at: {}", path.display()))?;

			buf.push((mode, name.to_owned(), hash_bytes));
		}
	}
	buf.sort_by(|a, b| a.1.cmp(&b.1));
	let buf: Vec<Vec<u8>> = buf
		.iter()
		.map(|e| [e.0.as_bytes(), b" ", e.1.as_encoded_bytes(), b"\0", &e.2].concat())
		.collect();
	let buf = buf.concat();
	let size = buf.len();
	let header = format!("tree {size}\0");
	let contents = [header.as_bytes(), &buf].concat();
	let sha1_digest = Sha1::digest(&contents);
	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
	encoder.write_all(&contents).context("write contents to encoder buffer")?;
	let contents = encoder.finish().context("retrieve encoded content")?;
	let hash = format!("{:x}", sha1_digest);

	let file_path = get_path_from_hash(&ObjectId::from(hash.clone()))?;
	let dir_path = file_path
		.parent()
		.with_context(|| format!("get parent directory at {}", file_path.display()))?;
	fs::create_dir_all(dir_path)
		.with_context(|| format!("create directory at {}", dir_path.display()))?;
	fs::write(&file_path, contents).with_context(|| format!("write to {}", file_path.display()))?;

	if !args.quiet {
		println!("{hash}");
	}
	Ok(())
}
