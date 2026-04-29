pub struct BST<T> {
    root: BSTNode<T>,
}

enum BSTNode<T> {
    Empty,
    Node(T, Box<BSTNode<T>>, Box<BSTNode<T>>),
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST {
            root: BSTNode::Empty,
        }
    }

    pub fn insert(&mut self, value: T) {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(node: &mut BSTNode<T>, value: T) {
        match node {
            BSTNode::Empty => {
                *node = BSTNode::Node(
                    value,
                    Box::new(BSTNode::Empty),
                    Box::new(BSTNode::Empty),
                );
            }
            BSTNode::Node(v, left, right) => {
                if value < *v {
                    Self::insert_node(left, value);
                } else if value > *v {
                    Self::insert_node(right, value);
                }
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        Self::contains_node(&self.root, value)
    }

    fn contains_node(node: &BSTNode<T>, value: &T) -> bool {
        match node {
            BSTNode::Empty => false,
            BSTNode::Node(v, left, right) => {
                if value < v {
                    Self::contains_node(left, value)
                } else if value > v {
                    Self::contains_node(right, value)
                } else {
                    true
                }
            }
        }
    }

    pub fn min(&self) -> Option<&T> {
        Self::min_node(&self.root)
    }

    fn min_node(node: &BSTNode<T>) -> Option<&T> {
        match node {
            BSTNode::Empty => None,
            BSTNode::Node(v, left, _) => Self::min_node(left).or(Some(v)),
        }
    }

    pub fn max(&self) -> Option<&T> {
        Self::max_node(&self.root)
    }

    fn max_node(node: &BSTNode<T>) -> Option<&T> {
        match node {
            BSTNode::Empty => None,
            BSTNode::Node(v, _, right) => Self::max_node(right).or(Some(v)),
        }
    }

    pub fn to_sorted_vec(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::in_order(&self.root, &mut result);
        result
    }

    fn in_order<'a>(node: &'a BSTNode<T>, result: &mut Vec<&'a T>) {
        if let BSTNode::Node(v, left, right) = node {
            Self::in_order(left, result);
            result.push(v);
            Self::in_order(right, result);
        }
    }

    pub fn height(&self) -> usize {
        Self::height_node(&self.root)
    }

    fn height_node(node: &BSTNode<T>) -> usize {
        match node {
            BSTNode::Empty => 0,
            BSTNode::Node(_, left, right) => {
                1 + std::cmp::max(Self::height_node(left), Self::height_node(right))
            }
        }
    }
}
