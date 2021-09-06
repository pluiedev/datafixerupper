use std::{fmt::Display, ops::Add};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Lifecycle {
    Stable,
    Experimental,
    Deprecated {
        since: i32
    }
}

impl Display for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lifecycle::Stable => f.write_str("Stable")?,
            Lifecycle::Experimental => f.write_str("Experimental")?,
            Lifecycle::Deprecated { since} => write!(f, "Deprecated since {}", since)?
        }
        Ok(())
    }
}

impl Add for Lifecycle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self == Self::Experimental || rhs == Self::Experimental {
            return Self::Experimental;
        }

        if let Self::Deprecated { since: s } = rhs {
            return if let Self::Deprecated { since } = self {
                if s < since {
                    rhs
                } else {
                    self
                }
            } else {
                rhs
            }
        }
        
        Self::Stable
    }
}