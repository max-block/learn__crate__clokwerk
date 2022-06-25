use std::time::Duration;

use clokwerk::{AsyncScheduler, TimeUnits};

async fn job() {
    println!("job start");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("job finish");
}

#[tokio::main]
async fn main() {
    let mut scheduler = AsyncScheduler::new();

    scheduler.every(2.seconds()).run(|| async { job().await });

    tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    tokio::time::sleep(Duration::from_secs(666663)).await;
}
