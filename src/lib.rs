/**
 * for ease of copy+paste:
 * 
 * A finite automata is defined by the 5-tuple (
 *   Q:  Set of all states in automata,
 *   Σ:  Finite alphabet,
 *   δ:  A function accepting (1) n in Q and (2) symbol in Σ, returning a set of Q, { Q }
 *   q0: Initial n in Q
 *   F:  Set of all match states in Q
 * )
 * 
 * ε
 * 
 * FA = (Q, Σ, δ, q0, F)
 * 
 */

use std::collections::HashMap;
use std::fmt;

pub mod from;

pub type StateId = usize;
pub type Transition = Option<char>;
pub type QSet = Vec<StateId>;
pub type DeltaQ = HashMap<Transition, QSet>;
pub type Delta = Vec<DeltaQ>;

#[derive(Clone, Debug, PartialEq)]
pub struct FA {
    // Q: Unordered vertices of δ, i.e. Q = Range { start: 0, end: machine.delta.len() }
    // Σ: ???
    delta: Delta, // δ
    q0: StateId,   //q0
    f: QSet,   // F
}

impl fmt::Display for FA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Finite Automata = (\n\
            \x20  Q: {{ {} }},\n\
            \x20  Σ: {{ 0..255 }},\n\
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
    use crate::from;
    #[test]
    fn supports_formatting() {
        println!("{:#?}", from::literal('a').unwrap());
        println!("{}", from::literal('a').unwrap());
        println!(
            "The concatenation of 'apple':\n{}",
            from::concatenation(
                from::concatenation(
                    from::concatenation(
                        from::concatenation(
                            from::literal('a').unwrap(),
                            from::literal('p').unwrap(),
                        )
                        .unwrap(),
                        from::literal('p').unwrap(),
                    )
                    .unwrap(),
                    from::literal('l').unwrap(),
                )
                .unwrap(),
                from::literal('e').unwrap(),
            )
            .unwrap()
        );
        println!(
            "The machine 'orange|apple':\n{}",
            from::union(
                from::concatenation(
                    from::concatenation(
                        from::concatenation(
                            from::concatenation(
                                from::concatenation(
                                    from::literal('o').unwrap(),
                                    from::literal('r').unwrap(),
                                )
                                .unwrap(),
                                from::literal('a').unwrap(),
                            )
                            .unwrap(),
                            from::literal('n').unwrap(),
                        )
                        .unwrap(),
                        from::literal('g').unwrap(),
                    )
                    .unwrap(),
                    from::literal('e').unwrap()
                )
                .unwrap(),
                from::concatenation(
                    from::concatenation(
                        from::concatenation(
                            from::concatenation(
                                from::literal('a').unwrap(),
                                from::literal('p').unwrap(),
                            )
                            .unwrap(),
                            from::literal('p').unwrap(),
                        )
                        .unwrap(),
                        from::literal('l').unwrap(),
                    )
                    .unwrap(),
                    from::literal('e').unwrap(),
                )
                .unwrap()
            )
            .unwrap()
        );
        assert!(true, "Can't use std::fmt::Display implementation");
    }
}
