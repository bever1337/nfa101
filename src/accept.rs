//! Acceptors, the smallest automata!

use crate::{DeltaQ, FA};
use std::collections::HashMap;

///
/// Returns an automaton accepting an epsilon
/// 
/// # Examples
///
/// Example 1:
/// 
/// ```rust
/// use automata::{accept, FA};
/// let epsilon_automaton_result: Result<FA, &'static str> = accept::epsilon();
/// match epsilon_automaton_result {
///     Ok(epsilon_automaton) => {
///         println!("Epsilon automaton: {}", epsilon_automaton);
///     },
///     Err(err) => {
///         println!("Error creating automaton: {}", err);
///     }
/// };
/// ```
///
/// # Implementation
///
/// ```ignore
/// {
///     delta: [],
///     q0: 0,
///     f: [0]
/// }
/// ```
///
/// # Definition
///
/// ```ignore
/// (
///     Q: { 0 },
///     Σ: { any character },
///     δ: none,
///     q0: 0,
///     F: { 0 }
/// )
/// ```
///
/// # Diagram
///
/// ```ignore
/// -- ε --> (( 0 ))
/// ```
///
pub fn epsilon() -> Result<FA, &'static str> {
    Ok(FA {
        delta: vec![HashMap::new()],
        q0: 0,
        f: vec![0],
    })
}

///
/// Returns an automaton accepting a literal
/// 
/// # Examples
///
/// Example 1:
/// 
/// ```rust
/// use automata::{accept, FA};
/// let literal_automaton_result: Result<FA, &'static str> = accept::literal('a');
/// match literal_automaton_result {
///     Ok(literal_automaton) => {
///         println!("Literal automaton: {}", literal_automaton);
///     },
///     Err(err) => {
///         println!("Error creating automaton: {}", err);
///     }
/// };
/// ```
///
/// # Implementation
///
/// ```ignore
/// {
///     delta: [
///         { Some(a): [1] }
///     ],
///     q0: 0,
///     f: [1]
/// }
/// ```
///
/// # Definition
///
/// ```ignore
/// (
///     Q: { 0, 1 },
///     Σ: { any character },
///     δ: (0 X 'a') => { 1 }
///     q0: 0,
///     F: { 1 }
/// )
/// ```
///
/// # Diagram
///
/// ```ignore
/// ----> ( 0 ) -- 'a' --> (( 1 ))
/// ```
///
pub fn literal(c: char) -> Result<FA, &'static str> {
    let mut delta_q0: DeltaQ = HashMap::new();
    if let Some(_) = delta_q0.insert(Some(c), vec![1]) {
        return Err("Unexpected error, new HashMap somehow had old value");
    }
    Ok(FA {
        delta: vec![delta_q0, HashMap::new()],
        q0: 0,
        f: vec![1],
    })
}

///
/// Returns an automaton accepting nothing
/// 
/// # Examples
///
/// Example 1:
/// 
/// ```rust
/// use automata::{accept, FA};
/// let nothing_automaton_result: Result<FA, &'static str> = accept::nothing();
/// match nothing_automaton_result {
///     Ok(nothing_automaton) => {
///         println!("Nothing automaton: {}", nothing_automaton);
///     },
///     Err(err) => {
///         println!("Error creating automaton: {}", err);
///     }
/// };
/// ```
///
/// # Implementation
///
/// ```ignore
/// {
///     delta: [
///       {}
///     ],
///     q0: 0,
///     f: []
/// }
/// ```
///
/// # Definition
///
/// ```ignore
/// (
///   Q: { 0 },
///   Σ: { any character },
///   δ: none,
///   q0: 0,
///   F: {}
/// )
/// ```
///
/// # Diagram
///
/// ```ignore
/// ----> ( 0 )
/// ```
///
pub fn nothing() -> Result<FA, &'static str> {
    Ok(FA {
        delta: vec![HashMap::new()],
        q0: 0,
        f: vec![],
    })
}

#[cfg(test)]
mod tests {
    use crate::{accept, FA};

    #[test]
    fn test_accepts_epsilon() {
        let accepts_epsilon_automaton: FA = accept::epsilon().unwrap();
        assert_eq!(
            1,
            accepts_epsilon_automaton.delta.len(),
            "Must only have one state"
        );
        assert_eq!(
            1,
            accepts_epsilon_automaton.f.len(),
            "Must only have one state in set of F (match states)"
        );
        assert_eq!(
            accepts_epsilon_automaton.q0, accepts_epsilon_automaton.f[0],
            "q0 (start state) must be in set of F (match states)."
        );
        assert_eq!(
            0,
            accepts_epsilon_automaton.delta[0].len(),
            "Machine must have zero transitions"
        );
    }

    #[test]
    fn test_from_literal() {
        let character_literal_automata: FA = accept::literal('a').unwrap();
        assert!(
            character_literal_automata.delta[character_literal_automata.q0]
                .contains_key(&Some('a')),
            "Input literal must be preserved"
        );
        assert_eq!(
            2,
            character_literal_automata.delta.len(),
            "Must only have two states"
        );
        assert_eq!(
            1,
            character_literal_automata.f.len(),
            "Must only have one state in set of F (match states)"
        );
        assert_ne!(
            character_literal_automata.q0, character_literal_automata.f[0],
            "q0 (start state) must not be in set of F (match states)."
        );
        assert_eq!(
            1,
            character_literal_automata.delta[character_literal_automata.q0].len(),
            "Machine requires one transition from q0"
        );
        assert_eq!(
            character_literal_automata.delta[character_literal_automata.q0]
                .values()
                .next()
                .unwrap()[0],
            character_literal_automata.f[0],
            "Machine must transition from q0 to one of F"
        );
        assert_eq!(
            0,
            character_literal_automata.delta[character_literal_automata.f[0]].len(),
            "Machine must not transition from match states"
        );
    }

    #[test]
    fn test_accepts_nothing() {
        let accepts_nothing_automaton: FA = accept::nothing().unwrap();
        assert_eq!(
            1,
            accepts_nothing_automaton.delta.len(),
            "Machine must have only one state"
        );
        assert_eq!(
            0,
            accepts_nothing_automaton.delta[0].len(),
            "Machine must have zero transitions"
        );
        assert_eq!(
            0, accepts_nothing_automaton.q0,
            "q0 (start state) must be first state"
        );
        assert_eq!(
            0,
            accepts_nothing_automaton.f.len(),
            "F must be empty set, must not have match states"
        );
    }
}
