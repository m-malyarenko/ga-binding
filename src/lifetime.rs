use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs;

use error::VarLifetimeError;

pub type VarId = u16;

#[derive(Clone, Copy, Debug)]
pub struct VarLifetime {
    pub id: VarId,
    t_def: u16,
    t_use: u16,
}

impl VarLifetime {
    pub fn new(id: VarId, t_def: u16, t_use: u16) -> Result<VarLifetime, VarLifetimeError> {
        let var_lt = VarLifetime { id, t_def, t_use };

        if t_def <= t_use {
            Ok(var_lt)
        } else {
            Err(VarLifetimeError::UseBeforeDef(var_lt))
        }
    }

    pub fn overlap(&self, other: &VarLifetime) -> bool {
        !(self.t_use < other.t_def || other.t_use < self.t_def)
    }
}

impl PartialEq for VarLifetime {
    fn eq(&self, other: &VarLifetime) -> bool {
        self.t_def == other.t_def && self.t_use == other.t_use
    }
}

impl PartialOrd for VarLifetime {
    fn partial_cmp(&self, other: &VarLifetime) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        if !self.overlap(other) {
            Some(self.t_use.cmp(&other.t_def))
        } else {
            None
        }
    }
}

impl VarLifetime {
    const CSV_SEPARATOR: char = ';';

    pub fn from_csv(file: &str) -> (Vec<VarLifetime>, HashMap<VarId, String>) {
        let contents =
            fs::read_to_string(file).expect(&format!("failed to read from file '{}'", file));

        let lines: Vec<&str> = contents.lines().collect();

        if lines.len() <= 1 {
            panic!("variables lifetime csv is empty")
        }

        let var_names: HashMap<_, _> = lines[0]
            .split(VarLifetime::CSV_SEPARATOR)
            .enumerate()
            .map(|(id, name)| (id as VarId, name.to_owned()))
            .collect();

        let vars_num = var_names.len();
        let cycles_num = lines.len() - 1; // Ignore variable names CSV line

        let mut vars_lt = vec![(0_u16, 0_u16); vars_num]; // Create empty lifetime vector
        let mut vars_def_status = vec![false; vars_num];
        let mut vars_use_status = vec![false; vars_num];

        for (cycle_idx, &line) in lines.iter().skip(1).enumerate() {
            for (var_id, tag) in line
                .split(VarLifetime::CSV_SEPARATOR)
                .take(vars_num)
                .enumerate()
            {
                if !tag.is_empty() {
                    /* Active cycle */
                    if vars_use_status[var_id] {
                        panic!("variable lifetime redefinition");
                    }

                    if !vars_def_status[var_id] {
                        vars_lt[var_id].0 = cycle_idx as u16;
                        vars_def_status[var_id] = true;
                    }
                } else {
                    /* Inactive cycle */
                    if vars_def_status[var_id] && !vars_use_status[var_id] {
                        vars_lt[var_id].1 = (cycle_idx - 1) as u16;
                        vars_use_status[var_id] = true;
                    }
                }
            }
        }

        /* Handle last cycle variable usage */
        for var_id in 0..vars_num {
            if vars_def_status[var_id] && !vars_use_status[var_id] {
                vars_use_status[var_id] = true;
                vars_lt[var_id].1 = (cycles_num - 1) as u16;
            }
        }

        let vars_lt = vars_lt
            .iter()
            .enumerate()
            .map(|(id, &(t_def, t_use))| {
                VarLifetime::new(id as VarId, t_def, t_use).unwrap()
            })
            .collect();

        (vars_lt, var_names)
    }
}

impl fmt::Display for VarLifetime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, def: {}, use: {}",
            self.id, self.t_def, self.t_use
        )
    }
}

pub mod error {
    use std::error::Error;
    use std::fmt;

    use super::VarLifetime;

    #[derive(Debug)]
    pub enum VarLifetimeError {
        UseBeforeDef(VarLifetime),
    }

    impl fmt::Display for VarLifetimeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                VarLifetimeError::UseBeforeDef(var_lt) => {
                    write!(f, "invalid lifetime: use before definition: {:?}", var_lt)
                }
            }
        }
    }

    impl Error for VarLifetimeError {}
}

