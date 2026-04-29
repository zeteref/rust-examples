// Day 10: Graph
//
// Implement a directed graph with cycle detection.
//
// Learning goals:
//   - Adjacency list representation with HashMap and Vec
//   - Graph algorithms (DFS-based cycle detection)
//   - Custom error types
//   - Efficient neighbor lookups

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum GraphError {
    NodeNotFound(usize),
}

pub struct Graph {
    // Define your fields here (e.g., adjacency list, node counter)
    _priv: (),
}

impl Graph {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    /// Adds a new node and returns its ID. Node IDs start at 0 and increment.
    pub fn add_node(&mut self) -> usize {
        todo!("Implement add_node")
    }

    /// Adds a directed edge from `from` to `to`.
    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), GraphError> {
        todo!("Implement add_edge")
    }

    /// Returns the neighbor IDs of the given node.
    pub fn neighbors(&self, node: usize) -> Result<Vec<usize>, GraphError> {
        todo!("Implement neighbors")
    }

    /// Returns true if the graph contains at least one cycle.
    pub fn has_cycle(&self) -> bool {
        todo!("Implement has_cycle")
    }

    /// Returns the total number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        todo!("Implement node_count")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node_returns_incrementing_ids_from_zero() {
        let mut g = Graph::new();
        assert_eq!(g.add_node(), 0);
        assert_eq!(g.add_node(), 1);
        assert_eq!(g.add_node(), 2);
    }

    #[test]
    fn add_edge_nonexistent_from_returns_error() {
        let mut g = Graph::new();
        g.add_node(); // only node 0
        let result = g.add_edge(42, 0);
        assert_eq!(result, Err(GraphError::NodeNotFound(42)));
    }

    #[test]
    fn add_edge_nonexistent_to_returns_error() {
        let mut g = Graph::new();
        g.add_node(); // only node 0
        let result = g.add_edge(0, 42);
        assert_eq!(result, Err(GraphError::NodeNotFound(42)));
    }

    #[test]
    fn neighbors_returns_correct_ids() {
        let mut g = Graph::new();
        g.add_node(); // 0
        g.add_node(); // 1
        g.add_node(); // 2
        g.add_edge(0, 1).unwrap();
        g.add_edge(0, 2).unwrap();

        let mut n = g.neighbors(0).unwrap();
        n.sort();
        assert_eq!(n, vec![1, 2]);
    }

    #[test]
    fn neighbors_on_node_with_no_edges_returns_empty() {
        let mut g = Graph::new();
        g.add_node(); // 0
        g.add_node(); // 1
        g.add_edge(0, 1).unwrap();

        // Node 1 has no outgoing edges
        assert_eq!(g.neighbors(1).unwrap(), vec![]);
    }

    #[test]
    fn neighbors_nonexistent_node_returns_error() {
        let g = Graph::new();
        assert_eq!(g.neighbors(42), Err(GraphError::NodeNotFound(42)));
    }

    #[test]
    fn has_cycle_on_empty_graph_is_false() {
        let g = Graph::new();
        assert!(!g.has_cycle());
    }

    #[test]
    fn has_cycle_single_node_no_self_edge_is_false() {
        let mut g = Graph::new();
        g.add_node(); // node 0, no edges
        assert!(!g.has_cycle());
    }

    #[test]
    fn has_cycle_linear_chain_is_false() {
        let mut g = Graph::new();
        // A -> B -> C
        g.add_node(); // 0
        g.add_node(); // 1
        g.add_node(); // 2
        g.add_edge(0, 1).unwrap();
        g.add_edge(1, 2).unwrap();
        assert!(!g.has_cycle());
    }

    #[test]
    fn has_cycle_triangle_is_true() {
        let mut g = Graph::new();
        // A -> B -> C -> A
        g.add_node(); // 0
        g.add_node(); // 1
        g.add_node(); // 2
        g.add_edge(0, 1).unwrap();
        g.add_edge(1, 2).unwrap();
        g.add_edge(2, 0).unwrap();
        assert!(g.has_cycle());
    }

    #[test]
    fn has_cycle_self_loop_is_true() {
        let mut g = Graph::new();
        g.add_node(); // 0
        g.add_edge(0, 0).unwrap();
        assert!(g.has_cycle());
    }

    #[test]
    fn has_cycle_multiple_components() {
        // Component 1: A -> B (no cycle)
        // Component 2: C -> D -> C (cycle)
        let mut g = Graph::new();
        g.add_node(); // 0 (A)
        g.add_node(); // 1 (B)
        g.add_node(); // 2 (C)
        g.add_node(); // 3 (D)
        g.add_edge(0, 1).unwrap();
        g.add_edge(2, 3).unwrap();
        g.add_edge(3, 2).unwrap();
        assert!(g.has_cycle());
    }

    #[test]
    fn node_count_returns_correct_count() {
        let mut g = Graph::new();
        assert_eq!(g.node_count(), 0);

        g.add_node();
        assert_eq!(g.node_count(), 1);

        g.add_node();
        g.add_node();
        assert_eq!(g.node_count(), 3);
    }
}
