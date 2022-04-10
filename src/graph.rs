use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use crate::lifetime::{VarLifetime, VarLifetimeId};

#[derive(Debug)]
pub struct VarLifetimeGraphNode {
    pub deg: u16,
    pub adj_set: HashSet<VarLifetimeId>,
}

impl VarLifetimeGraphNode {
    pub fn is_adjacent(&self, id: VarLifetimeId) -> bool {
        self.adj_set.contains(&id)
    }
}

impl fmt::Display for VarLifetimeGraphNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "degree: {}, adjacency set: {:?}", self.deg, self.adj_set)
    }
}

#[derive(Debug)]
pub struct VarLifetimeGraph {
    pub nodes: HashMap<VarLifetimeId, VarLifetimeGraphNode>,
}

impl VarLifetimeGraph {
    pub fn new(vars_lt: &[VarLifetime]) -> VarLifetimeGraph {
        let nodes = vars_lt
            .iter()
            .map(|var_lt| {
                let id = var_lt.id;
                let adj_set: HashSet<VarLifetimeId> = vars_lt
                    .iter()
                    .filter(|&v| v.id != var_lt.id && v.overlap(var_lt))
                    .map(|&v| v.id)
                    .collect();
                let deg = adj_set.len() as u16;

                (id, VarLifetimeGraphNode { deg, adj_set })
            })
            .collect();

        VarLifetimeGraph { nodes }
    }
}

impl fmt::Display for VarLifetimeGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_string: String = String::new();

        for (&id, node) in &self.nodes {
            output_string.push_str(&format!("id: {}, node: [{}]\n", id, node));
        }

        output_string.pop(); // Delete last newline character

        write!(f, "{}", output_string)
    }
}

impl VarLifetimeGraph {
    pub fn to_dot(&self, names: &HashMap<VarLifetimeId, String>) -> String {
        let mut dot_string = String::new();
        let mut used_edges = HashSet::new();

        dot_string += &"graph {\n";

        for (&id, node) in &self.nodes {
            dot_string.push('\t');

            if let Some(name) = names.get(&id) {
                dot_string += name;
            } else {
                dot_string += &id.to_string();
            };

            dot_string += &" -- { ";

            for &adj_node_id in &node.adj_set {
                if !used_edges.contains(&(adj_node_id, id)) {
                    if let Some(name) = names.get(&adj_node_id) {
                        dot_string += name;
                    } else {
                        dot_string += &id.to_string();
                    };

                    dot_string += &" ";
                    used_edges.insert((id, adj_node_id));
                }
            }

            dot_string += &"}\n";
        }

        dot_string += &"}\n";

        dot_string
    }
}