#[test]
fn var_lt_new_test() {
    let var_lt = VarLifetime::new(0, 1, 5);

    assert!(matches!(
        var_lt,
        Ok(VarLifetime {
            id: 0,
            t_def: 1,
            t_use: 5
        })
    ));

    let var_lt = VarLifetime::new(0, 5, 1);

    assert!(matches!(var_lt, Err(_)));
}

#[test]
fn var_lt_overlap_test() {
    /* Overlap cases */
    let var_lt_a = VarLifetime::new(1, 3, 12).unwrap();
    let var_lt_b = VarLifetime::new(2, 3, 12).unwrap();

    assert!(var_lt_a.overlap(&var_lt_b));
    assert!(var_lt_b.overlap(&var_lt_a));

    let var_lt_a = VarLifetime::new(1, 3, 12).unwrap();
    let var_lt_b = VarLifetime::new(2, 6, 17).unwrap();

    assert!(var_lt_a.overlap(&var_lt_b));
    assert!(var_lt_b.overlap(&var_lt_a));

    let var_lt_a = VarLifetime::new(1, 8, 10).unwrap();
    let var_lt_b = VarLifetime::new(2, 8, 15).unwrap();

    assert!(var_lt_a.overlap(&var_lt_b));
    assert!(var_lt_b.overlap(&var_lt_a));

    let var_lt_a = VarLifetime::new(1, 10, 15).unwrap();
    let var_lt_b = VarLifetime::new(2, 8, 15).unwrap();

    assert!(var_lt_a.overlap(&var_lt_b));
    assert!(var_lt_b.overlap(&var_lt_a));

    let var_lt_a = VarLifetime::new(1, 0, 0).unwrap();
    let var_lt_b = VarLifetime::new(2, 0, 3).unwrap();

    assert!(var_lt_a.overlap(&var_lt_b));
    assert!(var_lt_b.overlap(&var_lt_a));

    /* Non-overlap cases */
    let var_lt_a = VarLifetime::new(1, 6, 9).unwrap();
    let var_lt_b = VarLifetime::new(2, 10, 12).unwrap();

    assert!(!var_lt_a.overlap(&var_lt_b));
    assert!(!var_lt_b.overlap(&var_lt_a));

    let var_lt_a = VarLifetime::new(1, 6, 9).unwrap();
    let var_lt_b = VarLifetime::new(2, 20, 25).unwrap();

    assert!(!var_lt_a.overlap(&var_lt_b));
    assert!(!var_lt_b.overlap(&var_lt_a));
}

#[test]
fn var_lt_cmp_test() {
    let var_lt_a = VarLifetime::new(1, 3, 15).unwrap();
    let var_lt_b = VarLifetime::new(2, 3, 15).unwrap();

    assert!(var_lt_a == var_lt_b);

    assert!(matches!(
        var_lt_a.partial_cmp(&var_lt_b),
        Some(Ordering::Equal)
    ));

    let var_lt_a = VarLifetime::new(1, 6, 9).unwrap();
    let var_lt_b = VarLifetime::new(2, 20, 25).unwrap();

    assert!(matches!(
        var_lt_a.partial_cmp(&var_lt_b),
        Some(Ordering::Less)
    ));

    assert!(matches!(
        var_lt_b.partial_cmp(&var_lt_a),
        Some(Ordering::Greater)
    ));

    assert!(var_lt_a <= var_lt_b);
    assert!(var_lt_a < var_lt_b);

    let var_lt_a = VarLifetime::new(1, 3, 15).unwrap();
    let var_lt_b = VarLifetime::new(2, 14, 16).unwrap();

    assert!(matches!(var_lt_a.partial_cmp(&var_lt_b), None));
}

#[test]
fn var_lt_from_csv_test() {
    let vars_lt_file = "data/var_lifetime.csv";

    let (vars_lt, var_names) = VarLifetime::from_csv(vars_lt_file);

    for var_lt in vars_lt {
        println!("{}:\t\t{}", var_names[&var_lt.id], var_lt);
    }
}
