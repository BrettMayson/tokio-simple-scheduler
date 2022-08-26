use std::time::Duration;

use crate::job::Job;

#[derive(Default)]
pub struct Scheduler {
    jobs: Vec<Job>,
}

impl Scheduler {
    pub fn add(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn until(&mut self) -> Option<(Vec<&mut Job>, Duration)> {
        let mut next_jobs = Vec::new();
        let mut min_duration = None;
        for job in &mut self.jobs {
            if let Some(duration) = job.until() {
                let duration = duration.as_millis();
                if min_duration.is_none() || duration < min_duration.unwrap() {
                    min_duration = Some(duration);
                    next_jobs.clear();
                    next_jobs.push(job);
                } else if duration == min_duration.unwrap() {
                    next_jobs.push(job);
                }
            }
        }
        if let Some(duration) = min_duration {
            return Some((next_jobs, Duration::from_millis(duration as u64)));
        }
        None
    }

    pub async fn start(&mut self) {
        loop {
            if let Some((jobs, duration)) = self.until() {
                tokio::time::sleep(duration).await;
                for job in jobs {
                    job.run().await;
                }
            } else {
                return;
            }
        }
    }
}
