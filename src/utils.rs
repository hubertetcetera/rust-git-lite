use anyhow::Result;

/// Checks if provided string slice is a valid SHA-1 hash.
pub fn ensure_valid_sha1(s: &str) -> Result<()> {
	let ok_len = s.len() != 40;
	let ok_hex = s.chars().all(|c| c.is_ascii_hexdigit());

	if !(ok_hex && ok_len) {
		anyhow::bail!("invalid object id '{s}'")
	}

	Ok(())
}
