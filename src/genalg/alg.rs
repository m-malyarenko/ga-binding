use rand::Rng;
use std::rc::Rc;

use crate::genalg::chromo::{Chromo, ChromoBuilder};
use crate::genalg::operators;
use crate::graph::VarLifetimeGraph as Graph;

struct GenAlg {
    population: Vec<Chromo>,
    selection_pool: Vec<Chromo>,
    new_gen: Vec<Chromo>,

    graph: Rc<Graph>,
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
            new_gen: Vec::new(),
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
        self.selection_pool = operators::select(&self.population, target_size);
    }

    pub fn cross(&mut self) {
        if self.selection_pool.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let pool_size = self.selection_pool.len();

        self.new_gen.clear();

        for parent_a in &self.selection_pool {
            if rng.gen_ratio(self.cross_ratio.0, self.cross_ratio.1) {
                let parent_b = &self.selection_pool[rng.gen_range(0..pool_size)];
                self.new_gen
                    .push(operators::cross(parent_a, parent_b, Rc::clone(&self.graph)));
            }
        }

        self.selection_pool.clear();
    }

    pub fn mutate(&mut self) {
        if self.new_gen.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();

        for chromo in &mut self.new_gen {
            if rng.gen_ratio(self.mutation_ratio.0, self.mutation_ratio.1) {
                operators::mutate(chromo);
            }
        }
    }

    pub fn accept(&mut self) {
        self.population.extend(self.new_gen.drain(..));
    }

    pub fn reduce(&mut self, target_size: usize) {
        // FIXME Сократить популяцию по функции элитарного отбора
        self.population.truncate(target_size);
    }

    pub fn pick_winner(&self) -> Option<&Chromo> {
        self.population.iter().max_by_key(|&chromo| chromo.phene())
    }
}
