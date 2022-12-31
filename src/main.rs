use std::sync::{Arc, Mutex};

use state::InnerState;

#[macro_use]
extern crate rocket;
pub mod config;
pub mod jobs;
pub mod routes;
pub mod state;
pub mod util;

type AppState = Arc<Mutex<InnerState>>;

async fn server (appstate: AppState) -> Result<(), rocket::Error> {
	let _ = rocket::build()
		.manage(appstate.clone())
		//.attach(MainDB::init())
		.mount("/", routes![routes::index])
		.mount("/wake", routes![routes::wake::device])
		.mount("/ping", routes![routes::ping::status])
		.launch()
		.await.expect("Failed to launch server");
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
	// Load the configuration file
	let config = config::load_config();
	// Create the application

	let appstate = Arc::new(Mutex::new(InnerState::new(config)));

	let server_appstate = appstate.clone();
	let runtime_appstate = appstate.clone();

	// Run runtime and server as concurrent thread and wait for SIGINT
	// Run the server and the runtime concurrently
	// Terminate the server if the runtime terminates and vice versa
	// Return the result

	let server = tokio::spawn(async move {
		// Wait 10 seconds before starting the server
		tokio::time::sleep(std::time::Duration::from_secs(10)).await;
		server(server_appstate).await
	});

	let runtime = tokio::spawn(async move {
		jobs::start(runtime_appstate).await
	});

	let _ = tokio::select! {
		biased;
		_ = server => {
			println!("Server terminated");
		}
		_ = runtime => {
			println!("Runtime terminated");
		}
	};

	Ok(())
}