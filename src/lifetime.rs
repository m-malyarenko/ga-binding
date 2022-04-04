use std::cmp::Ordering;

use error::VarLifetimeError;

#[derive(Debug)]
pub struct VarLifetime {
    id: u16,
    t_def: u16,
    t_use: u16,
}

impl VarLifetime {
    pub fn new(id: u16, t_def: u16, t_use: u16) -> Result<VarLifetime, VarLifetimeError> {
        let var_lt = VarLifetime { id, t_def, t_use };

        if t_def <= t_use {
            Ok(var_lt)
        } else {
            Err(VarLifetimeError::UseBeforeDef(var_lt))
        }
    }

    pub fn overlap(&self, other: &VarLifetime) -> bool {
        self.t_def < other.t_use || self.t_use > other.t_def
    }
}

impl PartialEq for VarLifetime {
    fn eq(&self, other: &VarLifetime) -> bool {
        self.t_def == other.t_def && self.t_use == other.t_use
    }
}

impl PartialOrd for VarLifetime {
    fn partial_cmp(&self, other: &VarLifetime) -> Option<Ordering> {
        if !self.overlap(other) {
            Some(self.t_use.cmp(&other.t_def))
        } else {
            None
        }
    }
}

pub struct VarLifetimeTable {
    n_clk: u16,
    vars_lt: Vec<VarLifetime>,
}

impl VarLifetimeTable {
    pub fn new(
        n_clk: u16,
        vars_lt: Vec<VarLifetime>,
    ) -> Result<VarLifetimeTable, error::VarLifetimeError> {
        let mut valid_vars_lt = true;

        for var_lt in &vars_lt {
            if var_lt.t_use > n_clk {
                eprintln!(
                    "lifetime of {} out of bounds: max lifetime {}, varibale use time {}",
                    var_lt.id, n_clk, var_lt.t_use
                );
                valid_vars_lt = false;
            }
        }

        if valid_vars_lt {
            Ok(VarLifetimeTable { n_clk, vars_lt })
        } else {
            Err(error::VarLifetimeError::LifetimeOutOfBounds)
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
        LifetimeOutOfBounds,
    }

    impl fmt::Display for VarLifetimeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                VarLifetimeError::UseBeforeDef(var_lt) => {
                    write!(f, "invalid lifetime: use before definition: {:?}", var_lt)
                }
                VarLifetimeError::LifetimeOutOfBounds => {
                    write!(f, "variable lifetime of out of bounds")
                }
            }
        }
    }

    impl Error for VarLifetimeError {}
}
