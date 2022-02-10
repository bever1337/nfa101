use crate::{Delta, DeltaQ, QId};

/// Reference to automata initial and final states
#[derive(Debug, PartialEq)]
pub struct AutomataRef {
    /// initial state
    pub q0: QId,
    /// final state
    pub f: QId,
}

/// Augmented non-deterministic finite automaton
///
/// Augmented non-deterministic finite automata are defined by the 4-tuple:
/// - state, finite set of states
/// - delta, `δ ⊆ State × T × State` is a labeled transition relation with labels `T = Σ ⊎ {0, 1, ε}`
/// - q0, initial state
/// - f, final state
#[derive(Debug, PartialEq)]
pub struct ANFA {
    /// `δ ⊆ State × T × State` is a labeled transition relation with labels `T = Σ ⊎ {0, 1, ε}`
    pub delta: Delta,
    /// initial state
    pub q0: Option<QId>,
    /// final state
    pub f: Option<QId>,
}

impl ANFA {
    /// Returns an ANFA with no state. Must be finalized with `in_and_fin`
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// ```
    pub fn new() -> ANFA {
        ANFA {
            delta: vec![],
            q0: None,
            f: None,
        }
    }

    /// "Finalizes" ANFA by using initial and final states of sub-automaton
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// // For now, it's always safe to unwrap `in_and_fin`
    /// let machine_ref_a: AutomataRef = machine.expr_0().unwrap();
    /// machine.in_and_fin(&machine_ref_a).unwrap();
    /// ```
    pub fn in_and_fin(&mut self, machine_ref_a: &AutomataRef) -> Result<(), &'static str> {
        self.q0 = Some(machine_ref_a.q0);
        self.f = Some(machine_ref_a.f);

