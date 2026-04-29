// Day 3: MinMaxStack<T>
//
// Implement a generic stack that tracks the minimum and maximum values in O(1)
// after each push/pop operation.
//
// Learning goals:
//   - Generics with trait bounds (`T: Copy + PartialOrd`)
//   - Internal data structure design (tracking min/max alongside values)
//   - Option handling for empty states

pub struct MinMaxStack<T> {
    // Define your fields here
    _marker: std::marker::PhantomData<T>,
}

impl<T: Copy + PartialOrd> MinMaxStack<T> {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    pub fn push(&mut self, val: T) {
        todo!("Implement push")
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!("Implement pop")
    }

    pub fn peek(&self) -> Option<T> {
        todo!("Implement peek")
    }

    pub fn min(&self) -> Option<T> {
        todo!("Implement min")
    }

    pub fn max(&self) -> Option<T> {
        todo!("Implement max")
    }

    pub fn len(&self) -> usize {
        todo!("Implement len")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Implement is_empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_then_peek_returns_pushed_value() {
        let mut stack = MinMaxStack::new();
        stack.push(42);
        assert_eq!(stack.peek(), Some(42));
    }

    #[test]
    fn pop_on_empty_returns_none() {
        let mut stack: MinMaxStack<i32> = MinMaxStack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek_on_empty_returns_none() {
        let stack: MinMaxStack<i32> = MinMaxStack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn min_on_empty_returns_none() {
        let stack: MinMaxStack<i32> = MinMaxStack::new();
        assert_eq!(stack.min(), None);
    }

    #[test]
    fn max_on_empty_returns_none() {
        let stack: MinMaxStack<i32> = MinMaxStack::new();
        assert_eq!(stack.max(), None);
    }

    #[test]
    fn min_after_multiple_pushes() {
        let mut stack = MinMaxStack::new();
        stack.push(10);
        stack.push(5);
        stack.push(20);
        stack.push(3);
        assert_eq!(stack.min(), Some(3));
    }

    #[test]
    fn max_after_multiple_pushes() {
        let mut stack = MinMaxStack::new();
        stack.push(10);
        stack.push(5);
        stack.push(20);
        stack.push(3);
        assert_eq!(stack.max(), Some(20));
    }

    #[test]
    fn min_max_remain_correct_after_popping_extremes() {
        let mut stack = MinMaxStack::new();
        stack.push(3);
        stack.push(1);
        stack.push(4);
        stack.push(1);
        stack.push(5);
        // Stack: [3, 1, 4, 1, 5] -> min=1, max=5
        assert_eq!(stack.min(), Some(1));
        assert_eq!(stack.max(), Some(5));

        stack.pop(); // removes 5 -> [3, 1, 4, 1] -> min=1, max=4
        assert_eq!(stack.min(), Some(1));
        assert_eq!(stack.max(), Some(4));

        stack.pop(); // removes 1 (top of that pair) -> [3, 1, 4] -> min=1, max=4
        assert_eq!(stack.min(), Some(1));
        assert_eq!(stack.max(), Some(4));

        stack.pop(); // removes 4 -> [3, 1] -> min=1, max=3
        assert_eq!(stack.min(), Some(1));
        assert_eq!(stack.max(), Some(3));

        stack.pop(); // removes 1 -> [3] -> min=3, max=3
        assert_eq!(stack.min(), Some(3));
        assert_eq!(stack.max(), Some(3));
    }

    #[test]
    fn len_is_correct_after_pushes_and_pops() {
        let mut stack = MinMaxStack::new();
        assert_eq!(stack.len(), 0);

        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);

        stack.pop();
        assert_eq!(stack.len(), 2);

        stack.pop();
        stack.pop();
        assert_eq!(stack.len(), 0);

        // Pop on empty shouldn't go negative
        stack.pop();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn is_empty_behavior() {
        let mut stack = MinMaxStack::new();
        assert!(stack.is_empty());

        stack.push(1);
        assert!(!stack.is_empty());

        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn works_with_f64() {
        let mut stack = MinMaxStack::new();
        stack.push(3.14);
        stack.push(2.71);
        stack.push(1.61);
        assert_eq!(stack.peek(), Some(1.61));
        assert_eq!(stack.min(), Some(1.61));
        assert_eq!(stack.max(), Some(3.14));
    }

    #[test]
    fn duplicate_values_do_not_break_min_max() {
        let mut stack = MinMaxStack::new();
        stack.push(5);
        stack.push(5);
        stack.push(5);
        assert_eq!(stack.min(), Some(5));
        assert_eq!(stack.max(), Some(5));
        stack.pop();
        assert_eq!(stack.min(), Some(5));
        assert_eq!(stack.max(), Some(5));
    }
}
