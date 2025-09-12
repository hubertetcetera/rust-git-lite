use std::{fmt::Display, ops::Deref};

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
