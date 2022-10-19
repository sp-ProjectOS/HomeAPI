pub fn parse_mac_address(mac: &str) -> Result<[u8; 6], String> {
	let mut mac_bytes = [0u8; 6];
	let mut i = 0;
	for byte in mac.split(':') {
		mac_bytes[i] = u8::from_str_radix(byte, 16).map_err(|_| "Invalid MAC address")?;
		i += 1;
	}
	Ok(mac_bytes)
}