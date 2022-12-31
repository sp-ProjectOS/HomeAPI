//use rocket_db_pools::{Connection, sqlx};

//use crate::{MainDB};
// Path: src/routes/ping.rs

// Returns the status of the server and the status of the database connection as a JSON object
#[get("/")]
pub async fn status(/*mut db: Connection<MainDB>*/) -> String {
	// Check if the database is connected
	/* let result = sqlx::query("SELECT content FROM logs WHERE id = ?")
		.bind(1)
		.fetch_one(&mut *db).await;
	// Return the result
	format!("{{\"server\": \"online\", \"database\": \"{}\"}}", if result.is_ok() { "online" } else { "offline" }) */
	format!("{{\"server\": \"online\"}}")
}