#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

// for ease of copy+paste:
// ε
// FA = (Q, Σ, δ, q0, F)

pub mod anfa;

/// Unique state id
pub type QId = usize;

/// Transition is 2nd and 3rd arguments `T` and `State` in `δ ⊆ State × T × State`
pub type Transition = Option<(Option<char>, QId)>;

/// 1 or 2 results of δ, optionally labelled. When length is 2, operation is union.
pub type DeltaQ = [Transition; 2];

/// Index of Delta is the first argument `State` in `δ ⊆ State × T × State`
pub type Delta = alloc::vec::Vec<DeltaQ>;
