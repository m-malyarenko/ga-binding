use rand::random;

use crate::genalg::chromo::{Chromo, ChromoBuilder};

struct GenAlg {
    population: Vec<Chromo>,
    selection_pool: Vec<Chromo>,
    new_gen: Vec<Chromo>,

    chromo_builder: ChromoBuilder,

    mutation_ratio: (u32, u32),
    cross_ratio: (u32, u32),
}

impl GenAlg {
    pub fn new(
        chromo_builder: ChromoBuilder,
        mutation_ratio: (u32, u32),
        cross_ratio: (u32, u32),
    ) -> GenAlg {
        GenAlg {
            population: Vec::new(),
            selection_pool: Vec::new(),
            new_gen: Vec::new(),
            chromo_builder,
            mutation_ratio,
            cross_ratio,
        }
    }

    pub fn gen(&mut self, population_size: usize) {}

    pub fn select(&mut self) {
        todo!()
    }

    pub fn cross(&mut self) {
        todo!()
    }

    pub fn mutate(&mut self) {
        todo!()
    }

    pub fn accept(&mut self) {
        todo!()
    }

    pub fn reduce(&mut self) {
        todo!()
    }

    pub fn pick_winner(&self) -> Option<&Chromo> {
        todo!()
    }
}
