use std::collections::HashMap;
use std::rc::Rc;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::graph::VarLifetimeGraph as Graph;
use crate::graph::VarLifetimeGraphNode as GraphNode;
use crate::lifetime::VarLifetimeId as Id;

type Color = u16;

#[derive(Clone)]
pub struct Chromo {
    gene: Vec<Id>,
    graph: Rc<Graph>,
}

impl Chromo {
    pub fn size(&self) -> usize {
        self.gene.len()
    }

    pub fn gene(&self) -> &[Id] {
        &self.gene
    }

    pub fn phene(&self) -> u16 {
        self.color_graph().0
    }

    pub fn get_coloring(&self) -> Vec<(Id, Color)> {
        self.color_graph().1
    }

    pub fn swap_genes(&mut self, locus_a: usize, locus_b: usize) {
        if locus_a >= self.gene().len() || locus_a >= self.gene().len() {
            panic!("locus is out of bounds of chromosome")
        }
    
        self.gene.swap(locus_a, locus_b);
    }

    fn color_graph(&self) -> (u16, Vec<(Id, Color)>) {
        let mut coloring = HashMap::new();
        let mut current_color: Color = 0;

        for id in &self.gene {
            let node_adj_set = &self.graph.nodes[id].adj_set;
            let node_adj_coloring: Vec<Color> = node_adj_set
                .iter()
                .filter_map(|adj_id| coloring.get(adj_id))
                .copied()
                .collect();

            if node_adj_coloring
                .iter()
                .any(|&color| color == current_color)
            {
                current_color += 1;
            }

            coloring.insert(*id, current_color);
        }

        (current_color + 1, coloring.into_iter().collect())
    }
}

pub struct ChromoBuilder {
    graph: Rc<Graph>,
    low_deg_nodes_id: Vec<Id>,
    high_deg_nodes_id: Vec<Id>,
}

impl ChromoBuilder {
    pub fn new(graph: Graph) -> ChromoBuilder {
        let mut nodes_deg: Vec<u16> = graph.nodes.iter().map(|(_, v)| v.deg).collect();
        nodes_deg.sort();

        let nodes_deg_median = nodes_deg[(nodes_deg.len() + 1) / 2];
        let (low_deg_nodes, high_deg_nodes): (Vec<(&Id, &GraphNode)>, Vec<(&Id, &GraphNode)>) =
            graph
                .nodes
                .iter()
                .partition(|(_, node)| node.deg < nodes_deg_median);

        let low_deg_nodes_id = low_deg_nodes.iter().map(|(&id, _)| id).collect();
        let high_deg_nodes_id = high_deg_nodes.iter().map(|(&id, _)| id).collect();

        ChromoBuilder {
            graph: Rc::new(graph),
            low_deg_nodes_id,
            high_deg_nodes_id,
        }
    }

    pub fn yield_chromo(&self) -> Chromo {
        let mut rng = thread_rng();

        let mut low_genes = self.low_deg_nodes_id.clone();
        let mut high_genes = self.high_deg_nodes_id.clone();

        low_genes.shuffle(&mut rng);
        high_genes.shuffle(&mut rng);

        let gene = low_genes
            .into_iter()
            .chain(high_genes.into_iter())
            .collect();

        Chromo {
            gene,
            graph: Rc::clone(&self.graph),
        }
    }
}