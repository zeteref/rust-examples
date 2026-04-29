// Day 12: ThreadPool
//
// Implement a simple thread pool that executes concurrent tasks and gracefully
// shuts down, waiting for all submitted work to complete.
//
// Learning goals:
//   - Thread spawning and JoinHandles
//   - Channel-based communication (mpsc)
//   - Synchronization primitives: Arc, Mutex
//   - RAII cleanup with Drop
//   - FnOnce + Send + 'static trait bounds

pub struct ThreadPool {
    // Define your fields here (e.g., workers, sender channel)
    _priv: (),
}

impl ThreadPool {
    /// Creates a new ThreadPool with the given number of worker threads.
    /// Panics if size is 0.
    pub fn new(size: usize) -> Self {
        todo!("Implement new")
    }

    /// Submits a closure to be executed by one of the worker threads.
    /// The closure must be `FnOnce() + Send + 'static`.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!("Implement execute")
    }

    /// Shuts down the pool, waiting for all submitted tasks to complete.
    pub fn shutdown(self) {
        todo!("Implement shutdown")
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // If shutdown hasn't been called, clean up threads here
        todo!("Implement Drop for ThreadPool")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn new_constructs_without_panic() {
        let pool = ThreadPool::new(4);
        drop(pool);
    }

    #[test]
    fn execute_runs_closure() {
        let pool = ThreadPool::new(2);
        let data = Arc::new(Mutex::new(vec![]));

        for i in 0..10 {
            let data = Arc::clone(&data);
            pool.execute(move || {
                data.lock().unwrap().push(i);
            });
        }

        pool.shutdown();

        let mut results: Vec<u64> = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
        results.sort();
        assert_eq!(results, (0..10).collect::<Vec<u64>>());
    }

    #[test]
    fn all_tasks_complete_before_shutdown_returns() {
        let pool = ThreadPool::new(4);
        let data = Arc::new(Mutex::new(0u64));

        for _ in 0..20 {
            let data = Arc::clone(&data);
            pool.execute(move || {
                for _ in 0..1000 {
                    *data.lock().unwrap() += 1;
                }
            });
        }

        pool.shutdown();

        let total = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
        assert_eq!(total, 20 * 1000);
    }

    #[test]
    fn sum_of_squares_one_to_hundred() {
        let pool = ThreadPool::new(4);
        let sum = Arc::new(Mutex::new(0u64));

        for i in 1..=100u64 {
            let sum = Arc::clone(&sum);
            pool.execute(move || {
                *sum.lock().unwrap() += i * i;
            });
        }

        pool.shutdown();
        let result = Arc::try_unwrap(sum).unwrap().into_inner().unwrap();
        assert_eq!(result, 338350);
    }

    #[test]
    fn pool_size_one_still_executes_all_tasks() {
        let pool = ThreadPool::new(1);
        let data = Arc::new(Mutex::new(vec![]));

        for i in 0..5 {
            let data = Arc::clone(&data);
            pool.execute(move || {
                data.lock().unwrap().push(i);
            });
        }

        pool.shutdown();

        let mut results = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
        results.sort();
        assert_eq!(results, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn shutdown_can_be_called_without_deadlock() {
        let pool = ThreadPool::new(2);
        for _ in 0..5 {
            pool.execute(|| {
                let _x: u64 = 42;
            });
        }
        pool.shutdown();
    }

    #[test]
    fn tasks_can_capture_variables_from_outer_scope() {
        let pool = ThreadPool::new(2);
        let message = Arc::new(String::from("hello"));
        let results = Arc::new(Mutex::new(vec![]));

        for _ in 0..3 {
            let msg = Arc::clone(&message);
            let res = Arc::clone(&results);
            pool.execute(move || {
                res.lock().unwrap().push(msg.to_string());
            });
        }

        pool.shutdown();

        let collected = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
        for item in &collected {
            assert_eq!(item, "hello");
        }
        assert_eq!(collected.len(), 3);
    }
}
