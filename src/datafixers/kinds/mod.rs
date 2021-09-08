#[macro_use]
mod generated;
use std::collections::*;

// reexport
pub use self::generated::*;

kind1!(Vec);
kind1!(VecDeque);
kind1!(HashSet);
kind1!(BTreeSet);
kind1!(Option);
kind2!(Result);
kind2!(HashMap);
kind2!(BTreeMap);