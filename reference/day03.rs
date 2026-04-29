pub struct MinMaxStack<T> {
    data: Vec<T>,
    min_stack: Vec<T>,
    max_stack: Vec<T>,
}

impl<T: Copy + PartialOrd> MinMaxStack<T> {
    pub fn new() -> Self {
        MinMaxStack {
            data: Vec::new(),
            min_stack: Vec::new(),
            max_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        let new_min = match self.min_stack.last() {
            Some(&current_min) if val > current_min => current_min,
            _ => val,
        };
        let new_max = match self.max_stack.last() {
            Some(&current_max) if val < current_max => current_max,
            _ => val,
        };
        self.data.push(val);
        self.min_stack.push(new_min);
        self.max_stack.push(new_max);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.min_stack.pop();
        self.max_stack.pop();
        self.data.pop()
    }

    pub fn peek(&self) -> Option<T> {
        self.data.last().copied()
    }

    pub fn min(&self) -> Option<T> {
        self.min_stack.last().copied()
    }

    pub fn max(&self) -> Option<T> {
        self.max_stack.last().copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
