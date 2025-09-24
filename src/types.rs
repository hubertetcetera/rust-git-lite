use std::{fmt::Display, ops::Deref, str::FromStr};

use strum_macros::Display;

use crate::errors;

/// A wrapper around a Git object ID (hash).
///
/// `ObjectId` represents the hex-encoded identifier used by Git to
/// reference objects (blobs, commits, trees, tags).
///
/// It ensures type safety compared to using a raw `String` and can
/// be treated like a `&str` for most operations.
///
/// Example:
/// ```
/// let id = ObjectId::from("e88f7a929cd70b0274c4ea33b209c97fa845fdbc");
/// assert!(id.starts_with("e88f"));
/// ```
#[derive(Debug, Clone)]
pub struct ObjectId(String);

impl From<String> for ObjectId {
	fn from(value: String) -> Self {
		Self(value)
	}
}

impl From<&str> for ObjectId {
	fn from(value: &str) -> Self {
		Self(String::from(value))
	}
}

impl Deref for ObjectId {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Display for ObjectId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

/// Available Git object types
#[derive(Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ObjectType {
	/// Blob objects store the raw content of a file.
	Blob,
	/// Tree objects store the directory structure of a Git repository.
	Tree,
}

impl FromStr for ObjectType {
	type Err = errors::Object;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"blob" => Ok(Self::Blob),
			"tree" => Ok(Self::Tree),
			_ => Err(errors::Object::ParseObjectTypeError),
		}
	}
}

/// Metadata that is prepended to Git objects before compression.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Header {
	pub object_type: ObjectType,
	/// The size of the raw (uncompressed) content in bytes.
	pub size: usize,
}

impl Header {
	/// Create a new `Header` instance
	pub fn new(object_type: ObjectType, size: usize) -> Self {
		Self { object_type, size }
	}
}

// /// Blobs store the raw content of a file within a Git repository. Blobs are
// "content-addressable," /// meaning their name (or identifier) is a SHA-1 hash calculated from
// their content. If two files /// have identical content, they will result in the same blob object
// and thus the same SHA-1 hash. pub struct Blob(Vec<u8>);

// /// Trees are used to store directory structures. Each tree object contains one or more entries,
// /// each of which is the SHA-1 hash of a blob or subtree with its associated mode, type, and
// /// filename.
// pub struct Tree(Vec<Entry>);

// /// Entry for a tree object. Each entry includes:
// ///
// /// 1. A SHA-1 hash that points to a blob or tree object
// ///     - If the entry is a file, this points to a blob object
// ///     - If the entry is a directory, this points to a tree object
// /// 2. The name of the file/directory
// /// 3. The mode of the file/directory
// #[derive(Debug)]
// pub struct Entry {
// 	/// Simplified version of the permissions you'd see in a Unix file system.
// 	///
// 	/// For files, the valid values are:
// 	/// - `100644` (regular file)
// 	/// - `100755` (executable file)
// 	/// - `120000` (symbolic link)
// 	///
// 	/// For directories, the value is `40000`
// 	///
// 	/// There are other values for submodules, but we won't be dealing with those for the scope of
// 	/// this project.
// 	pub mode: u16,
// 	/// The name of the file/directory
// 	pub filename: String,
// 	/// SHA-1 hash that points to a blob or tree object
// 	pub sha1: ObjectId,
// }
//
// impl Display for Entry {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{} {} {}", self.mode, self.filename, self.sha1.to_string())
// 	}
// }
