// for ease of copy+paste:
// ε
// FA = (Q, Σ, δ, q0, F)

use std::convert::From;

/// Unique state id
pub type QId = usize;

/// Reference to automata initial and final states
#[derive(Clone, Debug, PartialEq)]
pub struct AutomataRef {
    /// initial state
    pub q0: QId,
    /// final state
    pub f: QId,
}

/// Transition is 2nd and 3rd arguments `T` and `State` in `δ ⊆ State × T × State`
pub type Transition = (Option<char>, QId);

/// 1 or 2 results of δ, optionally labelled. When length is 2, operation is union.
pub type DeltaQ = Vec<Transition>;

/// Index of Delta is the first argument `State` in `δ ⊆ State × T × State`
pub type Delta = Vec<DeltaQ>;

/// Augmented non-deterministic finite automaton
///
/// Augmented non-deterministic finite automata are defined by the 4-tuple:
/// - state,  finite set of states
/// - delta, `δ ⊆ State × T × State` is a labeled transition relation with labels `T = Σ ⊎ {0, 1, ε}`
/// - q0, initial state
/// - f, final state
#[derive(Clone, Debug, PartialEq)]
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
    /// use automata::{ ANFA, AutomataRef };
    ///
    /// let mut anfa = ANFA::new();
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
    /// use automata::{ ANFA, AutomataRef };
    ///
    /// let mut anfa = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let ref_0: AutomataRef = anfa.expr_0().unwrap();
    /// anfa.in_and_fin(&ref_0).unwrap();
    /// ```
    pub fn in_and_fin(&mut self, machine_ref_a: &AutomataRef) -> Result<(), &'static str> {
        self.q0 = Some(machine_ref_a.q0);
        self.f = Some(machine_ref_a.f);

        Ok(())
    }

    /// finite set of states
    pub fn state(self) -> std::ops::Range<QId> {
        std::ops::Range {
            start: 0,
            end: self.delta.len(),
        }
    }

    /// Returns reference to an acceptor that never transitions to a final state, i.e. accept nothing
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use automata::{ ANFA, AutomataRef };
    ///
    /// let mut anfa = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let ref_0: AutomataRef = anfa.expr_0().unwrap();
    /// anfa.in_and_fin(&ref_0).unwrap();
    /// ```
    ///
    /// # Implementation
    ///
    /// AutomataRef
    ///
    /// ```text
    /// AutomataRef {
    ///     q0: 0,
    ///     f: 1,
    /// }
    /// ```
    ///
    /// Into ANFA
    ///
    /// ```text
    /// {
    ///     delta: [
    ///         [],
    ///         []
    ///     ],
    ///     q0: 0,
    ///     f: 1,
    /// }
    /// ```
    ///
    /// # Definition
    ///
    /// ```text
    /// (
    ///   Q: { 0, 1 },
    ///   δ: none,
    ///   q0: 0,
    ///   F: 1
    /// )
    /// ```
    ///
    ///```text
    /// ( 0 )  (( 1 ))
    /// ```
    ///
    pub fn expr_0(&mut self) -> Result<AutomataRef, &'static str> {
        let q0 = self.delta.len();
        self.delta.push(vec![]);

        let f = self.delta.len();
        self.delta.push(vec![]);

        Ok(AutomataRef { q0, f })
    }

    ///
    /// Returns reference to an acceptor in final state, i.e. accept anything, AKA epsilon acceptor
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use automata::{ ANFA, AutomataRef };
    ///
    /// let mut anfa = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let ref_0: AutomataRef = anfa.expr_1().unwrap();
    /// anfa.in_and_fin(&ref_0).unwrap();
    /// ```
    ///
    /// # Implementation
    ///
    /// AutomataRef
    ///
    /// ```text
    /// AutomataRef {
    ///     q0: 0,
    ///     f: 0,
    /// }
    /// ```
    ///
    /// Into ANFA
    ///
    /// ```text
    /// ANFA {
    ///     delta: [
    ///         []
    ///     ],
    ///     q0: 0,
    ///     f: 0
    /// }
    /// ```
    ///
    /// # Definition
    ///
    /// ```text
    /// (
    ///   Q: { 0 },
    ///   δ: none,
    ///   q0: 0,
    ///   F: 0
    /// )
    /// ```
    ///
    ///```text
    /// (( 0 ))
    /// ```
    ///
    pub fn expr_1(&mut self) -> Result<AutomataRef, &'static str> {
        let q0: usize = self.delta.len();
        self.delta.push(vec![]);

        Ok(AutomataRef { q0, f: q0 })
    }

    ///
    /// Returns reference to an automaton accepting the provided any in Σ
    ///
    /// # Examples
    ///
    /// Example 1:
    ///
    /// ```rust
    /// use automata::{ ANFA, AutomataRef };
    ///
    /// let mut anfa = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let ref_0: AutomataRef = anfa.expr_a('a').unwrap();
    /// anfa.in_and_fin(&ref_0).unwrap();
    /// ```
    ///
    /// # Implementation
    ///
    /// AutomataRef
    ///
    /// ```text
    /// AutomataRef {
    ///     q0: 0,
    ///     f: 1
    /// }
    /// ```
    ///
    /// Into ANFA
    ///
    /// ```text
    /// ANFA {
    ///     delta: [
    ///         [(Some('a'), 1)],
    ///         []
    ///     ],
    ///     q0: 0,
    ///     f: 1
    /// }
    /// ```
    ///
    /// # Definition
    ///
    /// ```text
    /// (
    ///   Q: { 0, 1 },
    ///   δ: (0, 'a') => 1,
    ///   q0: 0,
    ///   F: 1
    /// )
    /// ```
    ///
    ///```text
    /// machine a
    /// ( 0 ) -- 'a' --> (( 1 ))
    /// ```
    ///
    pub fn expr_a(&mut self, c: char) -> Result<AutomataRef, &'static str> {
        let q0 = self.delta.len();
        let f = q0 + 1;
        let delta_q0: DeltaQ = vec![(Some(c), f)];
        self.delta.push(delta_q0);
        let delta_qf: DeltaQ = vec![];
        self.delta.push(delta_qf);

        Ok(AutomataRef { q0, f })
    }

    /// Concatenates machine slices a and b of the same stack
    ///
    /// Concatenation is an associative, binary operation:
    ///
    /// ```text
    /// machine_c = machine_a ⋅ machine_b
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
    /// use automata::{ ANFA, AutomataRef };
    ///
    /// let mut anfa = ANFA::new();
    /// let slice_a: AutomataRef = anfa.expr_a('a').unwrap();
    /// let slice_b: AutomataRef = anfa.expr_a('b').unwrap();
    /// let slice_c_result: Result<AutomataRef, &'static str> = anfa.concatenate(&slice_a, &slice_b);
    /// match slice_c_result {
    ///     Ok(slice_c) => {
    ///         println!("{:#?}", anfa.in_and_fin(&slice_c).unwrap());
    ///     },
    ///     Err(err) => {
    ///         println!("Error creating automaton: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// # Diagram
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
    ///
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
            _ => return Err("Final states of machine_a and machine_b can NOT have transitions"),
        }

        self.delta[machine_ref_a.f].push((None, *&machine_ref_b.q0));

        Ok(AutomataRef {
            q0: machine_ref_a.q0,
            f: machine_ref_b.f,
        })
    }

    ///
    /// From slice a, pushes the star operation -- a* -- to the stack, returning slice b
    ///
    /// Star is a unary operation:
    ///
    /// ```text
    /// machine_b = machine_a*
    /// ```
    ///
    /// # Examples
    ///
    /// # Diagram
    ///
    /// ```text
    /// machine_a
    /// ( 0 ) -- 'a' --> (( 1 ))
    ///
    /// machine b = machine_a*
    ///                     /-- 0 --> ( 0 ) -- 'a' --> ( 1 )
    /// ( 2 ) -- ε --> ( 3 ) <------------ ε ------------|
    ///                     \-- 1 --> (( 4 ))
    /// ```
    ///
    pub fn star(&mut self, machine_ref_a: &AutomataRef) -> Result<AutomataRef, &'static str> {
        match self.delta[machine_ref_a.f].len() {
            0 => {}
            _ => return Err("Final state of machine_a can NOT have transitions"),
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

    /// # Diagram
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
    ///     \ -- 1 --> ( 2 ) -- 'b' --> ( 2 ) --/
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
            _ => return Err("Final states of machine_a and machine_b can NOT have transitions"),
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
    /// use automata::{ ANFA, AutomataRef };
    ///
    /// let mut anfa = ANFA::new();
    /// // it's always safe to unwrap acceptor automata
    /// let ref_0: AutomataRef = anfa.expr_0().unwrap();
    /// anfa.in_and_fin(&ref_0).unwrap();
    /// println!("{}", anfa);
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
            "ANFA {{
    Q: {{ 0..{} }},
    δ: [
{}
    ],
    q0: {},
    f: {},
}}",
            self.delta.len(),
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
    use crate::ANFA;

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
    fn test_fmt() {
        // RE (a ∪ b)*b
        let mut anfa = ANFA::new();

        let mut ref_a = anfa.expr_a('a').unwrap();
        let ref_b = anfa.expr_a('b').unwrap();
        let ref_c = anfa.union(&mut ref_a, &ref_b).unwrap();
        let ref_d = anfa.star(&ref_c).unwrap();
        let ref_e = anfa.expr_a('b').unwrap();
        let ref_f = anfa.concatenate(&ref_d, &ref_e).unwrap();
        println!("{:#?}", ref_f);
        assert!(true, "Can't debug AutomataRef");

        anfa.in_and_fin(&ref_f).unwrap();
        println!("{:#?}", anfa);
        assert!(true, "Can't debug ANFA");

        println!("{}", anfa);
        assert!(true, "Can't display ANFA");
    }

    #[test]
    fn test_expr_0() {
        let mut anfa = ANFA::new();
        let slice_a = anfa.expr_0().unwrap();

        assert_eq!(
            anfa.delta.len(),
            2,
            "Expression 0 (nothing) pushes two states"
        );
        assert_eq!(
            anfa.delta[0].len(),
            0,
            "Expression 0 (nothing) cannot transition from q0"
        );
        assert_eq!(
            anfa.delta[1].len(),
            0,
            "Expression 0 (nothing) cannot transition from f"
        );
        assert_ne!(
            slice_a.q0, slice_a.f,
            "Expression 0 (nothing) starts and ends on different states"
        );
    }
    #[test]
    fn test_expr_1() {
        let mut anfa = ANFA::new();
        let slice_a = anfa.expr_1().unwrap();

        assert_eq!(
            anfa.delta.len(),
            1,
            "Expression 1 (epsilon) pushes one state"
        );
        assert_eq!(
            anfa.delta[0].len(),
            0,
            "Expression 1 (epsilon) does not transition from q0"
        );
        assert_eq!(
            slice_a.q0, slice_a.f,
            "Expression 1 (epsilon) starts and ends on the same state"
        );
    }

    #[test]
    fn test_expr_a() {
        let mut anfa = ANFA::new();
        let slice_a = anfa.expr_a('a').unwrap();

        assert_eq!(
            anfa.delta.len(),
            2,
            "Expression 'a' (literal) pushes two states"
        );
        assert_eq!(
            anfa.delta[0].len(),
            1,
            "Expression 'a' (literal) has one transition from q0"
        );
        assert_eq!(
            anfa.delta[0][0],
            (Some('a'), 1),
            "Expression 'a' (literal) transitions from q0 to f along 'a'"
        );
        assert_eq!(
            anfa.delta[1].len(),
            0,
            "Expression 'a' (literal) cannot transition from f"
        );
        assert_ne!(
            slice_a.q0, slice_a.f,
            "Expression 'a' (literal) starts and ends on different states"
        );
    }

    // #[test]
    // fn test_concatenate() {}

    // #[test]
    // fn test_star() {}

    // #[test]
    // fn test_union() {}
}
