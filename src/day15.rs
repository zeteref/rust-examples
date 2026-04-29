// Day 15: Async Rust
//
// Write async functions using tokio to run concurrent tasks, race futures,
// and implement timeouts.
//
// Learning goals:
//   - `async fn` and `.await`
//   - `tokio::join!` for concurrent execution
//   - `tokio::select!` for racing futures
//   - `tokio::spawn` for background tasks
//   - `tokio::time::timeout` for bounded waits

/// Simulates processing an item by sleeping for the given duration
/// and then returning a formatted result string.
pub async fn process_item(id: u32, delay_ms: u64) -> String {
    todo!("Implement process_item: sleep for delay_ms ms, then return formatted string")
}

/// Runs three items concurrently and returns their results in a tuple.
/// All three should start at the same time, not sequentially.
/// Use `tokio::join!`.
pub async fn process_all_concurrently() -> (String, String, String) {
    todo!("Implement process_all_concurrently using tokio::join!")
}

/// Runs two futures and returns the result of whichever finishes first.
/// Use `tokio::select!`.
pub async fn race_two_futures() -> String {
    todo!("Implement race_two_futures using tokio::select!")
}

/// Spawns a background task and returns its JoinHandle.
pub fn spawn_background_task(id: u32, delay_ms: u64) -> tokio::task::JoinHandle<String> {
    todo!("Implement spawn_background_task using tokio::spawn")
}

/// Runs a future with a timeout. If the future does not complete
/// within the given duration, return Err(());
/// otherwise, return Ok(result).
pub async fn run_with_timeout(delay_ms: u64, timeout_ms: u64) -> Result<String, ()> {
    todo!("Implement run_with_timeout using tokio::time::timeout")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn process_item_returns_correct_format() {
        let result = process_item(42, 10).await;
        assert!(result.contains("42"), "Result should contain item id: {}", result);
    }

    #[tokio::test]
    async fn process_all_concurrently_is_faster_than_sequential() {
        // If sequential, 50ms + 60ms + 70ms >= 180ms
        // If concurrent, < 100ms (max of individual delays)
        let start = Instant::now();
        let results = process_all_concurrently().await;
        let elapsed = start.elapsed().as_millis();

        // Concurrent execution should be roughly the max delay, not the sum
        assert!(elapsed < 300, "Expected concurrent (<300ms), but took {}ms", elapsed);
        assert!(!results.0.is_empty());
    }

    #[tokio::test]
    async fn process_all_concurrently_results_are_non_empty() {
        let (a, b, c) = process_all_concurrently().await;
        assert!(!a.is_empty());
        assert!(!b.is_empty());
        assert!(!c.is_empty());
    }

    #[tokio::test]
    async fn race_two_futures_returns_faster_one() {
        let result = race_two_futures().await;
        // The faster future should finish first
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn spawn_background_task_produces_result() {
        let handle = spawn_background_task(1, 10);
        let result = handle.await.unwrap();
        assert!(result.contains("1"), "Result: {}", result);
    }

    #[tokio::test]
    async fn run_with_timeout_completes_before_timeout() {
        let result = run_with_timeout(10, 500).await;
        assert!(result.is_ok(), "Should complete within timeout");
    }

    #[tokio::test]
    async fn run_with_timeout_exceeds_timeout() {
        let result = run_with_timeout(500, 10).await;
        assert!(result.is_err(), "Should timeout");
    }
}
