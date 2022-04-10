use std::collections::HashSet;
use std::rc::Rc;

use rand::Rng;

use crate::genalg::chromo::Chromo;
use crate::graph::VarLifetimeGraph as Graph;

pub fn select_ranking(population: &[Chromo], target_size: usize) -> Vec<Chromo> {
    if population.is_empty() {
        return Vec::default();
    } else if population.len() == 1 {
        return vec![population[0].clone()];
    }

    let mut population_ranking: Vec<(&Chromo, u16)> = population
        .iter()
        .map(|chromo| (chromo, chromo.phene()))
        .collect();

    population_ranking.sort_by(|(_, fit_a), (_, fit_b)| fit_b.cmp(fit_a));

    let roulette: Vec<_> =
        std::iter::successors(Some((0_usize, 1_usize)), |&(low_limit, secor_size)| {
            Some((low_limit + secor_size, secor_size + 1))
        })
        .take(population.len())
        .zip(population_ranking)
        .map(|((low_limit, secor_size), (c, _))| ((low_limit, low_limit + secor_size), c))
        .collect();

    let mut selection_pool = Vec::new();

    let mut rand = rand::thread_rng();
    let total_size = ((1 + population.len()) / 2) * population.len();

    for _i in 0..target_size.min(population.len()) {
        let selector = rand.gen_range(0..total_size);

        for &((low_limit, high_limit), c) in &roulette {
            if (selector >= low_limit) && (selector < high_limit) {
                selection_pool.push(c.clone());
                break;
            }
        }
    }

    selection_pool
}

pub fn select_elite(population: &[Chromo], target_size: usize) -> Vec<Chromo> {
    if population.is_empty() {
        return Vec::default();
    } else if population.len() == 1 {
        return vec![population[0].clone()];
    }

    let mut population_ranking: Vec<(&Chromo, u16)> = population
        .iter()
        .map(|chromo| (chromo, chromo.phene()))
        .collect();

    population_ranking.sort_by(|(_, fit_a), (_, fit_b)| fit_b.cmp(fit_a));

    population_ranking
        .iter()
        .take(target_size.min(population.len()))
        .map(|&(c, _)| c.clone())
        .collect()
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

    let mut used_id_set = HashSet::new();

    let child_gene = parent_a
        .gene()
        .iter()
        .zip(parent_b.gene())
        .map(|(&id_a, &id_b)| {
            if used_id_set.contains(&id_a) {
                used_id_set.insert(id_b);
                id_b
            } else if used_id_set.contains(&id_b) {
                used_id_set.insert(id_a);
                id_a
            } else {
                if graph.nodes[&id_a].deg < graph.nodes[&id_b].deg {
                    used_id_set.insert(id_a);
                    id_a
                } else {
                    used_id_set.insert(id_b);
                    id_b
                }
            }
        })
        .collect();

    Chromo::new(child_gene, graph)
}
