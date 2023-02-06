use crate::service::Service;
use chrono::Utc;
use tokio_schedule::{every, Job};

pub async fn start_scheduler() {
    let every_second = every(20)
        .minutes()
        .in_timezone(&Utc)
        .perform(|| async { Service::remove_old_links() });
    every_second.await;
}
