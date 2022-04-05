use std::collections::HashMap;
use std::collections::HashSet;

use crate::lifetime::{VarLifetime, VarLifetimeId};

#[derive(Debug)]
pub struct VarLifetimeGraphNode {
    pub deg: u16,
    adj_set: HashSet<VarLifetimeId>,
}

impl VarLifetimeGraphNode {
    pub fn is_adjacent(&self, id: VarLifetimeId) -> bool {
        self.adj_set.contains(&id)
    }
}

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
