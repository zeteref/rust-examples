use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub enum ConsList<T> {
    Nil,
    Cons(T, Rc<ConsList<T>>),
}

impl<T> ConsList<T> {
    pub fn new() -> Rc<Self> {
        Rc::new(ConsList::Nil)
    }

    pub fn prepend(list: &Rc<Self>, value: T) -> Rc<Self> {
        Rc::new(ConsList::Cons(value, Rc::clone(list)))
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            ConsList::Nil => None,
            ConsList::Cons(v, _) => Some(v),
        }
    }

    pub fn tail(&self) -> Option<Rc<Self>> {
        match self {
            ConsList::Nil => None,
            ConsList::Cons(_, tail) => Some(Rc::clone(tail)),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ConsList::Nil => 0,
            ConsList::Cons(_, tail) => 1 + tail.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, ConsList::Nil)
    }

    pub fn to_vec(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut current = self;
        while let ConsList::Cons(v, tail) = current {
            result.push(v);
            current = tail;
        }
        result
    }
}

#[derive(Clone)]
pub struct SharedCounter {
    value: Rc<RefCell<i32>>,
}

impl SharedCounter {
    pub fn new() -> Self {
        SharedCounter {
            value: Rc::new(RefCell::new(0)),
        }
    }

    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    pub fn decrement(&self) {
        *self.value.borrow_mut() -= 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }

    pub fn set(&self, value: i32) {
        *self.value.borrow_mut() = value;
    }
}

pub struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        MyBox { value }
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {}
}
