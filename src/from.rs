use crate::{DeltaQ, QSet, Transition, FA};
use std::collections::HashMap;

/**
 * the smallest automata = (
 *   Q: { 0, 1 },
 *   Σ: { .. },
 *   δ: [
 *     (0 X ε) => { 1 }
 *   ],
 *   q0: 0,
 *   F: { 1 })
 * )
 */

// -- ε --> (( 0 ))
pub fn empty() -> Result<FA, &'static str> {
    Ok(FA {
        delta: vec![HashMap::new()],
        q0: 0,
        f: vec![0],
    })
}

// ----> ( 0 ) -- 'a' --> (( 1 ))
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

// concatenation is a binary operation where machine_c = machine_a ⋅ machine_b
pub fn concatenation(machine_a: FA, machine_b: FA) -> Result<FA, &'static str> {
    match (machine_a.f.len(), machine_b.f.len()) {
        (0, _) | (_, 0) => return Err("No match states, cannot concatenate machine"),
        _ => {}
    };

    let mut machine_c: FA = machine_a;

    // machine_b.q0 = |machine_a.Q| (machine_b's initial state ID will become length of states of machine_a)
    let machine_b_next_q0 = machine_c.delta.len();
    // point previous machine_a.F (set of match states) to machine_b.q0 (initial state)
    for &match_i in &machine_c.f {
        if let Some(epsilon_transitions) = machine_c.delta[match_i].get_mut(&None) {
            epsilon_transitions.push(machine_b_next_q0);
        } else {
            if let Some(_) = machine_c.delta[match_i].insert(None, vec![machine_b_next_q0]) {
                // sanity check, machine_c.delta[match_n] matched None, so insert can't return Some
                return Err("Unexpected error, new HashMap somehow had old value");
            }
        }
    }

    // Shift machine_b δ (delta) transitions, push shifted machine_b transitions to machine_c states
    for delta_i in machine_b.delta {
        let mut machine_c_state_n: DeltaQ = HashMap::new();
        for (&transition_symbol, to_states) in delta_i.iter() {
            if let Some(_) = machine_c_state_n.insert(
                transition_symbol,
                to_states
                    .iter()
                    .map(|state_id| state_id + machine_b_next_q0)
                    .collect::<QSet>(),
            ) {
                // sanity check, brand-new hash map
                return Err("Unexpected error, new HashMap somehow had old value");
            };
        }
        machine_c.delta.push(machine_c_state_n);
    }

    // Shift machine_n F (match states), set machine_c F as shifted machine_n F
    let mut next_machine_c_matches: QSet = vec![];
    for match_i in &machine_b.f {
        next_machine_c_matches.push(match_i + machine_b_next_q0);
    }
    machine_c.f = next_machine_c_matches;
    Ok(machine_c)
}

// star is a unary operation where machine_b = machine_a*
pub fn star(machine_a: FA) -> Result<FA, &'static str> {
    let mut machine_b = machine_a;
    machine_b.q0 = machine_b.delta.len();
    machine_b.delta.push(HashMap::new());
    if let Some(_) = machine_b.delta[machine_b.q0].insert(None, vec![0]) {
        // Sanity check
        return Err("Unexpected error, new HashMap somehow had old value");
    }
    // for each match state in f, add epsilon transition to q0
    for q in &machine_b.f {
        if let Some(machine_b_delta_q_epsilon_q_set) = machine_b.delta[*q].get_mut(&None) {
            machine_b_delta_q_epsilon_q_set.push(machine_b.q0);
        } else {
            if let Some(_) = machine_b.delta[*q].insert(None, vec![machine_b.q0]) {
                // Sanity check
                return Err("Unexpected error, new HashMap somehow had old value");
            }
        }
    }
    machine_b.f.push(machine_b.q0);
    Ok(machine_b)
}

