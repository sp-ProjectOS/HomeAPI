#[macro_use] extern crate rocket;
pub mod routes;
pub mod config;
pub mod util;

#[derive(Debug)]
pub struct App {
	pub config: config::Config,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	// Load the configuration file
	let config = config::load_config();
	// Create the application
	let app = App { config };
	// Start the server
	let _rocket = rocket::build()
		.manage(app)
		.mount("/", routes![routes::index])
		.mount("/wake", routes![routes::wake::wol_wake])
		.launch()
		.await?;
	// Return the result
	Ok(())
}