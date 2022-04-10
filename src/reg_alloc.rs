use std::collections::HashMap;
use std::panic;
use std::fmt;

use crate::lifetime::VarId;
use crate::lifetime::VarLifetime;

pub type RegId = u16;

pub struct RegAlloc {
    pub id: RegId,
    cycle_cells: Vec<Option<VarId>>,
}

impl RegAlloc {
    pub fn new(
        binding: &[(VarId, RegId)],
        vars_lt: &HashMap<VarId, VarLifetime>,
        cycles_num: usize,
    ) -> Vec<RegAlloc> {
        let mut binding_map: HashMap<RegId, Vec<VarId>> = HashMap::new();

        for &(var_id, reg_id) in binding {
            if let Some(binded_vars) = binding_map.get_mut(&reg_id) {
                binded_vars.push(var_id);
            } else {
                binding_map.insert(reg_id, vec![var_id]);
            }
        }

        binding_map
            .into_iter()
            .map(|(reg_id, binding)| {
                let mut cycle_cells = vec![None; cycles_num];

                for var_id in binding {
                    let var_lt = vars_lt[&var_id];

                    if var_lt.1 as usize >= cycles_num {
                        panic!("variable lifetime exceeds global cycle number");
                    }

                    for cycle_cell in &mut cycle_cells[var_lt.0 as usize..=var_lt.1 as usize] {
                        *cycle_cell = Some(var_id);
                    }
                }

                RegAlloc {
                    id: reg_id,
                    cycle_cells,
                }
            })
            .collect()
    }
}

impl fmt::Display for RegAlloc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cycle_cells_strings: Vec<String> = self.cycle_cells.iter().map(|&var_id| {
            if let Some(id) = var_id {
                id.to_string()
            } else {
                "-".to_string()
            }
        }).collect();

        let cycle_cells_string = cycle_cells_strings.join("\t");

        write!(f, "R{}:\t[ {} ]", self.id, cycle_cells_string)
    }
}
