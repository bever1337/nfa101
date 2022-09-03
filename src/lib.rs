#![no_std]

extern crate alloc;

// for ease of copy+paste:
// ε
// FA = (Q, Σ, δ, q0, F)

// size of QId
// size of label
use alloc::vec;

/// Unique state id
/// TODO: Compile a regex large enough to overflow
pub type QId = usize;

/// A transition along an optional label to zero, one, or two States.
/// When a label is `None`, transition is an epsilon transition
/// and it always advances to its final states. When QId is `None`,
/// there is no transition. Transition is ordered. If `Transition.1[1]`
/// is `Some(QId)`, then `Transition.1[0]` must also be `Some(QId)`.
/// i.e. a union operation is when both `Option<QId>` are `Some(QId)`.
pub type Transition = (Option<char>, [Option<QId>; 2]);

/// DeltaFunction is a vector of ordered transitions that satisfy
/// the function `δ ⊆ State × T × State`. An index of `DeltaFunction`
/// is the first `State` paramter in the function. See `Transition`.
pub type DeltaFunction = vec::Vec<Transition>;

/// The initial and final states of an expression: [q0, f]
pub type AutomataRef = [QId; 2];

#[derive(Debug)]
pub struct ANFA {
    pub automata_refs: vec::Vec<AutomataRef>,
    pub delta: DeltaFunction,
}

impl ANFA {
    /// The ANFA constructor does not return a valid automaton.
    /// ANFA must be constructed by a static compiler factory
    fn new() -> ANFA {
        ANFA {
            automata_refs: vec::Vec::with_capacity(u32::MAX as usize),
            delta: vec::Vec::with_capacity(u32::MAX as usize),
        }
    }
}

pub mod compilers;
