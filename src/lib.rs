// symbols for ease of copy+paste:
// ε
// FA = (Q, Σ, δ, q0, F)
use std::collections::HashMap;
use std::fmt;

pub type Vertex = usize;

pub type Transition = Option<char>;

#[derive(Clone, Debug)]
pub struct Edge(Vertex, Transition, Vertex);

pub type State = HashMap<Transition, Vec<Vertex>>;

pub type Delta = Vec<State>;

pub type Matches = Vec<Vertex>;

/**
 * A finite automata is defined by the 5-tuple (
 *   Q:  Set of all states in automata,
 *   Σ:  Finite alphabet,
 *   δ:  A function accepting (1) n in Q and (2) symbol in Σ, returning a set of Q, { Q }
 *   q0: Initial n in Q
 *   F:  Set of all match states in Q
 * )
 */
#[derive(Clone, Debug, PartialEq)]
pub struct FA {
    // Q: Unordered vertices of δ, i.e. Q = { 0 .. delta.len() }
    // Σ: ???
    delta: Delta, // δ
    q0: Vertex,   //q0
    f: Matches,   // F
}

impl FA {
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
    fn from_unit(transition: Option<char>) -> Result<FA, &'static str> {
        let mut delta_q0: State = HashMap::new();
        let delta_q1: State = HashMap::new();
        assert!(
            delta_q0.insert(transition, vec![1]).is_none(),
            "Unexpected error, new HashMap somehow had old value"
        );
        Ok(FA {
            delta: vec![delta_q0, delta_q1],
            q0: 0,
            f: vec![1],
        })
    }

    // ( 0 ) -- ε --> (( 1 ))
    fn from_epsilon() -> Result<FA, &'static str> {
        FA::from_unit(None)
    }

    // ( 0 ) -- 'a' --> (( 1 ))
    fn from_literal(c: char) -> Result<FA, &'static str> {
        FA::from_unit(Some(c))
    }

    fn from_complement(machine: FA) -> Result<FA, &'static str> {
        Err("Not implemented")
    }

    fn from_concatenation(machine_a: FA, machine_b: FA) -> Result<FA, &'static str> {
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
            let mut machine_c_state_n: State = HashMap::new();
            for (&transition_symbol, to_states) in delta_i.iter() {
                if let Some(_) = machine_c_state_n.insert(
                    transition_symbol,
                    to_states
                        .iter()
                        .map(|state_id| state_id + machine_b_next_q0)
                        .collect::<Vec<Vertex>>(),
                ) {
                    // sanity check, brand-new hash map
                    return Err("Unexpected error, new HashMap somehow had old value");
                };
            }
            machine_c.delta.push(machine_c_state_n);
        }

        // Shift machine_n F (match states), set machine_c F as shifted machine_n F
        let mut next_machine_c_matches: Vec<Vertex> = vec![];
        for match_i in &machine_b.f {
            next_machine_c_matches.push(match_i + machine_b_next_q0);
        }
        machine_c.f = next_machine_c_matches;
        Ok(machine_c)
    }

    // fn from_difference(machine: FA) -> Result<FA, &'static str> {
    //     Err("Not implemented")
    // }

    // fn from_intersection(machine: FA) -> Result<FA, &'static str> {
    //     Err("Not implemented")
    // }

    // fn from_star(machine: FA) -> Result<FA, &'static str> {
    //     Err("Not implemented")
    // }

    // machine_c = machina_a ∪ machine_b
    // given machine_a:
    //   ( 0 ) -- A --> (( 1 ))
    // and machine_b:
    //   ( 0 ) -- B --> (( 1 ))
    // the union is machine_c:
    //       /-- ε --> ( 0 ) -- A --> (( 1 ))
    //   ( 2 )
    //       \-- ε --> ( 3 ) -- B --> (( 4 ))
    fn from_union(machine_a: FA, machine_b: FA) -> Result<FA, &'static str> {
        // construct first 'leg' of machine_c
        //       /-- ε --> ( 0 ) -- A --> (( 1 ))
        //   ( 2 )
        // add first epsilon transition from q0 of machine_c to the former q0 of machine_a
        let mut machine_c_delta_q0: State = HashMap::new();
        assert!(
            machine_c_delta_q0
                .insert(None, vec![machine_a.q0])
                .is_none(),
            "Unexpected error, previous value cannot exist in new hash map"
        );
        let mut machine_c: FA = FA {
            f: machine_a.f,            // begin with machine_a.F
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

        // construct second 'leg' of machine_c
        //   ( 2 )
        //       \-- ε --> ( 3 ) -- B --> (( 4 ))
        // recall delta is a function where (Q, Σ) -> [Q]
        // Q is an index in delta returning a HashMap. Σ (transition) is a key of HashMap returning a vector of state ids [Q]
        // for { A: [1] } in [{ A: [1] }, { ε: [2] }, { B: [3] }, { }]
        for machine_b_delta_q in machine_b.delta.iter() {
            let mut machine_c_delta_q: State = HashMap::new();
            // for (A, [1]) in { A: [1] }
            for (&machine_b_delta_q_transition, machine_b_delta_q_transition_q_set) in
                machine_b_delta_q
            {
                // shift machine_b [Q] for current delta function
                let machine_c_delta_q_transition_q_set = machine_b_delta_q_transition_q_set
                    .iter()
                    .map(|machine_b_delta_q_transition_q| {
                        machine_b_delta_q_transition_q + machine_b_shift
                    })
                    .collect::<Vec<Vertex>>();

                // insert shifted [Q] into transition map
                assert!(
                    machine_c_delta_q
                        .insert(
                            machine_b_delta_q_transition,
                            machine_c_delta_q_transition_q_set,
                        )
                        .is_none(),
                    "Unexpected error, previous value cannot exist in new hash map"
                );
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

        Ok(machine_c)
    }
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
    use crate::FA;

    #[test]
    fn test_from_unit() {
        let the_smallest_epsilon_automaton: FA = FA::from_unit(None).unwrap();
        let the_smallest_literal_automaton: FA = FA::from_unit(Some('a')).unwrap();
        for automaton in &[
            the_smallest_epsilon_automaton,
            the_smallest_literal_automaton,
        ] {
            assert_eq!(2, automaton.delta.len(), "Must only have two states");
            assert_eq!(
                1,
                automaton.f.len(),
                "Must only have one state in set of F (match states)"
            );
            assert_ne!(
                automaton.q0, automaton.f[0],
                "q0 (start state) must not be in set of F (match states)."
            );
            assert_eq!(
                1,
                automaton.delta[automaton.q0].len(),
                "Machine requires one transition from q0"
            );
            assert_eq!(
                automaton.delta[automaton.q0].values().next().unwrap()[0],
                automaton.f[0],
                "Machine must transition from q0 to one of F"
            );
            assert_eq!(
                0,
                automaton.delta[automaton.f[0]].len(),
                "Machine must not transition from match states"
            );
        }
    }

    #[test]
    fn test_from_epsilon() {
        let epsilon_automata: FA = FA::from_epsilon().unwrap();
        assert!(
            epsilon_automata.delta[epsilon_automata.q0].contains_key(&None),
            "An epsilon transition must be represented by none"
        );
    }

    #[test]
    fn test_from_literal() {
        let character_literal_automata: FA = FA::from_literal('a').unwrap();
        assert!(
            character_literal_automata.delta[character_literal_automata.q0]
                .contains_key(&Some('a')),
            "Input literal must be preserved"
        );
    }

    #[test]
    fn test_from_concatenation() {
        let machine_a = FA::from_literal('a').unwrap();
        let machine_b = FA::from_literal('b').unwrap();
        let machine_ab = FA::from_concatenation(
            FA::from_literal('a').unwrap(),
            FA::from_literal('b').unwrap(),
        )
        .unwrap();
        let machine_ab_c = FA::from_concatenation(
            FA::from_concatenation(
                FA::from_literal('a').unwrap(),
                FA::from_literal('b').unwrap(),
            )
            .unwrap(),
            FA::from_literal('c').unwrap(),
        )
        .unwrap();
        let machine_a_bc = FA::from_concatenation(
            FA::from_literal('a').unwrap(),
            FA::from_concatenation(
                FA::from_literal('b').unwrap(),
                FA::from_literal('c').unwrap(),
            )
            .unwrap(),
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

        let machine_a_or_b_and_c = FA::from_concatenation(
            // Union constructs a machine where |F| > 1
            FA::from_union(
                FA::from_literal('a').unwrap(),
                FA::from_literal('b').unwrap(),
            )
            .unwrap(),
            FA::from_literal('c').unwrap(),
        )
        .unwrap();
        assert_eq!(
            1,
            machine_a_or_b_and_c.f.len(),
            "Concatenation must point all machine_a F at machine_b q0"
        );
    }

    #[test]
    fn supports_formatting() {
        println!("{:#?}", FA::from_literal('a').unwrap());
        println!("{}", FA::from_literal('a').unwrap());
        println!(
            "The concatenation of 'apple':\n{}",
            FA::from_concatenation(
                FA::from_concatenation(
                    FA::from_concatenation(
                        FA::from_concatenation(
                            FA::from_literal('a').unwrap(),
                            FA::from_literal('p').unwrap(),
                        )
                        .unwrap(),
                        FA::from_literal('p').unwrap(),
                    )
                    .unwrap(),
                    FA::from_literal('l').unwrap(),
                )
                .unwrap(),
                FA::from_literal('e').unwrap(),
            )
            .unwrap()
        );
        println!(
            "The machine 'orange|apple':\n{}",
            FA::from_union(
                FA::from_concatenation(
                    FA::from_concatenation(
                        FA::from_concatenation(
                            FA::from_concatenation(
                                FA::from_concatenation(
                                    FA::from_literal('o').unwrap(),
                                    FA::from_literal('r').unwrap(),
                                )
                                .unwrap(),
                                FA::from_literal('a').unwrap(),
                            )
                            .unwrap(),
                            FA::from_literal('n').unwrap(),
                        )
                        .unwrap(),
                        FA::from_literal('g').unwrap(),
                    )
                    .unwrap(),
                    FA::from_literal('e').unwrap()
                )
                .unwrap(),
                FA::from_concatenation(
                    FA::from_concatenation(
                        FA::from_concatenation(
                            FA::from_concatenation(
                                FA::from_literal('a').unwrap(),
                                FA::from_literal('p').unwrap(),
                            )
                            .unwrap(),
                            FA::from_literal('p').unwrap(),
                        )
                        .unwrap(),
                        FA::from_literal('l').unwrap(),
                    )
                    .unwrap(),
                    FA::from_literal('e').unwrap(),
                )
                .unwrap()
            )
            .unwrap()
        );
    }
}
