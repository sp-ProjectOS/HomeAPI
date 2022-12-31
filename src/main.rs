use std::sync::{Arc, Mutex};

use clokwerk::ScheduleHandle;
use config::Config;
use state::InnerState;

#[macro_use]
extern crate rocket;
pub mod config;
pub mod jobs;
pub mod routes;
pub mod state;
pub mod util;

type AppState = Arc<Mutex<InnerState>>;

enum ThreadEnded {
    Server,
    Runtime,
}

struct Runtime {
    _scheduler_handle: ScheduleHandle,
}
impl Runtime {
    pub async fn new(appstate: AppState) -> Self {
        let scheduler_handle = jobs::start(appstate).await;
        Self { _scheduler_handle: scheduler_handle }
    }
    pub async fn run(self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

async fn server(a_s: AppState, c: Config) -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .manage(a_s.clone())
        .manage(c)
        //.attach(MainDB::init())
        .mount("/", routes![routes::index])
        .mount("/wake", routes![routes::wake::device])
        .mount("/ping", routes![routes::ping::status])
        .launch()
        .await
        .expect("Failed to launch server");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    // Load the configuration file
    let config = config::load_config();

    let server_config = config.clone();
    let appstate = Arc::new(Mutex::new(InnerState::new(config)));

    let server_appstate = appstate.clone();

    /*
     * Set handles to the server and runtime and be able to
     * gracefully shutdown the server when the runtime is
     * terminated and vice versa.
     * Run the server and runtime in parallel.
     */
    let server_handle = tokio::spawn(async move {
        println!("Starting server");
        server(server_appstate, server_config).await
    });
    let runtime = Runtime::new(appstate.clone()).await;
    let runtime_handle = tokio::spawn(async move {
        println!("Starting runtime");
        runtime.run().await
    });

    // Wait for either the server or runtime to terminate
    let _: Result<ThreadEnded, std::io::Error> = tokio::select! {
        _ = server_handle => Ok(ThreadEnded::Server),
        _ = runtime_handle => Ok(ThreadEnded::Runtime),
    };
	

    println!("Successfully terminated");
    Ok(())
}