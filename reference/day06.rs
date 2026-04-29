pub struct Pipeline<T> {
    steps: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T> Pipeline<T> {
    pub fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    pub fn add_step<F>(mut self, f: F) -> Self
    where
        F: Fn(T) -> T + 'static,
    {
        self.steps.push(Box::new(f));
        self
    }

    pub fn execute(&self, input: T) -> T {
        self.steps.iter().fold(input, |acc, step| step(acc))
    }

    pub fn execute_all(&self, inputs: Vec<T>) -> Vec<T> {
        inputs.into_iter().map(|input| self.execute(input)).collect()
    }
}
