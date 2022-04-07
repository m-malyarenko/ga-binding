use std::rc::Rc;
use std::collections::HashSet;

use rand::Rng;

use crate::genalg::chromo::Chromo;
use crate::graph::VarLifetimeGraph as Graph;
use crate::lifetime::VarLifetimeId as Id;

pub fn select(population: &[Chromo], target_size: usize) -> Vec<Chromo> {
    if population.is_empty() {
        return Vec::default();
    } else if population.len() == 1 {
        return vec![population[0].clone()];
    }

    fn fitness(phene: u16, norm_factor: f32) -> f32 {
        phene as f32 / norm_factor
    }

    let mut population_rating: Vec<(&Chromo, f32)> = population
        .iter()
        .map(|chromo| (chromo, fitness(chromo.phene(), population.len() as f32)))
        .collect();

    population_rating
        .sort_by(|(_, f_a), (_, f_b)| f_a.partial_cmp(f_b).expect("failed to compare fittnes"));

    let roulette: Vec<_> = population_rating
        .iter()
        .enumerate()
        .map(|(sector_size, &(c, _))| (sector_size as usize, c))
        .collect();

    let mut selection_pool = Vec::new();

    let mut rand = rand::thread_rng();
    let total_size = ((1 + population.len()) / 2) * population.len();

    for _i in 0..target_size.min(population.len()) {
        for &(sector_size, c) in &roulette {
            if rand.gen_ratio(sector_size as u32, total_size as u32) {
                selection_pool.push(c.clone());
                break;
            }
        }
    }

    selection_pool
}

pub fn mutate(chromo: &mut Chromo) {
    let chromo_size = chromo.size();
    let locus_a = rand::random::<usize>() % chromo_size;
    let locus_b = rand::random::<usize>() % chromo_size;

    chromo.swap_genes(locus_a, locus_b);
}

pub fn cross(parent_a: &Chromo, parent_b: &Chromo, graph: Rc<Graph>) -> Chromo {
    if parent_a.size() != parent_a.size() {
        panic!("crossing chromosomes with different size")
    }

    let mut unused_id_set: HashSet<Id> = parent_a.gene().iter().copied().collect();

    let child_gene = parent_a
        .gene()
        .iter()
        .zip(parent_b.gene())
        .map(|(&id_a, &id_b)| {
            if !unused_id_set.contains(&id_a) {
                id_b
            } else if !unused_id_set.contains(&id_b) {
                id_a
            } else {
                if graph.nodes[&id_a].deg < graph.nodes[&id_b].deg {
                    unused_id_set.remove(&id_a);
                    id_a
                } else {
                    unused_id_set.remove(&id_b);
                    id_b
                }
            }
        })
        .collect();

    Chromo::new(child_gene, graph)
}