        Ok(())
    }

    /// Returns reference to an acceptor that never transitions to a final state, i.e. accept nothing
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let machine_ref_a: AutomataRef = machine.expr_0().unwrap();
    /// machine.in_and_fin(&machine_ref_a).unwrap();
    /// ```
    ///
    /// # Definition
    ///
    /// ```text
    /// (
    ///   state: { 0, 1 },
    ///   delta:
    ///     0
    ///     1,
    ///   q0: 0,
    ///   f: 1
    /// )
    /// ```
    ///
    ///```text
    /// machine_a
    /// ( 0 )  (( 1 ))
    /// ```
    pub fn expr_0(&mut self) -> Result<AutomataRef, &'static str> {
        let q0 = self.delta.len();
        self.delta.push(vec![]);

        let f = self.delta.len();
        self.delta.push(vec![]);

        Ok(AutomataRef { q0, f })
    }

    /// Returns reference to an acceptor in final state, i.e. accept anything, AKA epsilon acceptor
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let machine_ref_a: AutomataRef = machine.expr_1().unwrap();
    /// machine.in_and_fin(&machine_ref_a).unwrap();
    /// ```
    ///
    /// # Definition
    ///
    /// ```text
    /// (
    ///   state: { 0 },
    ///   delta:
    ///     0,
    ///   q0: 0,
    ///   f: 0
    /// )
    /// ```
    ///
    ///```text
    /// machine_a
    /// (( 0 ))
    /// ```
    pub fn expr_1(&mut self) -> Result<AutomataRef, &'static str> {
        let q0: usize = self.delta.len();
        self.delta.push(vec![]);

        Ok(AutomataRef { q0, f: q0 })
    }

    /// Returns reference to an automaton accepting the provided any in Σ
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let machine_ref_a: AutomataRef = machine.expr_a('a').unwrap();
    /// machine.in_and_fin(&machine_ref_a).unwrap();
    /// ```
    ///
    /// # Definition
    ///
    /// ```text
    /// (
    ///   state: { 0, 1 },
    ///   delta: 0 × 'a' × 1
    ///   q0: 0,
    ///   f: 1
    /// )
    /// ```
    ///
    ///```text
    /// machine_a
    /// ( 0 ) -- 'a' --> (( 1 ))
    /// ```
    pub fn expr_a(&mut self, c: char) -> Result<AutomataRef, &'static str> {
        let q0 = self.delta.len();
        let f = q0 + 1;
        let delta_q0: DeltaQ = vec![(Some(c), f)];
        self.delta.push(delta_q0);
        let delta_qf: DeltaQ = vec![];
        self.delta.push(delta_qf);

        Ok(AutomataRef { q0, f })
    }

    /// Concatenates machine references a and b of the same stack
    ///
    /// ```text
    /// machine_c = machine_a ⋅ machine_b
    /// ```
    /// 
    /// Concatenation is an associative, binary operation:
    ///
    /// ```text
    /// machine_n = machine_a ⋅ machine_b ⋅ machine_c
    /// machine_n = (machine_a ⋅ machine_b) ⋅ machine_c
    /// machine_n = machine_a ⋅ (machine_b ⋅ machine_c)
    /// ```
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// let machine_ref_a: AutomataRef = machine.expr_a('a').unwrap();
    /// let machine_ref_b: AutomataRef = machine.expr_a('b').unwrap();
    /// let machine_ref_c_result: Result<AutomataRef, &'static str> = machine.concatenate(&machine_ref_a, &machine_ref_b);
    /// match machine_ref_c_result {
    ///     Ok(machine_ref_c) => {
    ///         println!("{:#?}", machine.in_and_fin(&machine_ref_c).unwrap());
    ///     },
    ///     Err(err) => {
    ///         println!("Error creating automaton: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// # Definition
    ///
    /// ```text
    /// (
    ///     state: { 0, 1, 2, 3 },
    ///     delta:
    ///         0 × a × 1
    ///         1 × ε × 2
    ///         2 × b × 3
    ///         3
    ///   q0: 0,
    ///   f: 1
    /// )
    /// ```
    ///
    ///```text
    /// machine_a
    /// ( 0 ) --> 'a' --> (( 1 ))
    ///
    /// machine_b
    /// ( 2 ) --> 'b' --> (( 3 ))
    ///
    /// machine_c
    /// ( 0 ) -- 'a' --> ( 1 ) -- ε --> ( 2 ) -- 'b' --> (( 3 ))
    /// ```
    pub fn concatenate(
        &mut self,
        machine_ref_a: &AutomataRef,
        machine_ref_b: &AutomataRef,
    ) -> Result<AutomataRef, &'static str> {
        match [
            self.delta[machine_ref_a.f].len(),
            self.delta[machine_ref_b.f].len(),
        ] {
            [0, 0] => {}
            _ => {
                return Err(
                    "Final states of machine_ref_a and machine_ref_b can NOT have transitions",
                )
            }
        }

        self.delta[machine_ref_a.f].push((None, *&machine_ref_b.q0));

        Ok(AutomataRef {
            q0: machine_ref_a.q0,
            f: machine_ref_b.f,
        })
    }

    /// Pushes new states and mutates machine_ref_a states so that machine_ref_a is accepted 0 or more times
    ///
    /// Star is a unary operation:
    ///
    /// ```text
    /// machine_b = machine_a*
    /// ```
    ///
    /// # Examples
    ///
    /// Example 1
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// let machine_ref_a: AutomataRef = machine.expr_a('a').unwrap();
    /// let machine_ref_b_result: Result<AutomataRef, &'static str> = machine.star(&machine_ref_a);
    /// match machine_ref_b_result {
    ///     Ok(machine_ref_b) => {
    ///         println!("{:#?}", machine.in_and_fin(&machine_ref_b).unwrap());
    ///     },
    ///     Err(err) => {
    ///         println!("Error creating automaton: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// # Definition
    ///
    /// ```text
    /// (
    ///     state: { 0..4 },
    ///     delta:
    ///         0 × a × 1
    ///         1 × ε × 3
    ///         2 × ε × 3
    ///         3 × 0 × 0
    ///         3 × 1 × 4
    ///         4
    ///     q0: 2,
    ///     f: 4,
    /// )
    /// ```
    ///
    /// ```text
    /// machine_a
    /// ( 0 ) -- 'a' --> (( 1 ))
    ///
    /// machine_b = machine_a*
    ///                     /-- 0 --> ( 0 ) -- 'a' --> ( 1 )
    /// ( 2 ) -- ε --> ( 3 ) <------------ ε ------------|
    ///                     \-- 1 --> (( 4 ))
    /// ```
    pub fn star(&mut self, machine_ref_a: &AutomataRef) -> Result<AutomataRef, &'static str> {
        match self.delta[machine_ref_a.f].len() {
            0 => {}
            _ => return Err("Final state of machine_ref_a can NOT have transitions"),
        };

        let q0 = self.delta.len();
        self.delta.push(vec![]);

        let q_next = q0 + 1;
        self.delta.push(vec![]);

        let f = q_next + 1;
        self.delta.push(vec![]);

        self.delta[q0].push((None, *&q_next));
        self.delta[q_next].push((None, *&machine_ref_a.q0));
        self.delta[q_next].push((None, *&f));
        self.delta[machine_ref_a.f].push((None, *&q_next));

        Ok(AutomataRef { q0, f })
    }

    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// let machine_ref_a: AutomataRef = machine.expr_a('a').unwrap();
    /// let machine_ref_b: AutomataRef = machine.expr_a('b').unwrap();
    /// let machine_ref_c_result: Result<AutomataRef, &'static str> = machine.union(&machine_ref_a, &machine_ref_b);
    /// match machine_ref_c_result {
    ///     Ok(machine_ref_c) => {
    ///         println!("{:#?}", machine.in_and_fin(&machine_ref_c).unwrap());
    ///     },
    ///     Err(err) => {
    ///         println!("Error creating automaton: {}", err);
    ///     }
    /// };
    /// ```
    ///
    ///  # Definition
    ///
    /// ```text
    /// (
    ///     state: { 0..5 },
    ///     delta:
    ///         0 × a × 1
    ///         1 × ε × 5
    ///         2 × b × 3
    ///         3 × ε × 5
    ///         4 × 0 × 0
    ///         4 × 1 × 2
    ///         5
    ///     q0: 4,
    ///     f: 5,
    /// )
    /// ```
    ///
    /// ```text
    /// machine_a
    /// ( 0 ) -- 'a' --> (( 1 ))
    ///
    /// machine_b
    /// ( 2 ) -- 'b' --> (( 3 ))
    ///
    /// machine_c = machina_a ∪ machine_b
    ///     / -- 0 --> ( 0 ) -- 'a' --> ( 1 ) --\
    /// ( 4 )                                    ε --> (( 5 ))
    ///     \ -- 1 --> ( 2 ) -- 'b' --> ( 3 ) --/
    /// ```
    pub fn union(
        &mut self,
        machine_ref_a: &AutomataRef,
        machine_ref_b: &AutomataRef,
    ) -> Result<AutomataRef, &'static str> {
        match [
            self.delta[machine_ref_a.f].len(),
            self.delta[machine_ref_b.f].len(),
        ] {
            [0, 0] => {}
            _ => {
                return Err(
                    "Final states of machine_ref_a and machine_ref_b can NOT have transitions",
                )
            }
        }

        let q0 = self.delta.len();
        let delta_q0: DeltaQ = vec![(None, machine_ref_a.q0), (None, machine_ref_b.q0)];
        self.delta.push(delta_q0);

        let f = q0 + 1;
        self.delta.push(vec![]);

        self.delta[machine_ref_a.f].push((None, *&f));
        self.delta[machine_ref_b.f].push((None, *&f));

        Ok(AutomataRef { q0, f })
    }
}

