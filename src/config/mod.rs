// This module parses, validates and stores the configuration for the application.
// It also provides a function to reloas the configuration file.

use yaml_rust::YamlLoader;
use self::device::Device;
use self::dns::DnsConfig;

pub mod device;
pub mod dns;
// The configuration struct // should be clonable
#[derive(Debug, Clone)]
pub struct Config {
	pub debug: bool,
    pub devices: Vec<Device>,
	pub dns: DnsConfig,
}
impl Config {
	/* fn clone(&self) -> Config{
		Config {
			devices: self.devices.clone(),
			dns: self.dns.clone(),
		}
	} */
}

pub static DEFAULT_CONFIG_FILE: &str = "config.yml";
// Load the configuration file
pub fn load_config() -> Config {
    // Load the configuration file
    let config_string =
        std::fs::read_to_string(DEFAULT_CONFIG_FILE).expect("Failed to read config file");
    // Parse the configuration file
    let config = YamlLoader::load_from_str(&config_string).expect("Failed to parse config file");
	// DEBUG Print the parsed configuration file
    let debugcfg = &config[0];
	
	// Load the devices
	let debug = config[0]["debug"].as_bool().unwrap();
	let devices = device::load_devices_config(&config[0]);
	let dns = dns::load_dns_config(&config[0]);

	if debug {
		println!("Config: {:#?}", debugcfg);
	}
	// Return the configuration
	Config { debug, devices, dns }
}
