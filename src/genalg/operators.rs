use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use rand::Rng;

use crate::genalg::chromo::Chromo;
use crate::graph::VarLifetimeGraph as Graph;
use crate::lifetime::VarId;

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

pub fn cross(
    parents: (&Chromo, &Chromo),
    graph: Rc<Graph>,
    eval_matrix: &HashMap<(VarId, VarId), u16>,
) -> Vec<Chromo> {
    if parents.0.size() != parents.1.size() {
        panic!("crossing chromosomes with different size")
    }

    fn inner(
        dominant: &Chromo,
        recessive: &Chromo,
        cross_point: usize,
        graph: &Graph,
        eval_matrix: &HashMap<(VarId, VarId), u16>,
    ) -> Vec<VarId> {
        let dominant_gene = dominant.gene().split_at(cross_point);
        let recessive_gene = recessive.gene().split_at(cross_point);

        let dominant_gene_tail: HashSet<VarId> = dominant_gene.1.iter().copied().collect();
        let recessive_gene_tail: HashSet<VarId> = recessive_gene.1.iter().copied().collect();

        let mut gene_tail_diff = &dominant_gene_tail - &recessive_gene_tail;
        let gene_tail_intersect = &dominant_gene_tail & &recessive_gene_tail;

        /* Child gene with None on conflict IDs */
        let child_gene: Vec<Option<VarId>> = dominant_gene
            .0
            .iter()
            .map(|&id| Some(id))
            .chain(recessive_gene.1.iter().map(|id| {
                if gene_tail_intersect.contains(id) {
                    Some(*id)
                } else {
                    None
                }
            }))
            .collect();

        /* Resolving conflict IDs */
        let mut prev_id = 0;
        let child_gene: Vec<VarId> = child_gene
            .iter()
            .enumerate()
            .map(|(locus, &opt_id)| {
                if let Some(id) = opt_id {
                    prev_id = id;
                    id
                } else {
                    if locus == 0 {
                        /* If no previous IDs - choose node ID with min degree */
                        let &id = gene_tail_diff
                            .iter()
                            .min_by_key(|&id| graph.nodes[id].deg)
                            .expect("empty graph");
                        prev_id = id;
                        gene_tail_diff.remove(&id);
                        id
                    } else {
                        /* Select node ID with by minimal evaluation wight */
                        let &id = gene_tail_diff
                            .iter()
                            .min_by_key(|&id| eval_matrix[&(prev_id, *id)])
                            .unwrap();
                        prev_id = id;
                        gene_tail_diff.remove(&id);
                        id
                    }
                }
            })
            .collect();

        child_gene
    }

    let mut rng = rand::thread_rng();

    let cross_point = rng.gen_range(0..parents.0.size());

    let child_a_gene = inner(&parents.0, &parents.1, cross_point, &graph, eval_matrix);
    let child_b_gene = inner(&parents.1, &parents.0, cross_point, &graph, eval_matrix);

    vec![
        Chromo::new(child_a_gene, graph.clone()),
        Chromo::new(child_b_gene, graph),
    ]
}
