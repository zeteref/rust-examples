// Day 12: Smart Pointers
//
// Build a shared persistent linked list using `Rc` and a mutable counter
// using `Rc<RefCell<T>>` to learn interior mutability and reference counting.
//
// Learning goals:
//   - `Rc<T>` for shared ownership (reference counting)
//   - `RefCell<T>` for interior mutability (mutation through shared refs)
//   - `Deref` and `Drop` traits
//   - Building immutable data structures with structural sharing

use std::rc::Rc;
use std::cell::RefCell;

// ── Part 1: Persistent ConsList ──────────────────────────────────────────────

/// A singly-linked list using reference counting for persistence.
/// Lists can share tails: prepending to a list creates a new list without
/// modifying the original.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConsList<T> {
    Nil,
    Cons(T, Rc<ConsList<T>>),
}

impl<T> ConsList<T> {
    /// Creates an empty list.
    pub fn new() -> Rc<Self> {
        todo!("Implement new")
    }

    /// Prepends a value to the front of the list, returning a new Rc.
    /// The original list is untouched (structural sharing).
    pub fn prepend(&self, value: T) -> Rc<Self>
    where
        T: Clone,
    {
        todo!("Implement prepend")
    }

    /// Returns a reference to the first element, or None if empty.
    pub fn head(&self) -> Option<&T> {
        todo!("Implement head")
    }

    /// Returns the tail of the list as a new Rc, or None if empty.
    pub fn tail(&self) -> Option<Rc<Self>> {
        todo!("Implement tail")
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        todo!("Implement len")
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        todo!("Implement is_empty")
    }

    /// Collects all elements into a Vec of references, from head to tail.
    pub fn to_vec(&self) -> Vec<&T> {
        todo!("Implement to_vec")
    }
}

// ── Part 2: SharedCounter ────────────────────────────────────────────────────

/// A counter that can be shared across multiple owners using Rc<RefCell<...>>.
/// All clones share the same underlying integer.
#[derive(Clone)]
pub struct SharedCounter {
    value: Rc<RefCell<i32>>,
}

impl SharedCounter {
    /// Creates a new counter with an initial value of 0.
    pub fn new() -> Self {
        todo!("Implement new")
    }

    /// Increments the counter by 1.
    pub fn increment(&self) {
        todo!("Implement increment")
    }

    /// Decrements the counter by 1.
    pub fn decrement(&self) {
        todo!("Implement decrement")
    }

    /// Returns the current value.
    pub fn get(&self) -> i32 {
        todo!("Implement get")
    }

    /// Sets the counter to a specific value.
    pub fn set(&self, value: i32) {
        todo!("Implement set")
    }
}

// ── Part 3: MyBox ────────────────────────────────────────────────────────────

/// A simple smart pointer that behaves like Box<T>.
/// Implements Deref so you can use `*mybox` to access the inner value,
/// and Drop to print a message when the value is freed.
pub struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        todo!("Implement new")
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!("Implement Deref for MyBox")
    }
}

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!("Implement DerefMut for MyBox")
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        todo!("Implement Drop for MyBox")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── ConsList tests ──────────────────────────────────────────────────────

    #[test]
    fn conslist_new_is_empty() {
        let list = ConsList::<i32>::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.head(), None);
        assert_eq!(list.tail(), None);
    }

    #[test]
    fn conslist_prepend_and_head() {
        let list = ConsList::<i32>::new();
        let list = list.prepend(42);
        assert_eq!(list.head(), Some(&42));
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn conslist_multiple_prepends() {
        let mut list = ConsList::<i32>::new();
        list = list.prepend(3);
        list = list.prepend(2);
        list = list.prepend(1);
        // list = [1, 2, 3]
        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.len(), 3);
        assert_eq!(list.to_vec(), vec![&1, &2, &3]);
    }

    #[test]
    fn conslist_tail_returns_rest() {
        let mut list = ConsList::<i32>::new();
        list = list.prepend(3);
        list = list.prepend(2);
        list = list.prepend(1);

        let tail = list.tail().unwrap();
        assert_eq!(tail.head(), Some(&2));
        assert_eq!(tail.len(), 2);
    }

    #[test]
    fn conslist_structural_sharing() {
        // Demonstrates Rc sharing: two lists share the same tail
        let tail = ConsList::<i32>::new().prepend(3).prepend(2);
        let list_a = tail.prepend(1);   // [1, 2, 3]
        let list_b = tail.prepend(10);  // [10, 2, 3]

        assert_eq!(list_a.to_vec(), vec![&1, &2, &3]);
        assert_eq!(list_b.to_vec(), vec![&10, &2, &3]);

        // Both lists share the tail [2, 3]
        assert_eq!(Rc::strong_count(&tail), 3); // tail + list_a + list_b
    }

    #[test]
    fn conslist_to_vec_on_empty() {
        let list = ConsList::<i32>::new();
        assert_eq!(list.to_vec(), Vec::<&i32>::new());
    }

    // ── SharedCounter tests ─────────────────────────────────────────────────

    #[test]
    fn shared_counter_starts_at_zero() {
        let counter = SharedCounter::new();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn shared_counter_increment_and_decrement() {
        let counter = SharedCounter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
        counter.decrement();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn shared_counter_clones_share_state() {
        let counter_a = SharedCounter::new();
        let counter_b = counter_a.clone();

        counter_a.increment();
        counter_a.increment();
        assert_eq!(counter_b.get(), 2); // Shared state!
        counter_b.decrement();
        assert_eq!(counter_a.get(), 1);
    }

    #[test]
    fn shared_counter_set() {
        let counter = SharedCounter::new();
        counter.set(100);
        assert_eq!(counter.get(), 100);
    }

    #[test]
    fn shared_counter_multiple_clones() {
        let c1 = SharedCounter::new();
        let c2 = c1.clone();
        let c3 = c2.clone();

        c1.increment();
        c2.increment();
        c3.increment();

        assert_eq!(c1.get(), 3);
        assert_eq!(c2.get(), 3);
        assert_eq!(c3.get(), 3);
    }

    // ── MyBox tests ─────────────────────────────────────────────────────────

    #[test]
    fn mybox_deref_works() {
        let x = MyBox::new(42);
        assert_eq!(*x, 42);
    }

    #[test]
    fn mybox_deref_mut_works() {
        let mut x = MyBox::new(10);
        *x = 20;
        assert_eq!(*x, 20);
    }

    #[test]
    fn mybox_deref_coercion_works() {
        let x = MyBox::new(String::from("hello"));
        // Deref coercion: &MyBox<String> -> &String -> &str
        assert_eq!(x.len(), 5); // String::len called via deref coercion
    }
}
