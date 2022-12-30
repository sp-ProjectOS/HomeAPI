use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct DnsConfig {
	pub domain: String,
	pub subdomain: String,
	pub token: String,
}

pub fn load_dns_config(config: &Yaml) -> DnsConfig {
	DnsConfig {
		domain: config["dns"]["domain"].as_str().unwrap().to_string(),
		subdomain: config["dns"]["subdomain"].as_str().unwrap().to_string(),
		token: config["dns"]["token"].as_str().unwrap().to_string(),
	}
}