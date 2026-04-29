// Day 6: Pipeline<T>
//
// Build a composable pipeline of transformation steps that can be applied
// to any input value. Supports the builder pattern.
//
// Learning goals:
//   - Generics with closure storage
//   - Builder pattern in Rust
//   - Storing and calling `Fn` closures
//   - Applying transformation chains

pub struct Pipeline<T> {
    // Define your fields here
    _marker: std::marker::PhantomData<T>,
}

impl<T> Pipeline<T> {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    /// Adds a transformation step to the pipeline. Steps are applied in the
    /// order they were added, not reversed.
    pub fn add_step<F>(mut self, f: F) -> Self
    where
        F: Fn(T) -> T + 'static,
    {
        todo!("Implement add_step")
    }

    /// Executes the pipeline on a single input value.
    pub fn execute(&self, input: T) -> T {
        todo!("Implement execute")
    }

    /// Executes the pipeline on each input value independently.
    pub fn execute_all(&self, inputs: Vec<T>) -> Vec<T> {
        todo!("Implement execute_all")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pipeline_returns_input_unchanged() {
        let pipe: Pipeline<i32> = Pipeline::new();
        assert_eq!(pipe.execute(42), 42);
    }

    #[test]
    fn single_step_transforms_correctly() {
        let pipe = Pipeline::new().add_step(|x: i32| x * 2);
        assert_eq!(pipe.execute(21), 42);
    }

    #[test]
    fn multiple_steps_applied_in_order() {
        let pipe = Pipeline::new()
            .add_step(|x: i32| x + 10)  // 5 + 10 = 15
            .add_step(|x: i32| x * 3)    // 15 * 3 = 45
            .add_step(|x: i32| x - 5);  // 45 - 5 = 40
        assert_eq!(pipe.execute(5), 40);
    }

    #[test]
    fn steps_are_not_reversed() {
        // If steps were reversed, subtract then add would give different result
        let pipe = Pipeline::new()
            .add_step(|x: i32| x - 1)
            .add_step(|x: i32| x + 2);
        assert_eq!(pipe.execute(10), 11); // (10-1)+2 = 11, not (10+2)-1 = 11 (fortuitously same)
        // Better test: subtract then multiply
        let pipe2 = Pipeline::new()
            .add_step(|x: i32| x - 1)
            .add_step(|x: i32| x * 3);
        assert_eq!(pipe2.execute(10), 27); // (10-1)*3 = 27, not (10*3)-1 = 29
    }

    #[test]
    fn execute_all_applies_to_each_input() {
        let pipe = Pipeline::new().add_step(|x: i32| x * x);
        assert_eq!(pipe.execute_all(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
    }

    #[test]
    fn execute_all_on_empty_vec_returns_empty_vec() {
        let pipe = Pipeline::new().add_step(|x: i32| x + 1);
        assert_eq!(pipe.execute_all(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn string_pipeline_transform() {
        let pipe = Pipeline::new()
            .add_step(|s: String| s.trim().to_string())
            .add_step(|s: String| s.to_lowercase())
            .add_step(|s: String| s.replace(' ', "_"))
            .add_step(|s: String| format!("{}_processed", s));

        let input = "  Hello World  ".to_string();
        let expected = "hello_world_processed".to_string();
        assert_eq!(pipe.execute(input), expected);
    }

    #[test]
    fn builder_pattern_chains_correctly() {
        // Verifies that add_step returns Self for chaining
        let pipe: Pipeline<String> = Pipeline::new()
            .add_step(|s: String| s)
            .add_step(|s: String| s);
        assert_eq!(pipe.execute("test".to_string()), "test".to_string());
    }
}
