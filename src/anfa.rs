use alloc::vec;

/// Unique state id
/// TODO: Compile a regex large enough to overflow
pub type QId = usize;

/// An optional transition along an optional label to another State.
/// When a Transition is `None`, the current state does not transition, i.e. it is a dead/trap state
/// When a label is `None`, transition is an epsilon transition and always advances to the final state.
pub type Transition = Option<(Option<char>, QId)>;

/// DeltaFunction is a vector of transition slices that satisfy
/// the function `δ ⊆ State × T × State`. When second Transition
/// is `Some(_)`, transition is a union
pub type DeltaFunction = alloc::vec::Vec<[Transition; 2]>;

/// The initial and final states of an expression: (q0, f)
pub type AutomataRef = (QId, QId);

#[derive(Debug)]
pub struct ANFA {
    automata_refs: vec::Vec<AutomataRef>,
    delta: DeltaFunction,
}

impl ANFA {
    /// The ANFA constructor does not return a valid automaton.
    /// ANFA must be constructed with a static acceptor factory:
    /// `from_expr_0`, `from_expr_1`, or `from_expr_a`
    fn new() -> ANFA {
        ANFA {
            automata_refs: vec![],
            delta: vec![],
        }
    }

    /// Returns a new ANFA that never transitions to a final state. See [ANFA::expr_0]
    ///
    /// ```rust
    /// use regexxx::anfa::ANFA;
    /// let machine = ANFA::from_expr_0().unwrap(); // always safe!
    /// ```
    pub fn from_expr_0() -> Result<ANFA, &'static str> {
        let mut machine_a = ANFA::new();
        match machine_a.expr_0() {
            Ok(()) => Ok(machine_a),
            Err(e) => Err(e),
        }
    }

    /// Returns a new ANFA in its final state. See [ANFA::expr_1]
    ///
    /// ```rust
    /// use regexxx::anfa::ANFA;
    /// let machine = ANFA::from_expr_1().unwrap(); // always safe!
    /// ```
    pub fn from_expr_1() -> Result<ANFA, &'static str> {
        let mut machine_a = ANFA::new();
        match machine_a.expr_1() {
            Ok(()) => Ok(machine_a),
            Err(e) => Err(e),
        }
    }

    /// Returns a new ANFA that transitions to a final state on 'a'. See [ANFA::expr_a]
    ///
    /// ```rust
    /// use regexxx::anfa::ANFA;
    /// let mut machine = ANFA::from_expr_a('a').unwrap(); // always safe!
    /// ```
    pub fn from_expr_a(c: char) -> Result<ANFA, &'static str> {
        let mut machine_a = ANFA::new();
        match machine_a.expr_a(c) {
            Ok(()) => Ok(machine_a),
            Err(e) => Err(e),
        }
    }
    /// Pushes an acceptor that never transitions, i.e. accept nothing
    ///
    /// ```rust
    /// use regexxx::anfa::ANFA;
    /// let mut machine = ANFA::from_expr_a('a').unwrap(); // always safe!
    /// match machine.expr_0() {
    ///     Ok(()) => {}
    ///     Err(err) => {
    ///       println!("expr_0 error: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// ```text
    /// Definition of `0`
    ///
    /// State table:
    /// | Q | T | Q |
    /// |---|---|---|
    /// | 0 |   |   | (q0)
    /// | 1 |   |   | (f)
    ///
    /// Graph:
    /// --> ( 0 )  (( 1 ))
    /// ```
    pub fn expr_0(&mut self) -> Result<(), &'static str> {
        let q0 = self.delta.len();
        let f = q0 + 1;
        let machine_a = (q0, f);
        self.delta.push([None, None]);
        self.delta.push([None, None]);
        self.automata_refs.push(machine_a);
        Ok(())
    }

    /// Pushes an acceptor in final state, i.e. accept anything, AKA epsilon acceptor
    ///
    /// ```rust
    /// use regexxx::anfa::ANFA;
    /// let mut machine = ANFA::from_expr_a('a').unwrap(); // always safe!
    /// match machine.expr_1() {
    ///     Ok(()) => {}
    ///     Err(err) => {
    ///       println!("expr_1 error: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// ```text
    /// Definition of `1`
    ///
    /// State table:
    /// | Q | T | Q |
    /// |---|---|---|
    /// | 0 |   |   | (q0 = f)
    ///
    /// Graph:
    /// --> (( 0 ))
    /// ```
    pub fn expr_1(&mut self) -> Result<(), &'static str> {
        let q0: usize = self.delta.len();
        let machine_a = (q0, q0);
        self.delta.push([None, None]);
        self.automata_refs.push(machine_a);
        Ok(())
    }

    /// Pushes an automaton that transitions to a final state on 'a'
    ///
    /// ```rust
    /// use regexxx::anfa::ANFA;
    /// let mut machine = ANFA::from_expr_a('a').unwrap(); // always safe!
    /// match machine.expr_a('b') {
    ///     Ok(()) => {}
    ///     Err(err) => {
    ///       println!("expr_a error: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// ```text
    /// Definition of `'a'`:
    ///
    /// State table:
    /// | Q | T | Q |
    /// |---|---|---|
    /// | 0 | a | 1 | (q0)
    /// | 1 |   |   | (f)
    ///
    /// Graph:
    /// --> ( 0 ) -- 'a' --> (( 1 ))
    /// ```
    pub fn expr_a(&mut self, c: char) -> Result<(), &'static str> {
        let q0 = self.delta.len();
        let f = q0 + 1;
        let machine_a = (q0, f);
        self.delta.push([Some((Some(c), f)), None]);
        self.delta.push([None, None]);
        self.automata_refs.push(machine_a);
        Ok(())
    }

    /// Concatenate machines 'a' and 'b'
    ///
    /// ```rust
    /// #![warn(unused)]
    /// use regexxx::anfa::ANFA;
    /// let mut machine = ANFA::from_expr_a('a').unwrap(); // always safe
    /// machine.expr_a('b').unwrap(); // (should be) safe
    /// match machine.concatenate() {
    ///     Ok(()) => {}
    ///     Err(err) => {
    ///         println!("Error concatenating 'a' and 'b'. Were there enough machines on the stack? Error: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// ```text
    /// Definition of `'a' ⋅ 'b'`
    ///
    /// State table:
    /// | Q | T | Q |
    /// |---|---|---|
    /// | 0 | a | 1 | (q0)
    /// | 1 | ε | 2 |
    /// | 2 | b | 3 |
    /// | 3 |   |   | (f)
    ///
    /// Graph:
    /// Expression 'a'
    /// --> ( 0 ) --> 'a' --> (( 1 ))
    ///
    /// Expression 'b'
    /// --> ( 2 ) --> 'b' --> (( 3 ))
    ///
    /// Expression 'a' ⋅ 'b'
    /// --> ( 0 ) -- 'a' --> ( 1 ) -- ε --> ( 2 ) -- 'b' --> (( 3 ))
    /// ```
    ///
    /// Concatenation is an associative, binary operation:
    /// ```text
    /// machine_n = machine_a ⋅ machine_b ⋅ machine_c
    /// machine_n = (machine_a ⋅ machine_b) ⋅ machine_c
    /// machine_n = machine_a ⋅ (machine_b ⋅ machine_c)
    /// ```
    pub fn concatenate(&mut self) -> Result<(), &'static str> {
        let (machine_b_q0, machine_b_f) = self.automata_refs.pop().unwrap();
        let (machine_a_q0, machine_a_f) = self.automata_refs.pop().unwrap();
        let machine_c = (machine_a_q0, machine_b_f);
        self.delta[machine_a_f] = [Some((None, machine_b_q0)), None]; // point 'a' at 'b'
        self.automata_refs.push(machine_c);
        Ok(())
    }

    /// Star is a unary operation so that the last machine may be repeated 0 or more times.
    ///
    /// ```rust
    /// use regexxx::anfa::ANFA;
    /// let mut machine = ANFA::from_expr_a('a').unwrap(); // always safe!
    /// match machine.star() {
    ///     Ok(()) => {}
    ///     Err(err) => {
    ///         println!("Error performing star operation on 'a'. Does 'a' exist? Error: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// ```text
    /// Definition of `'a' *`
    ///
    /// State table:
    /// | Q | T | Q |
    /// |---|---|---|
    /// | 0 | a | 1 |
    /// | 1 | ε | 3 |
    /// | 2 | ε | 3 | (q0)
    /// | 3 | ε | 0 |
    /// | 3 | ε | 4 |
    /// | 4 |   |   | (f)
    ///
    /// Graph:
    /// Expression 'a'
    /// --> ( 0 ) -- 'a' --> (( 1 ))
    ///
    /// Expression 'a'*
    ///                         /-- 0 --> ( 0 ) -- 'a' --> ( 1 )
    /// --> ( 2 ) -- ε --> ( 3 ) <------------ ε ------------|
    ///                         \-- 1 --> (( 4 ))
    /// ```
    pub fn star(&mut self) -> Result<(), &'static str> {
        let q0 = self.delta.len();
        let q_next = q0 + 1;
        let f = q_next + 1;
        let machine_b = (q0, f);
        let (machine_a_q0, machine_a_f) = self.automata_refs.pop().unwrap();
        self.delta.push([Some((None, q_next)), None]);
        self.delta
            .push([Some((None, machine_a_q0)), Some((None, f))]);
        self.delta.push([None, None]);
        self.delta[machine_a_f] = [Some((None, q_next)), None];
        self.automata_refs.push(machine_b);
        Ok(())
    }

    /// ```rust
    /// use regexxx::anfa::ANFA;
    /// let mut machine = ANFA::from_expr_a('a').unwrap(); // always safe!
    /// machine.expr_a('b').unwrap(); // (should be) always safe!
    /// match machine.union() {
    ///     Ok(()) => {}
    ///     Err(err) => {
    ///         println!("Error peforming union. Were there enough operands? See: {}", err);
    ///     }
    /// };
    /// ```
    ///
    /// ```text
    /// Definition of `'a' ∪ 'b'`
    ///
    /// State table:
    /// | Q | T | Q |
    /// |---|---|---|
    /// | 0 | a | 1 |
    /// | 1 | ε | 5 |
    /// | 2 | b | 3 |
    /// | 3 | ε | 5 |
    /// | 4 | ε | 0 | (q0)
    /// | 4 | ε | 2 | (q0)
    /// | 5 |   |   | (f)
    ///
    /// Graph:
    /// Expression 'a'
    /// ( 0 ) -- 'a' --> (( 1 ))
    ///
    /// Expression 'b'
    /// ( 2 ) -- 'b' --> (( 3 ))
    ///
    /// Expression 'a' ∪ 'b'
    ///     / -- 0 --> ( 0 ) -- 'a' --> ( 1 ) --\
    /// ( 4 )                                    ε --> (( 5 ))
    ///     \ -- 1 --> ( 2 ) -- 'b' --> ( 3 ) --/
    /// ```
    pub fn union(&mut self) -> Result<(), &'static str> {
        let q0 = self.delta.len();
        let f = q0 + 1;
        let machine_c = (q0, f);
        let (machine_b_q0, machine_b_f) = self.automata_refs.pop().unwrap();
        let (machine_a_q0, machine_a_f) = self.automata_refs.pop().unwrap();
        self.delta
            .push([Some((None, machine_a_q0)), Some((None, machine_b_q0))]);
        self.delta.push([None, None]);
        self.delta[machine_a_f] = [Some((None, f)), None];
        self.delta[machine_b_f] = [Some((None, f)), None];
        self.automata_refs.push(machine_c);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::anfa::ANFA;

    #[test]
    fn test_expr_0() {
        let mut machine = ANFA::new();
        let ref_count_0 = machine.automata_refs.len();
        machine.expr_0().unwrap();
        let ref_count_1 = machine.automata_refs.len();
        let (machine_a_q0, machine_a_f) = &machine.automata_refs[machine.automata_refs.len() - 1];

        assert_eq!(ref_count_0 + 1, ref_count_1, "expr_0 adds one automata ref");
        assert_eq!(
            machine.delta.len(),
            2,
            "Expression 0 (nothing) pushes two states"
        );
        assert_eq!(
            machine.delta[0],
            [None, None],
            "Expression 0 (nothing) cannot transition from q0"
        );
        assert_eq!(
            machine.delta[1],
            [None, None],
            "Expression 0 (nothing) cannot transition from f"
        );
        assert_ne!(
            machine_a_q0, machine_a_f,
            "Expression 0 (nothing) starts and ends on different states"
        );
    }

    #[test]
    fn test_expr_1() {
        let mut machine = ANFA::new();
        let ref_count_0 = machine.automata_refs.len();
        machine.expr_1().unwrap();
        let ref_count_1 = machine.automata_refs.len();
        let (machine_a_q0, machine_a_f) = &machine.automata_refs[machine.automata_refs.len() - 1];

        assert_eq!(ref_count_0 + 1, ref_count_1, "expr_0 adds one automata ref");
        assert_eq!(
            machine.delta.len(),
            1,
            "Expression 1 (epsilon) pushes one state"
        );
        assert_eq!(
            machine.delta[0],
            [None, None],
            "Expression 1 (epsilon) does not transition from q0"
        );
        assert_eq!(
            machine_a_q0, machine_a_f,
            "Expression 1 (epsilon) starts and ends on the same state"
        );
    }

    #[test]
    fn test_expr_a() {
        let mut machine = ANFA::new();
        let ref_count_0 = machine.automata_refs.len();
        machine.expr_a('a').unwrap();
        let ref_count_1 = machine.automata_refs.len();
        let (machine_a_q0, machine_a_f) = &machine.automata_refs[machine.automata_refs.len() - 1];

        assert_eq!(ref_count_0 + 1, ref_count_1, "expr_0 adds one automata ref");
        assert_eq!(
            machine.delta.len(),
            2,
            "Expression 'a' (literal) pushes two states"
        );
        assert_eq!(
            machine.delta[0],
            [Some((Some('a'), 1)), None],
            "Expression 'a' (literal) transitions from q0 to f along 'a'"
        );
        assert_eq!(
            machine.delta[1],
            [None, None],
            "Expression 'a' (literal) cannot transition from f"
        );
        assert_ne!(
            machine_a_q0, machine_a_f,
            "Expression 'a' (literal) starts and ends on different states"
        );
    }
}
