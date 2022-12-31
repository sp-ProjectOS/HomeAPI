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

	/*
	 * Set handles to the server and runtime and be able to
	 * gracefully shutdown the server when the runtime is
	 * terminated and vice versa.
	 * Run the server and runtime in parallel.
	 */
	let server_handle = tokio::spawn(async move {
		println!("Starting server");
		server(server_appstate).await
	});
	let runtime_handle = tokio::spawn(async move {
		println!("Starting runtime");
		jobs::start(runtime_appstate).await
	});

	// Wait for either the server or runtime to terminate
	let _ = tokio::select! {
		biased;
		_ = server_handle => {
			println!("Server terminated");
			appstate.lock().unwrap().terminate();
		}
		_ = runtime_handle => {
			println!("Runtime terminated");
			appstate.lock().unwrap().terminate();
		}
		
	};

	println!("Successfully terminated");
	Ok(())
}