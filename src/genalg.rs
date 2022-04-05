use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::graph::VarLifetimeGraph as Graph;
use crate::graph::VarLifetimeGraphNode as GraphNode;
use crate::lifetime::VarLifetimeId as Id;

type Color = u16;

pub struct Chromo<'a> {
    gene: Vec<Id>,
    phene: Option<u16>,
    graph: &'a Graph,
    coloring: Option<Vec<(Id, Color)>>,
}

impl<'a> Chromo<'a> {
    pub fn gene(&mut self) -> &[Id] {
        &self.gene
    }

    pub fn phene(&mut self) -> u16 {
        if let None = self.phene {
            let (phene, coloring) = self.color_graph();
            self.phene = Some(phene);
            self.coloring = Some(coloring);
        }

        self.phene.unwrap()
    }

    pub fn get_coloring(&mut self) -> &[(Id, Color)] {
        if let None = self.coloring {
            let (phene, coloring) = self.color_graph();
            self.phene = Some(phene);
            self.coloring = Some(coloring);
        }

        self.coloring.as_ref().unwrap()
    }

    fn color_graph(&self) -> (u16, Vec<(Id, Color)>) {
        let mut coloring = Vec::new();
        let mut current_colour: Color = 0;

        coloring.push((self.gene[0], current_colour));

        let mut prev_id: Id = self.gene[0];

        for &id in self.gene.iter().skip(1) {
            if self.graph.nodes[&prev_id].is_adjacent(id) {
                current_colour += 1;
            }

            coloring.push((id, current_colour));
            prev_id = id;
        }

        (current_colour + 1, coloring)
    }
}

pub struct ChromoBuilder {
    graph: Graph,
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
            graph,
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
            phene: None,
            graph: &self.graph,
            coloring: None,
        }
    }
}
