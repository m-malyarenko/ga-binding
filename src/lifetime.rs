use std::cmp::Ordering;

use error::VarLifetimeError;

pub type VarLifetimeId = u16;

#[derive(Clone, Copy, Debug)]
pub struct VarLifetime {
    pub id: VarLifetimeId,
    t_def: u16,
    t_use: u16,
}

impl VarLifetime {
    pub fn new(id: VarLifetimeId, t_def: u16, t_use: u16) -> Result<VarLifetime, VarLifetimeError> {
        let var_lt = VarLifetime { id, t_def, t_use };

        if t_def <= t_use {
            Ok(var_lt)
        } else {
            Err(VarLifetimeError::UseBeforeDef(var_lt))
        }
    }

    pub fn overlap(&self, other: &VarLifetime) -> bool {
        !(self.t_use <= other.t_def || other.t_use <= self.t_def)
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

    /* Non-overlap cases */
    let var_lt_a = VarLifetime::new(1, 6, 9).unwrap();
    let var_lt_b = VarLifetime::new(2, 9, 12).unwrap();

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