// union is a binary operation where machine_c = machina_a ∪ machine_b
pub fn union(machine_a: FA, machine_b: FA) -> Result<FA, &'static str> {
    // given machine_a:
    //   ( 0 ) -- A --> (( 1 ))
    // construct the first 'leg' of machine_c:
    //       /-- ε --> ( 0 ) -- A --> (( 1 ))
    //   ( 2 )

    // add first epsilon transition from q0 of machine_c to the former q0 of machine_a
    let mut machine_c_delta_q0: DeltaQ = HashMap::new();
    if let Some(_) = machine_c_delta_q0.insert(None, vec![machine_a.q0]) {
        return Err("Unexpected error, previous value cannot exist in new hash map");
    }
    let mut machine_c: FA = FA {
        f: machine_a.f,
        q0: machine_a.delta.len(), // q0 (initial state) of machine_c is equal to the length of Q (states) of machine_a, i.e. machine_c.q0 = | machine_a.Q |
        delta: vec![machine_a.delta, vec![machine_c_delta_q0]].concat(),
    };

    // q0 (initial state) of shifted machine_b is equal to the length of Q (states) of machine_c, i.e. shifted machine_b.q0 = | machine_c.Q |
    // Given how machine_c was initialized, we could assume that the shifted machine_b.q0 is next state after machine_c.q0, i.e. shifted machine_b.q0 = | machine_a.Q | + 1
    let machine_b_shift = machine_c.delta.len();
    // add second epsilon transition from q0 of machine_c to the shifted q0 of machine_b
    machine_c.delta[machine_c.q0]
        .get_mut(&None)
        .unwrap() // unwrapping because we already asserted this key/value
        .push(machine_b_shift);

    // given machine_b:
    //   ( 0 ) -- B --> (( 1 ))
    // construct the second 'leg' of machine_c:
    //   ( 2 )
    //       \-- ε --> ( 3 ) -- B --> (( 4 ))
    // recall delta is a function where (Q, Σ) -> [Q]
    // Q is an index in delta returning a HashMap. Σ (transition) is a key of HashMap returning a vector of state ids [Q]
    // for { A: [1] } in [{ A: [1] }, { ε: [2] }, { B: [3] }, { }]
    for machine_b_delta_q in machine_b.delta.iter() {
        let mut machine_c_delta_q: DeltaQ = HashMap::new();
        // for (A, [1]) in { A: [1] }
        for (&machine_b_delta_q_transition, machine_b_delta_q_transition_q_set) in machine_b_delta_q
        {
            // shift machine_b [Q] for current delta function
            let machine_c_delta_q_transition_q_set = machine_b_delta_q_transition_q_set
                .iter()
                .map(|machine_b_delta_q_transition_q| {
                    machine_b_delta_q_transition_q + machine_b_shift
                })
                .collect::<QSet>();

            // insert shifted [Q] into transition map
            if let Some(_) = machine_c_delta_q.insert(
                machine_b_delta_q_transition,
                machine_c_delta_q_transition_q_set,
            ) {
                return Err("Unexpected error, previous value cannot exist in new hash map");
            }
        }

        // add shifted delta function from machine_b onto machine_c
        machine_c.delta.push(machine_c_delta_q);
    }

    // add shifted machine_b.F (match states) to machine_c.F (match states)
    for machine_b_previous_match_i in machine_b.f {
        machine_c
            .f
            .push(machine_b_previous_match_i + machine_b_shift);
    }

    // the union is machine_c:
    //       /-- ε --> ( 0 ) -- A --> (( 1 ))
    //   ( 2 )
    //       \-- ε --> ( 3 ) -- B --> (( 4 ))
    Ok(machine_c)
}

#[cfg(test)]
mod tests {
    use crate::{from, FA};

    #[test]
    fn test_from_empty() {
        let empty_automaton: FA = from::empty().unwrap();
        assert_eq!(1, empty_automaton.delta.len(), "Must only have one state");
        assert_eq!(
            1,
            empty_automaton.f.len(),
            "Must only have one state in set of F (match states)"
        );
        assert_eq!(
            empty_automaton.q0, empty_automaton.f[0],
            "q0 (start state) must be in set of F (match states)."
        );
        assert_eq!(
            0,
            empty_automaton.delta[0].len(),
            "Machine must have zero transitions"
        );
    }

