use crate::AppState;
use self_update::cargo_crate_version;

use super::TaskTrait;

//https://github.com/ProjectOS-wt/HomeAPI
const REPO_OWNER: &str = "ProjectOS-wt";
const REPO_NAME: &str = "HomeAPI";

#[derive(Debug, Clone, Copy)]
pub(super) struct SelfUpdateTask;

#[async_trait]
impl TaskTrait for SelfUpdateTask {
    async fn run(&self, state: AppState) {
        let config = state.lock().unwrap().config.clone();
        // Get the configuration
        let debug_flag = config.debug.clone();
        let selfupdate_bool = config.selfupdate.clone();

        if !selfupdate_bool {
            return;
        }

        if debug_flag {
            println!("Running SelfUpdate job");
        }

        let crate_name = env!("CARGO_PKG_NAME");

        let pre_update = match self_update::backends::github::Update::configure()
            .repo_owner(REPO_OWNER)
            .repo_name(REPO_NAME)
            .bin_name(crate_name)
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()
        {
            Ok(pre_update) => pre_update,
			Err(e) => {
				println!("Error configuring: {}", e);
				return;
			}
        };
        let status = match pre_update.update() {
            Ok(status) => status,
            Err(e) => {
                println!("Error updating: {}", e);
                return;
            }
        };
        println!("Update status: `{}`!", status.version());
    }
}
