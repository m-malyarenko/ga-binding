use std::collections::HashMap;
use std::fmt;
use std::panic;
use std::rc::Rc;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::graph::VarLifetimeGraph as Graph;
use crate::graph::VarLifetimeGraphNode as GraphNode;
use crate::lifetime::VarId;

type Color = u16;

#[derive(Clone, Debug)]
pub struct Chromo {
    gene: Vec<VarId>,
    graph: Rc<Graph>,

    phene: Color,
    coloring: Vec<(VarId, Color)>,
}

impl Chromo {
    pub fn new(gene: Vec<VarId>, graph: Rc<Graph>) -> Chromo {
        let (phene, coloring) = Chromo::color_graph(&gene, &graph);

        Chromo {
            gene,
            graph,
            phene,
            coloring,
        }
    }

    pub fn size(&self) -> usize {
        self.gene.len()
    }

    pub fn gene(&self) -> &[VarId] {
        &self.gene
    }

    pub fn phene(&self) -> u16 {
        self.phene
    }

    pub fn get_coloring(&self) -> &[(VarId, Color)] {
        &self.coloring
    }

    pub fn swap_genes(&mut self, locus_a: usize, locus_b: usize) {
        if locus_a >= self.gene().len() || locus_a >= self.gene().len() {
            panic!("locus is out of bounds of chromosome")
        }

        self.gene.swap(locus_a, locus_b);

        (self.phene, self.coloring) = Chromo::color_graph(&self.gene, &self.graph);
    }

    fn color_graph(gene: &[VarId], graph: &Graph) -> (u16, Vec<(VarId, Color)>) {
        let mut coloring = HashMap::new();
        let mut current_color: Color = 0;

        for id in gene {
            let node_adj_set = &graph.nodes[id].adj_set;
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

impl fmt::Display for Chromo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "gene: {:?}, phene: {}, coloring: {:?}",
            self.gene,
            self.phene(),
            self.get_coloring()
        )
    }
}

pub struct ChromoBuilder {
    graph: Rc<Graph>,
    low_deg_nodes_id: Vec<VarId>,
    high_deg_nodes_id: Vec<VarId>,
}

impl ChromoBuilder {
    pub fn new(graph: Rc<Graph>) -> ChromoBuilder {
        let mut nodes_deg: Vec<u16> = graph.nodes.iter().map(|(_, v)| v.deg).collect();
        nodes_deg.sort();

        let nodes_deg_median = nodes_deg[(nodes_deg.len() + 1) / 2];
        let (low_deg_nodes, high_deg_nodes): (
            Vec<(&VarId, &GraphNode)>,
            Vec<(&VarId, &GraphNode)>,
        ) = graph
            .nodes
            .iter()
            .partition(|(_, node)| node.deg < nodes_deg_median);

        let low_deg_nodes_id = low_deg_nodes.iter().map(|(&id, _)| id).collect();
        let high_deg_nodes_id = high_deg_nodes.iter().map(|(&id, _)| id).collect();

        ChromoBuilder {
            graph: graph,
            low_deg_nodes_id,
            high_deg_nodes_id,
        }
    }

    pub fn build_rand_chromo(&self) -> Chromo {
        let mut rng = thread_rng();

        let mut low_genes = self.low_deg_nodes_id.clone();
        let mut high_genes = self.high_deg_nodes_id.clone();

        low_genes.shuffle(&mut rng);
        high_genes.shuffle(&mut rng);

        let gene: Vec<VarId> = low_genes
            .into_iter()
            .chain(high_genes.into_iter())
            .collect();

        let (phene, coloring) = Chromo::color_graph(&gene, &self.graph);

        Chromo {
            gene,
            graph: Rc::clone(&self.graph),
            phene,
            coloring,
        }
    }
}
