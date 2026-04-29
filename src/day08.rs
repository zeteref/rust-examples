// Day 8: Binary Search Tree
//
// Implement a generic binary search tree using a recursive enum.
//
// Learning goals:
//   - Recursive data types with `Box<T>`
//   - Pattern matching on enums
//   - In-order tree traversal
//   - Tree height calculation

pub struct BST<T> {
    root: BSTNode<T>,
}

enum BSTNode<T> {
    Empty,
    Node(T, Box<BSTNode<T>>, Box<BSTNode<T>>),
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    /// Inserts a value into the tree. If the value already exists, behavior
    /// must be consistent (insert to the right or ignore; do not panic).
    pub fn insert(&mut self, value: T) {
        todo!("Implement insert")
    }

    /// Returns true if the value exists in the tree.
    pub fn contains(&self, value: &T) -> bool {
        todo!("Implement contains")
    }

    /// Returns a reference to the minimum value, or None if empty.
    pub fn min(&self) -> Option<&T> {
        todo!("Implement min")
    }

    /// Returns a reference to the maximum value, or None if empty.
    pub fn max(&self) -> Option<&T> {
        todo!("Implement max")
    }

    /// Returns all elements in ascending sorted order (in-order traversal).
    pub fn to_sorted_vec(&self) -> Vec<&T> {
        todo!("Implement to_sorted_vec")
    }

    /// Returns the height of the tree (0 for empty, 1 for single node).
    pub fn height(&self) -> usize {
        todo!("Implement height")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_on_empty_tree_returns_false() {
        let bst: BST<i32> = BST::new();
        assert!(!bst.contains(&5));
    }

    #[test]
    fn insert_then_contains_returns_true() {
        let mut bst = BST::new();
        bst.insert(10);
        assert!(bst.contains(&10));
    }

    #[test]
    fn contains_finds_elements_in_both_subtrees() {
        let mut bst = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(3);
        bst.insert(7);
        bst.insert(12);
        bst.insert(17);
        for val in &[3, 5, 7, 10, 12, 15, 17] {
            assert!(bst.contains(val), "Expected tree to contain {}", val);
        }
    }

    #[test]
    fn min_on_empty_tree_returns_none() {
        let bst: BST<i32> = BST::new();
        assert_eq!(bst.min(), None);
    }

    #[test]
    fn max_on_empty_tree_returns_none() {
        let bst: BST<i32> = BST::new();
        assert_eq!(bst.max(), None);
    }

    #[test]
    fn min_returns_correct_value() {
        let mut bst = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(3);
        bst.insert(7);
        assert_eq!(bst.min(), Some(&3));
    }

    #[test]
    fn max_returns_correct_value() {
        let mut bst = BST::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(3);
        bst.insert(7);
        assert_eq!(bst.max(), Some(&15));
    }

    #[test]
    fn to_sorted_vec_on_empty_returns_empty() {
        let bst: BST<i32> = BST::new();
        assert_eq!(bst.to_sorted_vec(), Vec::<&i32>::new());
    }

    #[test]
    fn to_sorted_vec_returns_ascending_order() {
        let mut bst = BST::new();
        for val in &[5, 3, 7, 1, 4] {
            bst.insert(*val);
        }
        let sorted = bst.to_sorted_vec();
        assert_eq!(sorted, vec![&1, &3, &4, &5, &7]);
    }

    #[test]
    fn height_of_empty_tree_is_zero() {
        let bst: BST<i32> = BST::new();
        assert_eq!(bst.height(), 0);
    }

    #[test]
    fn height_of_single_node_is_one() {
        let mut bst = BST::new();
        bst.insert(42);
        assert_eq!(bst.height(), 1);
    }

    #[test]
    fn height_of_balanced_tree() {
        // Insert in an order that produces a balanced tree:
        //       4
        //     /   \
        //    2     6
        //   / \   / \
        //  1   3 5   7
        let mut bst = BST::new();
        for val in &[4, 2, 6, 1, 3, 5, 7] {
            bst.insert(*val);
        }
        assert_eq!(bst.height(), 3);
    }

    #[test]
    fn height_of_unbalanced_tree() {
        // Insert in ascending order: 1 -> 2 -> 3 -> 4 -> 5
        let mut bst = BST::new();
        for val in 1..=5 {
            bst.insert(val);
        }
        assert_eq!(bst.height(), 5);
    }

    #[test]
    fn duplicate_values_do_not_panic() {
        // The spec requires duplicates to not panic. Either ignore or insert.
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(5);
        bst.insert(5);
        // Tree must still be valid
        assert!(bst.contains(&5));
        let sorted = bst.to_sorted_vec();
        // If duplicates are ignored: [&5]
        // If duplicates are inserted: [&5, &5, &5]
        // Both are acceptable.
        assert!(!sorted.is_empty());
    }
}
