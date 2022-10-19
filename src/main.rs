use rocket_db_pools::{Database};
use rocket_db_pools::sqlx::{self};

#[macro_use] extern crate rocket;
pub mod routes;
pub mod config;
pub mod util;

#[derive(Database)]
#[database("sqlite_main")]
pub struct MainDB(sqlx::SqlitePool);
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
		.attach(MainDB::init())
		.mount("/", routes![routes::index])
		.mount("/wake", routes![routes::wake::device])
		.mount("/ping", routes![routes::ping::status])
		.launch()
		.await?;
	// Return the result
	Ok(())
}