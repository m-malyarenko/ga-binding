use rand::Rng;
use std::collections::HashMap;
use std::rc::Rc;

use crate::genalg::chromo::{Chromo, ChromoBuilder};
use crate::genalg::operators;
use crate::graph::VarLifetimeGraph as Graph;
use crate::lifetime::VarId;

pub struct GenAlg {
    pub population: Vec<Chromo>,
    pub selection_pool: Vec<Chromo>,
    pub next_gen: Vec<Chromo>,

    graph: Rc<Graph>,
    eval_matrix: HashMap<(VarId, VarId), u16>,
    chromo_builder: ChromoBuilder,

    mutation_ratio: (u32, u32),
    cross_ratio: (u32, u32),
}

impl GenAlg {
    pub fn new(
        graph: Rc<Graph>,
        chromo_builder: ChromoBuilder,
        mutation_ratio: (u32, u32),
        cross_ratio: (u32, u32),
    ) -> GenAlg {
        GenAlg {
            population: Vec::new(),
            selection_pool: Vec::new(),
            next_gen: Vec::new(),
            eval_matrix: GenAlg::create_eval_matrix(&graph),
            graph,
            chromo_builder,
            mutation_ratio,
            cross_ratio,
        }
    }

    pub fn gen(&mut self, population_size: usize) {
        self.population = std::iter::from_fn(|| Some(self.chromo_builder.build_rand_chromo()))
            .take(population_size)
            .collect();
    }

    pub fn select(&mut self, target_size: usize) {
        self.selection_pool = operators::select_ranking(&self.population, target_size);
    }

    pub fn cross(&mut self) {
        if self.selection_pool.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let pool_size = self.selection_pool.len();

        self.next_gen.clear();

        for parent_a in &self.selection_pool {
            if rng.gen_ratio(self.cross_ratio.0, self.cross_ratio.1) {
                let parent_b = &self.selection_pool[rng.gen_range(0..pool_size)];
                self.next_gen.extend(operators::cross(
                    (parent_a, parent_b),
                    Rc::clone(&self.graph),
                    &self.eval_matrix,
                ));
            }
        }

        self.selection_pool.clear();
    }

    pub fn mutate(&mut self) {
        if self.next_gen.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();

        for chromo in &mut self.next_gen {
            if rng.gen_ratio(self.mutation_ratio.0, self.mutation_ratio.1) {
                operators::mutate(chromo);
            }
        }
    }

    pub fn accept(&mut self) {
        self.population.extend(self.next_gen.drain(..));
    }

    pub fn reduce(&mut self, target_size: usize) {
        self.population = operators::select_elite(&self.population, target_size);
    }

    pub fn pick_winner(&self) -> Option<&Chromo> {
        self.population.iter().min_by_key(|&chromo| chromo.phene())
    }

    fn create_eval_matrix(graph: &Graph) -> HashMap<(VarId, VarId), u16> {
        let nodes_num = graph.nodes.len() as u16;

        graph
            .nodes
            .iter()
            .flat_map(|(id_a, node_a)| {
                graph.nodes.iter().filter_map(|(id_b, node_b)| {
                    if *id_a != *id_b {
                        let weight = node_a.deg
                            + node_b.deg
                            + if node_a.is_adjacent(*id_b) {
                                nodes_num / 2 // TODO Придумать осознанный штрафной коэффициент
                            } else {
                                0
                            };
                        Some(((*id_a, *id_b), weight))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}
