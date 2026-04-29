use std::time::Duration;
use tokio::time::sleep;

pub async fn process_item(id: u32, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("processed item {}", id)
}

pub async fn process_all_concurrently() -> (String, String, String) {
    let (a, b, c) = tokio::join!(
        process_item(1, 30),
        process_item(2, 40),
        process_item(3, 50),
    );
    (a, b, c)
}

pub async fn race_two_futures() -> String {
    tokio::select! {
        result = process_item(1, 10) => result,
        result = process_item(2, 100) => result,
    }
}

pub fn spawn_background_task(id: u32, delay_ms: u64) -> tokio::task::JoinHandle<String> {
    tokio::spawn(async move { process_item(id, delay_ms).await })
}

pub async fn run_with_timeout(delay_ms: u64, timeout_ms: u64) -> Result<String, ()> {
    tokio::time::timeout(
        Duration::from_millis(timeout_ms),
        process_item(1, delay_ms),
    )
    .await
    .map_err(|_| ())
}
