use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum GraphError {
    NodeNotFound(usize),
}

pub struct Graph {
    adj: HashMap<usize, Vec<usize>>,
    next_id: usize,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_node(&mut self) -> usize {
        let id = self.next_id;
        self.adj.insert(id, Vec::new());
        self.next_id += 1;
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), GraphError> {
        if !self.adj.contains_key(&from) {
            return Err(GraphError::NodeNotFound(from));
        }
        if !self.adj.contains_key(&to) {
            return Err(GraphError::NodeNotFound(to));
        }
        self.adj.get_mut(&from).unwrap().push(to);
        Ok(())
    }

    pub fn neighbors(&self, node: usize) -> Result<Vec<usize>, GraphError> {
        self.adj
            .get(&node)
            .cloned()
            .ok_or(GraphError::NodeNotFound(node))
    }

    pub fn has_cycle(&self) -> bool {
        let node_ids: Vec<usize> = self.adj.keys().copied().collect();
        let mut visited = HashMap::new();
        let mut in_stack = HashMap::new();
        for &id in &node_ids {
            visited.insert(id, false);
            in_stack.insert(id, false);
        }

        for &node in &node_ids {
            if !visited[&node] {
                if Self::dfs_cycle(node, &self.adj, &mut visited, &mut in_stack) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs_cycle(
        node: usize,
        adj: &HashMap<usize, Vec<usize>>,
        visited: &mut HashMap<usize, bool>,
        in_stack: &mut HashMap<usize, bool>,
    ) -> bool {
        visited.insert(node, true);
        in_stack.insert(node, true);

        if let Some(neighbors) = adj.get(&node) {
            for &neighbor in neighbors {
                if !visited[&neighbor] {
                    if Self::dfs_cycle(neighbor, adj, visited, in_stack) {
                        return true;
                    }
                } else if *in_stack.get(&neighbor).unwrap_or(&false) {
                    return true;
                }
            }
        }

        in_stack.insert(node, false);
        false
    }

    pub fn node_count(&self) -> usize {
        self.adj.len()
    }
}
