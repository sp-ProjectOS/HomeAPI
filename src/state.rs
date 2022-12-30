//use rocket_db_pools::{Database};
//use rocket_db_pools::sqlx::{self};

use crate::config;


//#[derive(Database)]
//#[database("sqlite_main")]
//pub struct MainDB(sqlx::SqlitePool);
#[derive(Debug, Clone)]
pub struct InnerState {
    pub config: config::Config,
}
impl InnerState {
    pub fn new(config: config::Config) -> Self {
        Self { config }
    }
}