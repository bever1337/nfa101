#[cfg(test)]
mod tests {
    use std::fmt;

    pub type Vertex = usize;

    pub type Transition = Option<char>;

    #[derive(Clone, Debug)]
    pub struct Edge(Vertex, Transition, Vertex);

    pub type State = Vec<Edge>;

    #[derive(Clone, Debug)]
    pub struct FA {
        matches: Vec<Vertex>,
        states: Vec<State>,
    }

    impl FA {
        fn from_literal(c: char) -> Result<FA, &'static str> {
            Ok(FA {
                matches: vec![1],
                states: vec![vec![Edge(0, Some(c), 1)], vec![]],
            })
        }

        // concatenation is a binary operation composed left-to-right
        // machine_c = machine_a ⋅ machine_b
        // machine_e = machine_c ⋅ machine_d
        // machine_o = machine_a ⋅ machine_b ⋅ ... ⋅ machine_n
        fn from_composed_concatenation_closure(machines: Vec<FA>) -> Result<FA, &'static str> {
            match machines.len() {
                0 | 1 => return Err("Concatenation requires two or more machines"),
                _ => {}
            };

            let mut machines_iterator = machines.iter();
            // "push" machine_a to machine_c
            let mut machine_c: FA = machines_iterator.next().unwrap().clone(); // length already checked
            match machine_c.matches.len() {
                0 => return Err("No match states, cannot concatenate machine"),
                _ => {}
            };

            // On first loop where n = 1, machine_n = machine_b
            for machine_n in machines_iterator {
                match machine_n.matches.len() {
                    0 => return Err("No match states, cannot concatenate machine"),
                    _ => {}
                };

                // q0 (initial state) of machine_n becomes next, unused state of machine_c
                let machine_n_next_q0 = machine_c.states.len();

                // Push epsilon transitions to q0 (initial state) of machine_n from F (match states) of machine_a
                // machine_a was already "pushed" to machine_c with corrected state IDs, so we reference F of machine_a by F of machine_c
                for machine_c_match in machine_c.matches {
                    machine_c.states[machine_c_match].push(Edge(
                        machine_c_match,
                        None,
                        machine_n_next_q0,
                    ));
                }

                // Shift machine_n δ (delta) transitions, push shifted machine_n transitions to machine_c states
                for machine_n_state in &machine_n.states {
                    let mut next_machine_c_state: State = vec![];
                    for machine_n_state_m_edge_o in machine_n_state {
                        next_machine_c_state.push(Edge(
                            machine_n_state_m_edge_o.0 + machine_n_next_q0,
                            machine_n_state_m_edge_o.1,
                            machine_n_state_m_edge_o.2 + machine_n_next_q0,
                        ))
                    }
                    machine_c.states.push(next_machine_c_state);
                }

                // Shift machine_n F (match states), set machine_c F as shifted machine_n F
                let mut next_machine_c_matches: Vec<Vertex> = vec![];
                for machine_n_match in &machine_n.matches {
                    next_machine_c_matches.push(machine_n_match + machine_n_next_q0);
                }
                machine_c.matches = next_machine_c_matches;
            }
            Ok(machine_c)
        }
    }

    impl fmt::Display for FA {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // ε
            write!(
                f,
                "Finite Automata = (\n\
                \x20  Q: {{ {} }},\n\
                \x20  Σ: {{ 0..255 }},\n\
                \x20  δ: (Q, Σ) -> [Q],{}\n\
                \x20  q0: {},\n\
                \x20  F: {{ {} }}\n)
            ",
                self.states
                    .iter()
                    .enumerate()
                    .map(|(index, _state)| { format!("{}", index) })
                    .collect::<Vec<_>>()
                    .join(", "),
                self.states
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
                0,
                self.matches
                    .iter()
                    .map(|accept_state_index| format!("{}", accept_state_index))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    #[test]
    fn character_literal() {
        let character_literal_automata: FA = FA::from_literal('f').unwrap();

        assert_eq!(
            2,
            character_literal_automata.states.len(),
            "Must only have two states, q0 (start state) and F (match state)"
        );
        assert_eq!(
            1,
            character_literal_automata.states[0].len(),
            "q0 (start state) must have one transition"
        );
        assert_eq!(
            1, character_literal_automata.states[0][0].2,
            "q0 (start state) transition must go to match state"
        );
        assert_eq!(
            'f',
            character_literal_automata.states[0][0].1.unwrap(),
            "q0 (start state) edge value must be input literal"
        );
        assert_eq!(
            0,
            character_literal_automata.states[1].len(),
            "No transitions from F (match state)"
        );
        assert_eq!(
            1, character_literal_automata.matches[0],
            "Final state must be one of F (match state)"
        );
        assert_eq!(
            1,
            character_literal_automata.matches.len(),
            "F (match state) is set of one"
        );
    }

    #[test]
    fn supports_literals_and_concatenation() {
        // Simplest NFA, a single literal
        println!("{:#?}", FA::from_literal('A'));
        // An epsilon transition is an edge with no value
        println!("{:#?}", Edge(0, None, 1));
        println!(
            "Debug: concatenate {{a}}, {{p}}, {{p}}, {{l}}, {{e}}:\n{:#?}",
            FA::from_composed_concatenation_closure(vec![
                FA::from_literal('A').unwrap(),
                FA::from_literal('P').unwrap(),
                FA::from_literal('P').unwrap(),
                FA::from_literal('L').unwrap(),
                FA::from_literal('E').unwrap(),
            ])
            .unwrap()
        );
        println!("Literal 'A':\n{}", FA::from_literal('A').unwrap());
        println!(
            "Concatenate {{a}}, {{p}}, {{p}}, {{l}}, {{e}}:\n{}",
            FA::from_composed_concatenation_closure(vec![
                FA::from_literal('A').unwrap(),
                FA::from_literal('P').unwrap(),
                FA::from_literal('P').unwrap(),
                FA::from_literal('L').unwrap(),
                FA::from_literal('E').unwrap(),
            ])
            .unwrap()
        );
        // println!("{}", FA::from_literal('A').unwrap());
    }
}
