use crate::AppState;
use job_scheduler::{Job, JobScheduler};

use self::ddns::DnsTask;

mod ddns;
// Let's do a struct for the jobs to make it easier to add them
#[derive(Debug)]
struct ScheduledJob<T: TaskTrait + Copy> {
    name: String,
    cron_interval: String,
    state: AppState,
    task: T,
}
#[async_trait]
trait TaskTrait {
    async fn run(&self, state: AppState);
}

impl<T: TaskTrait + Copy> ScheduledJob<T> {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn run_task(&self) {
        futures::executor::block_on(async {
            self.task.run(self.state.clone()).await;
        });
    }
}

pub async fn start(state: AppState) {
    // Start the jobs using job_scheduler
    let mut sched = JobScheduler::new();

    // Add the jobs to the scheduler
    let jobs = vec![ScheduledJob {
        name: "DDNS".to_string(),
        cron_interval: "0 0,15,30,45 * * * *".to_string(),
        state: state.clone(),
        task: DnsTask,
    }];

	let debug_flag = state.lock().unwrap().config.debug.clone();

	if debug_flag {
		println!("Jobs: {:#?}", jobs);
	}


    for job in jobs {
        println!("Added job: {}", job.name());
        sched.add(Job::new(job.cron_interval.parse().unwrap(), move || {
            job.run_task();
        }));
    }

    // Start the scheduler

    

    if debug_flag {
        println!("Running scheduler");
    }
    loop {
        sched.tick();
        std::thread::sleep(std::time::Duration::from_millis(1000*30));
    }
}
