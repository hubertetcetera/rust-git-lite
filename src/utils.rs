use anyhow::Result;

/// Checks if provided string slice is a valid SHA-1 hash.
pub fn ensure_valid_sha1(s: &str) -> Result<()> {
	let ok_len = s.len() == 40;
	println!("ok_len: {ok_len}");
	let ok_hex = s.chars().all(|c| c.is_ascii_hexdigit());
	println!("ok_hex: {ok_hex}");

	if !(ok_hex && ok_len) {
		anyhow::bail!("invalid object id '{s}'")
	}

	Ok(())
}
