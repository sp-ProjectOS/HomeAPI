use rocket::State;
// Path: src/routes/wake.rs
use wake_on_lan;
use crate::{util::net::parse_mac_address, App};

#[post("/<name>")]
pub fn wol_wake(app:&State<App>,name: &str) -> String {
	// Load the configuration file
	let config = &app.config;
	// Find the device in the configuration file
	let device = config.devices.iter().find(|device| device.name == name);
	// Check if the device was found
	if device.is_none() {
		return format!("Device {} not found", name);
	}
	// Convert the MAC address to a byte array
	let mac_string = &device.unwrap().mac;
	/* let mac : [u8;6] = mac_string.split(":").map(|x| u8::from_str_radix(x, 16).unwrap()).collect(); */
	let mac = parse_mac_address(&mac_string).unwrap();
	// Create the magic packet
	let magic_packet = wake_on_lan::MagicPacket::new(&mac);
	// Wake the device
	let result = magic_packet.send();
	// Check if the device was woken
	if result.is_err() {
		return format!("Failed to wake device {}", name);
	}
	// Return the result
	format!("Woke device {}", name)
}