impl std::fmt::Display for ANFA {
    /// Formats an ANFA implmentation as a formal definition
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use regexxx::anfa::{ ANFA, AutomataRef };
    ///
    /// let mut machine = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let machine_ref_a: AutomataRef = machine.expr_0().unwrap();
    /// machine.in_and_fin(&machine_ref_a).unwrap();
    /// println!("{}", machine);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let delta_table = self
            .delta
            .iter()
            .enumerate()
            .map(|(index, delta_q)| {
                vec![
                    "\t",
                    &index.to_string(),
                    &(match delta_q.len() {
                        0 => String::from(""),
                        1 => vec![
                            " × ",
                            &(match delta_q[0].0 {
                                Some(transition) => transition.to_string(),
                                None => String::from("ε"),
                            }),
                            " × ",
                            &delta_q[0].1.to_string(),
                        ]
                        .join(""),
                        2 => vec![
                            " × 0 × ",
                            &delta_q[0].1.to_string(),
                            "\n\t",
                            &index.to_string(),
                            " × 1 × ",
                            &delta_q[1].1.to_string(),
                        ]
                        .join(""),
                        _ => String::from("PANIC!"),
                    }),
                ]
                .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "ANFA (
    state: {{ 0..{} }},
    delta:
{}
    q0: {},
    f: {},
)",
            self.delta.len() - 1,
            delta_table,
            match self.q0 {
                Some(c) => c.to_string(),
                None => String::from("_"),
            },
            match self.f {
                Some(c) => c.to_string(),
                None => String::from("_"),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::anfa::{AutomataRef, ANFA};

    #[test]
    fn test_new() {
        assert_eq!(
            ANFA::new(),
            ANFA {
                delta: vec![],
                q0: None,
                f: None
            },
            "A new stack must be empty"
        );
    }

    #[test]
    fn test_in_and_fin() {
        let mut machine = ANFA::new();
        let machine_ref_a = machine.expr_0().unwrap();
        machine.in_and_fin(&machine_ref_a).unwrap();
        match [machine.q0, machine.f] {
            [Some(machine_q0), Some(machine_f)] => {
                assert_eq!(
                    machine_ref_a,
                    AutomataRef {
                        q0: machine_q0,
                        f: machine_f,
                    },
                    "Machine did not use q0 and f from machine_ref_a"
                );
            }
            [None, _] => {
                assert!(false, "Machine is missing initial state");
            }
            [_, None] => {
                assert!(false, "Machine is missing final state");
            }
        }
    }

    #[test]
    fn test_expr_0() {
        let mut machine = ANFA::new();
        let machine_ref_a = machine.expr_0().unwrap();
        machine.in_and_fin(&machine_ref_a).unwrap();

        println!("{:#?}", machine_ref_a);
        println!("{:#?}", machine);
        println!("{}", machine);
        assert!(true, "Can't format expr_0");
        assert_eq!(
            machine.delta.len(),
            2,
            "Expression 0 (nothing) pushes two states"
        );
        assert_eq!(
            machine.delta[0].len(),
            0,
            "Expression 0 (nothing) cannot transition from q0"
        );
        assert_eq!(
            machine.delta[1].len(),
            0,
            "Expression 0 (nothing) cannot transition from f"
        );
        assert_ne!(
            machine_ref_a.q0, machine_ref_a.f,
            "Expression 0 (nothing) starts and ends on different states"
        );
    }
    #[test]
    fn test_expr_1() {
        let mut machine = ANFA::new();
        let machine_ref_a = machine.expr_1().unwrap();
        machine.in_and_fin(&machine_ref_a).unwrap();

        println!("{:#?}", machine_ref_a);
        println!("{:#?}", machine);
        println!("{}", machine);
        assert!(true, "Can't format expr_1");
        assert_eq!(
            machine.delta.len(),
            1,
            "Expression 1 (epsilon) pushes one state"
        );
        assert_eq!(
            machine.delta[0].len(),
            0,
            "Expression 1 (epsilon) does not transition from q0"
        );
        assert_eq!(
            machine_ref_a.q0, machine_ref_a.f,
            "Expression 1 (epsilon) starts and ends on the same state"
        );
    }

    #[test]
    fn test_expr_a() {
        let mut machine = ANFA::new();
        let machine_ref_a = machine.expr_a('a').unwrap();
        machine.in_and_fin(&machine_ref_a).unwrap();

        println!("test_expr_a formatting");
        println!("{:#?}", machine_ref_a);
        println!("{:#?}", machine);
        println!("{}", machine);
        assert!(true, "Can't format expr_a");
        assert_eq!(
            machine.delta.len(),
            2,
            "Expression 'a' (literal) pushes two states"
        );
        assert_eq!(
            machine.delta[0].len(),
            1,
            "Expression 'a' (literal) has one transition from q0"
        );
        assert_eq!(
            machine.delta[0][0],
            (Some('a'), 1),
            "Expression 'a' (literal) transitions from q0 to f along 'a'"
        );
        assert_eq!(
            machine.delta[1].len(),
            0,
            "Expression 'a' (literal) cannot transition from f"
        );
        assert_ne!(
            machine_ref_a.q0, machine_ref_a.f,
            "Expression 'a' (literal) starts and ends on different states"
        );
    }

    #[test]
    fn test_concatenate() {
        let mut machine_a = ANFA::new();
        let machine_ref_a = machine_a.expr_a('a').unwrap();
        let ref_a_states_pushed = machine_a.delta.len();
        let machine_ref_b = machine_a.expr_a('b').unwrap();
        let ref_b_states_pushed = machine_a.delta.len() - ref_a_states_pushed;
        let machine_ref_c = machine_a
            .concatenate(&machine_ref_a, &machine_ref_b)
            .unwrap();
        machine_a.in_and_fin(&machine_ref_c).unwrap();
        let ref_c_states_length = *&machine_a.delta.len();

        assert_eq!(
            ref_a_states_pushed + ref_b_states_pushed,
            ref_c_states_length,
            "Concatenation created new states, | machine_c Q | = | machine_a Q | + | machine_b Q |"
        );

        assert_eq!(
            machine_a.q0,
            Some(machine_ref_a.q0),
            "q0 was not from left-side of concatenation operation"
        );

        assert_eq!(
            machine_a.delta[machine_ref_a.f].len(),
            1,
            "machine_ref_a.f was a union"
        );

        assert_eq!(
            machine_a.delta[machine_ref_a.f],
            vec![(None, machine_ref_b.q0)],
            "machine_ref_a.f did not epsilon transition to machine_ref_b.q0"
        );

        assert_eq!(
            machine_a.f,
            Some(machine_ref_b.f),
            "machine did not finish at machine_ref_b.f"
        );

        // testing associativity, kind of verbose
        // machine_d = (machine_a ⋅ machine_b) ⋅ machine_c
        let mut machine_b = ANFA::new();
        let machine_b_ref_a = machine_b.expr_a('a').unwrap();
        let machine_b_ref_b = machine_b.expr_a('b').unwrap();
        let machine_b_ref_c = machine_b.expr_a('c').unwrap();
        let machine_b_ref_ab = machine_b
            .concatenate(&machine_b_ref_a, &machine_b_ref_b)
            .unwrap();
        let machine_b_ref_abc = machine_b
            .concatenate(&machine_b_ref_ab, &machine_b_ref_c)
            .unwrap();
        machine_b.in_and_fin(&machine_b_ref_abc).unwrap();
        // machine_d = machine_a ⋅ (machine_b ⋅ machine_c)
        let mut machine_c = ANFA::new();
        let machine_c_ref_a = machine_c.expr_a('a').unwrap();
        let machine_c_ref_b = machine_c.expr_a('b').unwrap();
        let machine_c_ref_c = machine_c.expr_a('c').unwrap();
        let machine_c_ref_bc = machine_c
            .concatenate(&machine_c_ref_b, &machine_c_ref_c)
            .unwrap();
        let machine_c_ref_abc = machine_c
            .concatenate(&machine_c_ref_a, &machine_c_ref_bc)
            .unwrap();
        machine_c.in_and_fin(&machine_c_ref_abc).unwrap();
        assert_eq!(machine_b, machine_c, "Concatenation was not associative");

        println!("{:#?}", machine_ref_c);
        println!("{:#?}", machine_a);
        println!("{}", machine_a);
        assert!(true, "Can't format concatenate");
    }

    #[test]
    fn test_star() {
        let mut machine = ANFA::new();
        let machine_ref_a = machine.expr_a('a').unwrap();
        let machine_ref_b = machine.star(&machine_ref_a).unwrap();
        machine.in_and_fin(&machine_ref_b).unwrap();

        match [machine.q0, machine.f] {
            [Some(machine_q0), Some(machine_f)] => {
                assert_eq!(
                    machine.delta[machine_q0].len(),
                    1,
                    "Initial state was a union"
                );
                assert_eq!(
                    machine.delta[machine_q0][0].0, None,
                    "Initial state did not have epsilon transition"
                );
                // assert_eq!(machine.delta[machine_ref_a.f]);
                let union_state_id = machine.delta[machine_q0][0].1;
                let union_ref = &machine.delta[union_state_id];
                assert_eq!(
                    union_ref[0],
                    (None, machine_ref_a.q0),
                    "After epsilon transition from q0, machine did not go left to machine_ref_a.q0"
                );
                assert_eq!(
                    union_ref[1],
                    (None, machine_f),
                    "After epsilon transition from q0, machine did not go right to final state"
                );
                assert_eq!(
                    machine.delta[machine_ref_a.f][0].1, union_state_id,
                    "Final state of machine_ref_a must transition back to union"
                )
            }
            [_, _] => {}
        };

        println!("{:#?}", machine_ref_b);
        println!("{:#?}", machine);
        println!("{}", machine);
        assert!(true, "Can't format star");
    }

    #[test]
    fn test_union() {
        let mut machine = ANFA::new();
        let machine_ref_a = machine.expr_a('a').unwrap();
        let machine_ref_b = machine.expr_a('b').unwrap();
        let machine_ref_c = machine.union(&machine_ref_a, &machine_ref_b).unwrap();
        machine.in_and_fin(&machine_ref_c).unwrap();

        match machine.q0 {
            Some(machine_q0) => {
                assert_eq!(
                    machine.delta[machine_q0],
                    vec![(None, machine_ref_a.q0), (None, machine_ref_b.q0)],
                    "Initial state was not a union of machine_a and machine_b initial states"
                );
            }
            _ => {
                assert!(false, "Machine did not have initial state");
            }
        };

        match machine.f {
            Some(machine_f) => {
                assert_eq!(
                    machine.delta[machine_ref_a.f],
                    vec![(None, machine_f)],
                    "Final state of machine_ref_a must have epsilon transition to final state of machine"
                );
                assert_eq!(
                    machine.delta[machine_ref_b.f],
                    vec![(None, machine_f)],
                    "Final state of machine_ref_b must have epsilon transition to final state of machine"
                );
            }
            _ => {
                assert!(false, "Machine did not have final state");
            }
        };

        println!("{:#?}", machine_ref_c);
        println!("{:#?}", machine);
        println!("{}", machine);
        assert!(true, "Can't format union");
    }

    #[test]
    fn test_impl_fmt() {
        // RE (a + b)*b
        let mut machine = ANFA::new();
        let machine_ref_a = machine.expr_a('a').unwrap();
        let machine_ref_b = machine.expr_a('b').unwrap();
        let machine_ref_c = machine.union(&machine_ref_a, &machine_ref_b).unwrap();
        let machine_ref_d = machine.star(&machine_ref_c).unwrap();
        let machine_ref_e = machine.expr_a('b').unwrap();
        let machine_ref_f = machine.concatenate(&machine_ref_d, &machine_ref_e).unwrap();
        println!("{:#?}", machine_ref_f);
        assert!(true, "Can't debug AutomataRef");

        machine.in_and_fin(&machine_ref_f).unwrap();
        println!("{:#?}", machine);
        assert!(true, "Can't debug ANFA");

        println!("{}", machine);
        assert!(true, "Can't display ANFA");
    }
}
