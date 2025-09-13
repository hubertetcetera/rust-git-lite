/// Checks if provided string slice is a valid SHA-1 hash. Returns `true` if it is, `false` if not.
pub fn is_valid_sha1(s: &str) -> bool {
	if s.len() != 40 {
		return false;
	}

	s.chars().all(|c| c.is_ascii_hexdigit())
}
