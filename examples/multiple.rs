use std::time::Duration;

use chrono::Utc;
use tokio_simple_scheduler::{Job, Scheduler};

#[tokio::main]
async fn main() {
    let mut scheduler = Scheduler::default();
    scheduler.add(
        Job::new("every 2", "*/2 * * * * *", || {
            Box::pin(async {
                println!("{:?} - Every 2 seconds", Utc::now());
            })
        })
        .unwrap(),
    );
    scheduler.add(
        Job::new("every 4", "*/4 * * * * *", || {
            Box::pin(async {
                println!("{:?} - Every 4 seconds", Utc::now());
            })
        })
        .unwrap(),
    );
    scheduler.add(
        Job::new("every 5", "*/5 * * * * *", || {
            Box::pin(async {
                println!("{:?} - Every 5 seconds", Utc::now());
            })
        })
        .unwrap(),
    );
    scheduler.add(
        Job::new("every 10", "*/10 * * * * *", || {
            Box::pin(async {
                println!("{:?} - Every 10 seconds", Utc::now());
            })
        })
        .unwrap(),
    );
    scheduler.add(
        Job::new("every 30", "*/30 * * * * *", || {
            Box::pin(async {
                println!("this job");
                tokio::time::sleep(Duration::from_secs(3)).await;
                println!("takes a few");
                tokio::time::sleep(Duration::from_secs(3)).await;
                println!("seconds to run");
            })
        })
        .unwrap(),
    );
    scheduler.start().await;
}
