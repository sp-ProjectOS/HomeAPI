// This module parses, validates and stores the configuration for the application.
// It also provides a function to reloas the configuration file.

use yaml_rust::YamlLoader;

// The configuration struct
#[derive(Debug)]
pub struct Config{
    pub devices: Vec<Device>,
}
#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub mac: String,
    pub ip: String,
    pub device_type: DeviceType,
    _type_string: String,
}
#[derive(Debug)]
pub enum DeviceType {
    Computer,
    Phone,
    Tablet,
    Router,
    Printer,
    Other,
}
pub static DEFAULT_CONFIG_FILE: &str = "config.yml";
// Load the configuration file
pub fn load_config() -> Config {
	// DEBUG: Print the current working directory
	println!("Current working directory: {}", std::env::current_dir().unwrap().display());
    // Load the configuration file
	let config_string = std::fs::read_to_string(DEFAULT_CONFIG_FILE).expect("Failed to read config file");
    // Parse the configuration file
    let config = YamlLoader::load_from_str(&config_string).expect("Failed to parse config file");

    // Parse the configuration file
    let mut devices: Vec<Device> = Vec::new();
    for device in config[0]["devices"].as_vec().unwrap() {
        devices.push(Device {
            name: device["name"].as_str().unwrap().to_string(),
            mac: device["mac"].as_str().unwrap().to_string(),
            ip: device["ip"].as_str().unwrap().to_string(),
            device_type: match device["type"].as_str().unwrap() {
                "computer" => DeviceType::Computer,
                "phone" => DeviceType::Phone,
                "tablet" => DeviceType::Tablet,
                "router" => DeviceType::Router,
                "printer" => DeviceType::Printer,
                _ => DeviceType::Other,
            },
            _type_string: device["type"].as_str().unwrap().to_string(),
        });
    }

    // Return the configuration
    Config { devices }
}
