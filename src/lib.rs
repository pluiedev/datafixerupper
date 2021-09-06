use dynamicops::DynamicOps;
use std::{fmt::Display, ops::Add};

pub mod encoder;
pub mod dynamicops;
pub mod dataresult;
pub mod lifecycle;
pub mod mapencoder;


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

trait Keys {
    fn keys<T, I>(&self, ops: impl DynamicOps<T>) -> I
    where
        I: Iterator<Item = T>;

    /*
    static Keyable forStrings(final Supplier<Stream<String>> keys) {
        return new Keyable() {
            @Override
            public <T> Stream<T> keys(final DynamicOps<T> ops) {
                return keys.get().map(ops::createString);
            }
        };
    }
    */
}
