use std::{fmt::Display, ops::Deref};

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
