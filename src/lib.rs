// for ease of copy+paste:
// ε
// FA = (Q, Σ, δ, q0, F)
use std::collections::HashMap;
use std::fmt;

pub mod accept;
pub mod from;

/// Unique id representing a state, `usize`
pub type StateId = usize;

// A transition between states, `Option<char>`
// pub type Transition = Option<char>;

/// Set of states, `Vec<StateId>`
pub type QSet = Vec<StateId>;

/// Second-half of delta function, `HashMap<Option<char>, QSet>`
pub type DeltaQ = HashMap<Option<char>, QSet>;

/// First-half of delta function, `Vec<DeltaQ>`
pub type Delta = Vec<DeltaQ>;

///
/// A struct (mostly) representing the formal definition of a finite automaton
///
/// # Definition
///
/// A machine is defined by the 5-tuple
/// - Q:  Set of all states in automata,
/// - Σ:  Finite alphabet,
/// - δ:  A function accepting 1 of each Q and Σ, returning a set of Q
/// - q0: Initial Q
/// - F:  Set of all match states in Q
///
/// # Examples
///
/// Example 1:
///
/// ```
/// use automata::{accept, from, FA};
/// use std::ops::{Range};
///
/// let machine_a = accept::literal('a').unwrap();
/// ```
///
/// Define the automaton using rust types and the `machine_a` object:
/// - Q:  `Range { start: 0, end: machine_a.delta.len() }`
/// - Σ:  `Option<char>`,
/// - δ:  `machine_a.delta[machine_a.q0].get(&Some('a')).unwrap()`
/// - q0: `machine_a.q0`
/// - F:  `machine_a.f`
///
#[derive(Clone, Debug, PartialEq)]
pub struct FA {
    pub delta: Delta,
    pub q0: StateId,
    pub f: QSet,
}

impl fmt::Display for FA {
    /// Formats the definition of an automaton by 5-tuple
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Finite Automata = (\n\
            \x20  Q: {{ {} }},\n\
            \x20  Σ: {{ Any Opton<char> }},\n\
            \x20  δ: (Q, Σ) -> [Q],{}\n\
            \x20  q0: {},\n\
            \x20  F: {{ {} }}\n)",
            self.delta
                .iter()
                .enumerate()
                .map(|(index, _state)| { format!("{}", index) })
                .collect::<Vec<_>>()
                .join(", "),
            self.delta
                .iter()
                .enumerate()
                .map(|(delta_q, delta_transitions)| {
                    [
                        String::from("\n\x20    "),
                        delta_q.to_string(),
                        String::from(": Q"),
                        delta_transitions
                            .iter()
                            .map(|(&transition, q_set)| {
                                let c = match transition {
                                    Some(character) => character,
                                    None => 'ε',
                                };
                                format!(
                                    ", {}: Σ -> {{ {} }}",
                                    c,
                                    q_set
                                        .iter()
                                        .map(|q| q.to_string())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                )
                            })
                            .collect::<Vec<_>>()
                            .join(", "),
                    ]
                    .join("")
                })
                .collect::<Vec<_>>()
                .join(""),
            self.q0,
            self.f
                .iter()
                .map(|accept_state_index| format!("{}", accept_state_index))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::accept;
    #[test]
    fn supports_formatting() {
        println!("{:#?}", accept::literal('a').unwrap());
        assert!(true, "Can't use debug format");

        println!("{}", accept::literal('a').unwrap());
        assert!(true, "Can't use std::fmt::Display implementation");
    }
}
