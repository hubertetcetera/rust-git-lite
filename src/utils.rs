use crate::types::ObjectId;
use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;
use std::{fs, io::Read, path::PathBuf};

/// Checks if provided string slice is a valid SHA-1 hash.
pub fn ensure_valid_sha1(s: &str) -> Result<()> {
	let ok_len = s.len() == 40;
	let ok_hex = s.chars().all(|c| c.is_ascii_hexdigit());

	if !(ok_hex && ok_len) {
		anyhow::bail!("invalid object id '{s}'")
	}

	Ok(())
}

/// Derives the path to an object from its hash.
pub fn get_path_from_hash(hash: ObjectId) -> Result<PathBuf> {
	ensure_valid_sha1(&hash.to_string())?;
	// Git derives the path to an object from its hash.
	//
	// For example, the path for the object with the hash `e88f7a929cd70b0274c4ea33b209c97fa845fdbc`
	// would be: `.git/objects/e8/8f7a929cd70b0274c4ea33b209c97fa845fdbc`
	let (dir, file) = hash.split_at(2);
	Ok(PathBuf::from(".git").join("objects").join(dir).join(file))
}

/// Decompress file at given path if it exists.
pub fn zlib_decode(path: &PathBuf) -> Result<String> {
	let compressed =
		fs::read(&path).with_context(|| format!("read object file at '{}'", path.display()))?;
	let mut decoder = ZlibDecoder::new(&compressed[..]);
	let mut content = String::new();
	decoder
		.read_to_string(&mut content)
		.with_context(|| format!("decompress object at '{}'", path.display()))?;

	Ok(content)
}

/// Strips the header for given git object and returns the resulting content. For example:
///
/// The format of a blob object file looks like this (after Zlib decompression):
/// ```
///  blob <size>\0<content>
/// ```
/// `<size>` is the size of the content (in bytes)
/// `\0` is a null byte
/// `<content>` is the actual content of the file which the function returns.
pub fn strip_header(content: String) -> Result<String> {
	let null_pos = content.find('\0').with_context(|| "find NUL after header:")?; // Get the position of the null byte (`\0`)
	Ok(content.clone().split_off(null_pos + 1))
}

#[cfg(test)]
mod test {
	use crate::utils::ensure_valid_sha1;

	#[test]
	fn should_pass_sha1_is_correct() {
		let result = ensure_valid_sha1("e3cf93f814459d888602bea15035f348f6208e8c");
		assert!(result.is_ok());
	}

	#[test]
	fn should_fail_sha1_is_too_short() {
		let result = ensure_valid_sha1("e3cf93f814459d888602bea15035f348f6208e8");
		assert!(result.is_err());
	}

	#[test]
	fn should_fail_sha1_is_too_long() {
		let result = ensure_valid_sha1("e3cf93f814459d888602bea15035f348f6208e8ce");
		assert!(result.is_err());
	}

	#[test]
	fn should_fail_sha1_contains_non_hex_char() {
		let result = ensure_valid_sha1("g3cf93f814459d888602bea15035f348f6208e8");
		assert!(result.is_err());
	}
}
