use rocket::serde::json::serde_json::json;

use crate::AppState;

use super::TaskTrait;

//use cloudflare::endpoints::dns::DnsContent;

const SELFIP_URL: &str = "https://api.ipify.org";
const RAWCLOUDFLARE_URL: &str = "https://api.cloudflare.com/client/v4";
const LIST_ZONES_PATH: &str = "/zones";
const LIST_RECORDS_PATH: &str = "/zones/{{ZONE_IDENTIFIER}}/dns_records";
const UPDATE_RECORD_PATH: &str = "/zones/{{ZONE_IDENTIFIER}}/dns_records/{{RECORD_IDENTIFIER}}";
// We define the functionality of the DDNS job here.
// This is the function that will be called by the scheduler.

#[derive(Debug, Clone, Copy)]
pub(super) struct DnsTask;
#[async_trait]
impl TaskTrait for DnsTask {
    async fn run(&self, state: AppState) {
		
		let config = state.lock().unwrap().config.clone();
		// Get the configuration
		let debug_flag = config.debug.clone();
		let dns_zone_name = config.dns.domain.clone();
        let dns_record_name = config.dns.subdomain.clone();
        let dns_token = config.dns.token.clone();
		
        let client = reqwest::Client::new();

		if debug_flag {
			println!("Running DDNS job");
		}

        // Raw API call version
        // Use when cloudflare_rs is not working
        // Get the IP address
        let ip = client
            .get(SELFIP_URL)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
		if debug_flag {
			// Print the IP
			println!("IP: {}", ip);
		}

		// Get the zone ID
		let list_zones_path = LIST_ZONES_PATH;
		let list_zones_url = format!("{}{}", RAWCLOUDFLARE_URL, list_zones_path);
		let zones_raw = client.get(list_zones_url)
			.header("Authorization", format!("Bearer {}", dns_token))
			.send()
			.await
			.unwrap()
			.text()
			.await
			.unwrap();

			

		let json_zones= json::parse(&zones_raw).unwrap();

		if debug_flag {
			// Print the zones
			println!("Zones: {}", json_zones);
		}

		// Iterate through the zones and find the one we want
		let dns_zone_id = json_zones["result"].members().find(|zone| {
			zone["name"].as_str().unwrap() == dns_zone_name
		}).expect("Did not find dns zone")["id"].to_string();
		
		if debug_flag {
			// Print the zone ID
			println!("DNS Zone ID: {}", dns_zone_id);
		}



        // Check if the record exists and if it is the same IP
        let list_records_path = LIST_RECORDS_PATH.replace("{{ZONE_IDENTIFIER}}", &dns_zone_id);
        let list_records_url = format!("{}{}", RAWCLOUDFLARE_URL, list_records_path);

        let records_raw = client
            .get(list_records_url)
            .header("Authorization", format!("Bearer {}", dns_token))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

		if debug_flag {
			// Print the response
			println!("List records response: {}", records_raw);
		}
        
        let json_records = json::parse(&records_raw).unwrap();

        // Find the record id if it exists
		let dns_record = json_records["result"].members().find(|record| {
			record["name"].as_str().unwrap().to_string().starts_with(&dns_record_name)
		}).expect("Record not found");

		let dns_record_id = dns_record["id"].as_str().unwrap_or("");
		let dns_record_ip = dns_record["content"].as_str().unwrap_or("");

		if debug_flag {
			// Print the record ID
			println!("DNS Record ID: {}", dns_record_id);
		}

		let record_exists = dns_record_id != "";
		let ip_is_same = dns_record_ip.to_string().eq(&ip);

		if debug_flag {
			// Print if the record exists and if the IP is the same
			println!("Record exists: {}", record_exists);
			println!("IP is the same: {}", ip_is_same);
		}


        // If the record exists and the IP is different, update the record
        if record_exists && !ip_is_same {
            let update_record_path = UPDATE_RECORD_PATH
                .replace("{{ZONE_IDENTIFIER}}", &dns_zone_id)
                .replace("{{RECORD_IDENTIFIER}}", &dns_record_id);
            let update_record_url = format!("{}{}", RAWCLOUDFLARE_URL, update_record_path);

            let update_res_raw = client
                .put(update_record_url)
                .header("Authorization", format!("Bearer {}", dns_token))
                .json(&json!({
                    "type": "A",
                    "comment": "HomeAPI_DDNS",
					"name": format!("{}.{}", dns_record_name, dns_zone_name),
                    "content": ip,
                    "ttl": 900,// 15 minutes
                    "proxied": false
                }))
                .send()
                .await
                .unwrap();

			if debug_flag {
				// Print the response
				println!("Update record response: {}", update_res_raw.text().await.unwrap());
			}

            return ();
        }

        // Update the DNS record using the Cloudflare API
        // Use when cloudflare_rs is working
        /* // Get the IP address
        let ip = reqwest::get(DDNS_URL).await?.unwrap().text().unwrap();
        // Update the DNS record using cloudflare_rs
        let creds = cloudflare::framework::auth::Credentials::UserAuthToken {
            token: app.config.cloudflare_api_key.clone(),
        };
        let zone_identifier = app.config.cloudflare_zone_identifier.clone();
        let dns_identifier = app.config.cloudflare_dns_identifier.clone();
        let dns_content = DnsContent::A { content: ip };
        let params = cloudflare::endpoints::dns::UpdateDnsRecordParams {
            ttl: None,
            proxied: Ok(true),
            name: "ddns",
            content: dns_content,
        };

        let req = cloudflare::endpoints::dns::UpdateDnsRecord(
            zone_identifier,
            dns_identifier,
            dns_content,
        );

        let client = cloudflare::framework::HttpApiClient::new(
            creds,
            cloudflare::framework::HttpApiClientConfig::default(),
            cloudflare::framework::Environment::Production,
        );

        // Return the result
        Ok(()) */
		if debug_flag {
			println!("DDNS job finished");
		}
		()
    }
}