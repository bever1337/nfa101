// symbols for ease of copy+paste:
// ε
// FA = (Q, Σ, δ, q0, F)
use std::fmt;

pub type Vertex = usize;

pub type Transition = Option<char>;

#[derive(Clone, Debug)]
pub struct Edge(Vertex, Transition, Vertex);

pub type State = Vec<Edge>;

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
#[derive(Clone, Debug)]
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
        Ok(FA {
            f: vec![1],
            delta: vec![vec![Edge(0, transition, 1)], vec![]],
            q0: 0,
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

    /**
     * Set constructions
     */
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
        for machine_c_previous_match_i in machine_c.f {
            machine_c.delta[machine_c_previous_match_i].push(Edge(
                machine_c_previous_match_i,
                None,
                machine_b_next_q0,
            ));
        }

        // Shift machine_b δ (delta) transitions, push shifted machine_b transitions to machine_c states
        for machine_b_state_m in &machine_b.delta {
            let mut next_machine_c_state: State = vec![];
            for machine_b_state_m_edge_n in machine_b_state_m {
                next_machine_c_state.push(Edge(
                    machine_b_state_m_edge_n.0 + machine_b_next_q0,
                    machine_b_state_m_edge_n.1,
                    machine_b_state_m_edge_n.2 + machine_b_next_q0,
                ))
            }
            machine_c.delta.push(next_machine_c_state);
        }

        // Shift machine_n F (match states), set machine_c F as shifted machine_n F
        let mut next_machine_c_matches: Vec<Vertex> = vec![];
        for machine_b_match_m in &machine_b.f {
            next_machine_c_matches.push(machine_b_match_m + machine_b_next_q0);
        }
        machine_c.f = next_machine_c_matches;
        Ok(machine_c)
    }

    fn from_difference(machine: FA) -> Result<FA, &'static str> {
        Err("Not implemented")
    }

    fn from_intersection(machine: FA) -> Result<FA, &'static str> {
        Err("Not implemented")
    }

    fn from_star(machine: FA) -> Result<FA, &'static str> {
        Err("Not implemented")
    }

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
        // machine_c.q0 = | machine_a.Q |
        // q0 (initial state) of machine_c is equal to the length of Q (states) of machine_a
        let machine_c_q0 = machine_a.delta.len();
        let mut machine_c: FA = FA {
            f: machine_a.f, // begin with machine_a.F
            q0: machine_c_q0,
            delta: vec![
                machine_a.delta,
                vec![vec![Edge(machine_c_q0, None, machine_a.q0)]],
            ]
            .concat(),
        };

        // shifted machine_b.q0 = | machine_c.Q |
        // q0 (initial state) of shifted machine_b is equal to the length of Q (states) of machine_c
        // Given how machine_c was initialized, we could assume that the shifted machine_b.q0 is next state after machine_c.q0, i.e.
        // shifted machine_b.q0 = | machine_a.Q | + 1
        let machine_b_shift = machine_c.delta.len();
        machine_c.delta[machine_c.q0].push(Edge(machine_c.q0, None, machine_b_shift));
        for (index_i, machine_b_previous_state_i) in machine_b.delta.iter().enumerate() {
            machine_c.delta.push(vec![]);
            for machine_b_previous_state_i_edge_j in machine_b_previous_state_i {
                machine_c.delta[index_i + machine_b_shift].push(Edge(
                    machine_b_previous_state_i_edge_j.0 + machine_b_shift,
                    machine_b_previous_state_i_edge_j.1,
                    machine_b_previous_state_i_edge_j.2 + machine_b_shift,
                ));
            }
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
            \x20  F: {{ {} }}\n)
        ",
            self.delta
                .iter()
                .enumerate()
                .map(|(index, _state)| { format!("{}", index) })
                .collect::<Vec<_>>()
                .join(", "),
            self.delta
                .iter()
                .enumerate()
                .map(|(index, _state)| {
                    [
                        String::from("\n\x20    "),
                        index.to_string(),
                        String::from(": "),
                        _state
                            .iter()
                            .map(|edge| {
                                let c = match edge.1 {
                                    Some(character) => character,
                                    None => 'ε',
                                };
                                format!("{} -> {{ {} }}", c, edge.2)
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
    use crate::{Edge, FA};

    #[test]
    fn test_from_unit() {
        let the_smallest_epsilon_automaton: FA = FA::from_unit(None).unwrap();
        let the_smallest_literal_automaton: FA = FA::from_unit(Some('a')).unwrap();
        let automata = [
            the_smallest_epsilon_automaton,
            the_smallest_literal_automaton,
        ];
        for automaton in &automata {
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
                automaton.delta[automaton.q0][0].2, automaton.f[0],
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
        assert_eq!(
            None, epsilon_automata.delta[0][0].1,
            "An epsilon transition must be represented by none"
        );
    }

    #[test]
    fn test_from_literal() {
        let character_literal_automata: FA = FA::from_literal('a').unwrap();
        assert_eq!(
            'a',
            character_literal_automata.delta[0][0].1.unwrap(),
            "Input literal must be preserved"
        );
    }

    #[test]
    fn test_from_concatenation() {
        // let machine_foo = FA::from_concatenation(vec![
        //     FA::from_literal('f').unwrap(),
        //     FA::from_literal('o').unwrap(),
        //     FA::from_literal('o').unwrap(),
        // ])
        // .unwrap();
        // let _foo = FA {
        //     q0: 0,
        //     states: vec![
        //         vec![Edge(0, Some('f'), 1)],
        //         vec![Edge(1, None, 2)],
        //         vec![Edge(2, Some('o'), 3)],
        //         vec![Edge(3, None, 4)],
        //         vec![Edge(4, Some('o'), 5)],
        //         vec![],
        //     ],
        //     matches: vec![5],
        // };
    }

    #[test]
    fn supports_literals_and_concatenation() {
        // Simplest NFA, a single literal
        // println!("{:#?}", FA::from_literal('A').unwrap());
        // println!(
        //     "{:#?}",
        //     FA::from_create_empty_states_left(FA::from_literal('A').unwrap(), 2).unwrap()
        // );
        // An epsilon transition is an edge with no value
        // println!("{:#?}", Edge(0, None, 1));
        println!(
            "{}\n\n{:#?}",
            FA::from_union(
                FA::from_literal('a').unwrap(),
                FA::from_literal('b').unwrap()
            )
            .unwrap(),
            FA::from_union(
                FA::from_literal('a').unwrap(),
                FA::from_literal('b').unwrap()
            )
            .unwrap(),
        );
        // println!(
        //     "Debug: concatenate {{a}}, {{p}}, {{p}}, {{l}}, {{e}}:\n{:#?}",
        //     FA::from_composed_concatenation(vec![
        //         FA::from_literal('A').unwrap(),
        //         FA::from_literal('P').unwrap(),
        //         FA::from_literal('P').unwrap(),
        //         FA::from_literal('L').unwrap(),
        //         FA::from_literal('E').unwrap(),
        //     ])
        //     .unwrap()
        // );
        // println!("Literal 'A':\n{}", FA::from_literal('A').unwrap());
        // println!(
        //     "Concatenate {{a}}, {{p}}, {{p}}, {{l}}, {{e}}:\n{}",
        //     FA::from_composed_concatenation(vec![
        //         FA::from_literal('A').unwrap(),
        //         FA::from_literal('P').unwrap(),
        //         FA::from_literal('P').unwrap(),
        //         FA::from_literal('L').unwrap(),
        //         FA::from_literal('E').unwrap(),
        //     ])
        //     .unwrap()
        // );
        // println!("{}", FA::from_literal('A').unwrap());
    }
}