    #[test]
    fn test_from_literal() {
        let character_literal_automata: FA = from::literal('a').unwrap();
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
    fn test_from_concatenation() {
        let machine_a = from::literal('a').unwrap();
        let machine_b = from::literal('b').unwrap();
        let machine_ab =
            from::concatenation(from::literal('a').unwrap(), from::literal('b').unwrap()).unwrap();
        let machine_ab_c = from::concatenation(
            from::concatenation(from::literal('a').unwrap(), from::literal('b').unwrap()).unwrap(),
            from::literal('c').unwrap(),
        )
        .unwrap();
        let machine_a_bc = from::concatenation(
            from::literal('a').unwrap(),
            from::concatenation(from::literal('b').unwrap(), from::literal('c').unwrap()).unwrap(),
        )
        .unwrap();

        assert_eq!(
            machine_a.delta.len() + machine_b.delta.len(),
            machine_ab.delta.len(),
            "| machine_c Q | = | machine_a Q | + | machine_b Q |"
        );
        assert_eq!(
            machine_b.f.len(),
            machine_ab.f.len(),
            "| machine_c F | = | machine_b F |"
        );
        assert_eq!(
            machine_a.q0, machine_ab.q0,
            "q0 must be preserved by concatenation"
        );
        for match_i in machine_a.f {
            let machine_ab_delta_q_epsilon = machine_ab.delta[match_i].get(&None);
            // ε
            // FA = (Q, Σ, δ, q0, F)
            assert!(
                machine_ab_delta_q_epsilon.is_some(),
                "For f in machine_a.F, machine_c.δ(f, ε) => [ machine_c.F ] i.e. Delta of machine ab for each of F of machine a must have an epsilon transition."
            );
            // there is probably a better assertion that doesn't rely on knowing the internal workings of concatenation
            // example, test that machine_a 'f' points to a node with an edge pointing to one of machine c 'f'
            assert!(
                machine_ab_delta_q_epsilon
                    .unwrap()
                    .contains(&machine_a.delta.len()),
                "F of machine_a points to q0 of machine_b"
            );
        }

        // machine_d = machine_a ⋅ machine_b ⋅ machine_c
        // machine_d = (machine_a ⋅ machine_b) ⋅ machine_c
        // machine_d = machine_a ⋅ (machine_b ⋅ machine_c)
        assert_eq!(
            machine_ab_c, machine_a_bc,
            "Concatenation must be associative"
        );

        let machine_a_or_b_and_c = from::concatenation(
            // Union constructs a machine where |F| > 1
            from::union(from::literal('a').unwrap(), from::literal('b').unwrap()).unwrap(),
            from::literal('c').unwrap(),
        )
        .unwrap();
        assert_eq!(
            1,
            machine_a_or_b_and_c.f.len(),
            "Concatenation must point all machine_a F at machine_b q0"
        );
    }

    #[test]
    fn test_from_star() {
        let machine_a = from::literal('a').unwrap();
        let machine_b = from::star(from::literal('a').unwrap()).unwrap();
        assert_eq!(
            machine_a.delta.len() + 1,
            machine_b.delta.len(),
            "Star operation must only create one new state"
        );
        assert_eq!(
            machine_a.f.len() + 1,
            machine_b.f.len(),
            "Star operation must only create one new match state"
        );
        for q in machine_a.f {
            assert!(
                machine_b.delta[q]
                    .get(&None)
                    .unwrap()
                    .contains(&machine_b.q0),
                "q in F of machine_a must have epsilon transition to q0 of machine_b"
            );
        }
        assert!(
            machine_b.f.contains(&machine_b.q0),
            "q0 after star operation must be match state"
        );
        assert!(
            machine_b.delta[machine_b.q0]
                .get(&None)
                .unwrap()
                .contains(&machine_a.q0),
            "q0 of machine_b must transition to q0 of machine_a"
        );
    }

    #[test]
    fn test_from_union() {
        let machine_a = from::literal('a').unwrap();
        let machine_b = from::literal('b').unwrap();
        let machine_a_or_b =
            from::union(from::literal('a').unwrap(), from::literal('b').unwrap()).unwrap();
        assert_eq!(
            machine_a_or_b.delta.len(),
            machine_a.delta.len() + machine_b.delta.len() + 1,
            "Union of two machines must only create one new state"
        );
        assert_eq!(
            machine_a_or_b.delta[machine_a_or_b.q0]
                .get(&None)
                .unwrap()
                .len(),
            2,
            "q0 of machine_c must have two epsilon transitions"
        );
        assert_eq!(
          machine_a_or_b.f.len(),
          machine_a.f.len() + machine_b.f.len(),
          "Union must result in same number of match states, | machine_c F | = | machine_a F | + | machine_b F |"
        );
    }
}
