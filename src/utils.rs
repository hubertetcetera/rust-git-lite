use anyhow::Result;

/// Checks if provided string slice is a valid SHA-1 hash.
pub fn ensure_valid_sha1(s: &str) -> Result<()> {
	let ok_len = s.len() == 40;
	let ok_hex = s.chars().all(|c| c.is_ascii_hexdigit());

	if !(ok_hex && ok_len) {
		anyhow::bail!("invalid object id '{s}'")
	}

	Ok(())
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
