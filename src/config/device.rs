use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
	pub id: String,
    pub mac: String,
    pub ip: String,
    pub device_type: DeviceType,
    _type_string: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum DeviceType {
    Computer,
    Phone,
    Tablet,
    Router,
    Printer,
	Switch,
    Other,
}

pub fn load_devices_config(config: &Yaml) -> Vec<Device> {
	let mut devices: Vec<Device> = Vec::new();
	for device in config["devices"].as_vec().expect("No devices specified in config file") {
		devices.push(Device {
			name: device["name"].as_str().unwrap().to_string(),
			id: device["id"].as_str().unwrap().to_string(),
			mac: device["mac"].as_str().unwrap().to_string(),
			ip: device["ip"].as_str().unwrap().to_string(),
			device_type: match device["type"].as_str().expect(format!("Device type not specified for device {}", device["name"].as_str().unwrap()).as_str()) {
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
	devices
}