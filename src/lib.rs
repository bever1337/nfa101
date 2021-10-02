#[cfg(test)]
mod tests {
    // use regex_syntax::hir::{Hir, Visitor};
    // use regex_syntax::hir::{visit, Hir, Visitor};
    // use regex_syntax::Parser;
    // use std::collections::HashMap;
    use std::fmt;

    pub type Vertex = usize;

    pub type Transition = Option<char>;

    #[derive(Clone, Debug)]
    pub struct Edge(Vertex, Transition, Vertex);

    // impl Edge {
    //     fn is_epsilon(&self) -> bool {
    //         match &self.1 {
    //             Some(_) => false,
    //             None => true,
    //         }
    //     }
    // }

    // impl fmt::Display for Edge {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         let c = match self.1 {
    //             Some(character) => character,
    //             None => 'ε',
    //         };
    //         write!(f, "( {} ) -- {} --> ( {} )", self.0, c, self.2)
    //     }
    // }

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

        fn from_composed_concatenation_closure(machines: Vec<FA>) -> Result<FA, &'static str> {
            match machines.len() {
                0 | 1 => return Err("Concatenation requires two or more machines"),
                _ => {}
            };

            let mut machines_iterator = machines.iter();
            let machine_one: &FA = machines_iterator.next().unwrap();
            match machine_one.matches.len() {
                0 => return Err("No match states, cannot concatenate machine"),
                _ => {}
            };

            let mut machine_three = FA {
                matches: machine_one.matches.to_vec(),
                states: machine_one.states.to_vec(),
            };

            for machine_n in machines_iterator {
                match machine_n.matches.len() {
                    0 => return Err("No match states, cannot concatenate machine"),
                    _ => {}
                };
                let last_index = machine_three.states.len();
                for previous_match in machine_three.matches {
                    machine_three.states[previous_match].append(&mut vec![Edge(
                        previous_match,
                        None,
                        last_index,
                    )]);
                }
                machine_three.states.append(
                    &mut machine_n
                        .states
                        .iter()
                        .map(|edges| {
                            edges
                                .iter()
                                .map(|edge| Edge(edge.0 + last_index, edge.1, edge.2 + last_index))
                                .collect::<Vec<Edge>>()
                        })
                        .collect::<Vec<State>>(),
                );
                machine_three.matches = machine_n
                    .matches
                    .iter()
                    .map(|match_index| match_index + last_index)
                    .collect::<Vec<Vertex>>();
            }
            Ok(machine_three)
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
