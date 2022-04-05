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
