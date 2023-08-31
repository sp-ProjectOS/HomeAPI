use std::time::Duration;

use crate::AppState;
use clokwerk::{Interval, ScheduleHandle, Scheduler};

//use self::ddns::DnsTask;
use self::selfupdate::SelfUpdateTask;

mod ddns;
mod selfupdate;
// Let's do a struct for the jobs to make it easier to add them
#[derive(Debug)]
struct ScheduledJob<T: TaskTrait + Copy> {
    name: String,
    interval: Interval,
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
        // use tokio runtime to run the task
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.task.run(self.state.clone()));
    }
}

pub async fn start(state: AppState) -> ScheduleHandle {
    // Start the jobs using job_scheduler
    let mut sched = Scheduler::new();

    // Add the jobs to the scheduler
    let jobs = vec![
       /*  ScheduledJob {
            name: "DDNS".to_string(),
            interval: Interval::Minutes(15),
            state: state.clone(),
            task: DnsTask,
        }, */
        ScheduledJob {
			name: "SelfUpdate".to_string(),
			//interval: Interval::Days(1),
			interval: Interval::Minutes(1),
			state: state.clone(),
			task: SelfUpdateTask,
		},
    ];

    let debug_flag = state.lock().unwrap().config.debug.clone();

    if debug_flag {
        println!("Jobs: {:#?}", jobs);
    }

    for job in jobs {
        println!("Added job: {}", job.name());
        sched.every(job.interval).run(move || job.run_task());
    }

    // Start the scheduler

    if debug_flag {
        println!("Running scheduler");
    }

    return sched.watch_thread(Duration::from_millis(100));
}
