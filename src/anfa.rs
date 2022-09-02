// size of QId
// size of label
use alloc::vec;

/// Unique state id
/// TODO: Compile a regex large enough to overflow
pub type QId = usize;

/// A transition along an optional label to zero, one, or two States.
/// When a label is `None`, transition is an epsilon transition
/// and it always advances to its final states. When QId is `None`,
/// there is no transition. Transition is ordered. If `Transition.1[1]`
/// is `Some(QId)`, then `Transition.1[0]` must also be `Some(QId)`.
/// i.e. a union operation is when both `Option<QId>` are `Some(QId)`.
pub type Transition = (Option<char>, [Option<QId>; 2]);

/// DeltaFunction is a vector of ordered transitions that satisfy
/// the function `δ ⊆ State × T × State`. An index of `DeltaFunction`
/// is the first `State` paramter in the function. See `Transition`.
pub type DeltaFunction = vec::Vec<Transition>;

/// The initial and final states of an expression: [q0, f]
pub type AutomataRef = [QId; 2];

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
            automata_refs: vec::Vec::with_capacity(u32::MAX as usize),
            delta: vec::Vec::with_capacity(u32::MAX as usize),
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
        let machine_a = [q0, f];
        self.delta.push((
            // push non-transitioning state
            None,
            [None, None],
        ));
        self.delta.push((
            // push final state
            None,
            [None, None],
        ));
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
        let q0 = self.delta.len();
        let f = q0;
        let machine_a = [q0, f];
        self.delta.push((
            // push final state
            None,
            [None, None],
        ));
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
        let machine_a = [q0, f];
        self.delta.push((
            // push transition to Q `f` along Label `c`
            Some(c),
            [Some(f), None],
        ));
        self.delta.push((
            // push final state
            None,
            [None, None],
        ));
        self.automata_refs.push(machine_a);
        Ok(())
    }

    /// Concatenate machines 'a' and 'b'
    ///
    /// ```rust
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
        match self.automata_refs.len() {
            0 | 1 => {
                return Err("Concatenation requires two operands.");
            }
            _ => {}
        };
        let [machine_b_q0, machine_b_f] = match self.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Concatenation requires two operands. (Race condition.)");
            }
            Some(machine_b) => machine_b,
        };
        let [machine_a_q0, machine_a_f] = match self.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Concatenation requires two operands. (Race condition.)");
            }
            Some(machine_a) => machine_a,
        };
        let machine_c = [machine_a_q0, machine_b_f];
        self.delta[machine_a_f] = (
            // point 'a' at 'b'
            None,
            [Some(machine_b_q0), None],
        );
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
    /// | Q | T | Q    |
    /// |---|---|------|
    /// | 0 | a | 1    |
    /// | 1 | ε | 3    |
    /// | 2 | ε | 3    | (q0)
    /// | 3 | ε | 0, 4 |
    /// | 4 |   |      | (f)
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
        match self.automata_refs.len() {
            0 => {
                return Err("Star requires one operand.");
            }
            _ => {}
        };
        let [machine_a_q0, machine_a_f] = match self.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Star requires one operand. (Race condition.)");
            }
            Some(machine_a) => machine_a,
        };
        let machine_b_q0 = self.delta.len();
        let machine_b_q = machine_b_q0 + 1;
        let machine_b_f = machine_b_q0 + 2;
        let machine_b = [machine_b_q0, machine_b_f];
        self.delta.push((
            // push epsilon transition to union
            None,
            [Some(machine_b_q), None],
        ));
        self.delta.push((
            // push union of machine_a and final state
            None,
            [Some(machine_a_q0), Some(machine_b_f)],
        ));
        self.delta.push((
            // push final state
            None,
            [None, None],
        ));
        self.delta[machine_a_f] = (
            // point machine_a at union
            None,
            [Some(machine_b_q), None],
        );
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
    /// | Q | T | Q    |
    /// |---|---|------|
    /// | 0 | a | 1    |
    /// | 1 | ε | 5    |
    /// | 2 | b | 3    |
    /// | 3 | ε | 5    |
    /// | 4 | ε | 0, 2 | (q0)
    /// | 4 | ε | 2    | (q0)
    /// | 5 |   |      | (f)
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
        let machine_c_q0 = self.delta.len();
        match machine_c_q0 {
            0 | 1 => {
                return Err("Union requires two operands.");
            }
            _ => {}
        };
        let machine_c_f = machine_c_q0 + 1;
        let machine_c = [machine_c_q0, machine_c_f];
        let [machine_b_q0, machine_b_f] = match self.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Union requires two operands. (Race condition.)");
            }
            Some(machine_b) => machine_b,
        };
        let [machine_a_q0, machine_a_f] = match self.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Union requires two operands. (Race condition.)");
            }
            Some(machine_a) => machine_a,
        };
        self.delta.push((
            // push union transition
            None,
            [Some(machine_a_q0), Some(machine_b_q0)],
        ));
        self.delta.push((
            // push final state
            None,
            [None, None],
        ));
        self.delta[machine_a_f] = (
            // point machine_a at machine_c
            None,
            [Some(machine_c_f), None],
        );
        self.delta[machine_b_f] = (
            // point machine_b at machine_c
            None,
            [Some(machine_c_f), None],
        );
        self.automata_refs.push(machine_c);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::anfa::ANFA;

    #[test]
    fn test_expr_0() {
        let mut machine = ANFA::from_expr_0().unwrap();
        assert_eq!(
            machine.automata_refs.len(),
            1,
            "Expression 0 (nothing) pushes one machine"
        );
        assert_eq!(
            machine.delta.len(),
            2,
            "Expression 0 (nothing) pushes two states"
        );
        assert_eq!(
            machine.delta[0],
            (None, [None, None]),
            "Expression 0 (nothing) cannot transition from q0"
        );
        assert_eq!(
            machine.delta[1],
            (None, [None, None]),
            "Expression 0 (nothing) cannot transition from f"
        );
        let [machine_a_q0, machine_a_f] = &machine.automata_refs[machine.automata_refs.len() - 1];
        assert_ne!(
            machine_a_q0, machine_a_f,
            "Expression 0 (nothing) starts and ends on different states"
        );

        // run twice to make sure pushing expressions isn't affected by prior pushed expressions
        machine.expr_0().unwrap();
        assert_eq!(
            machine.automata_refs.len(),
            2,
            "(Repeated) Expression 0 (nothing) pushes one machine"
        );
        assert_eq!(
            machine.delta.len(),
            4,
            "(Repeated) Expression 0 (nothing) pushes two states"
        );
    }

    #[test]
    fn test_expr_1() {
        let mut machine = ANFA::from_expr_1().unwrap();

        assert_eq!(
            machine.automata_refs.len(),
            1,
            "Expression 1 (epsilon) pushes one automata ref"
        );
        assert_eq!(
            machine.delta.len(),
            1,
            "Expression 1 (epsilon) pushes one state"
        );
        assert_eq!(
            machine.delta[0],
            (None, [None, None]),
            "Expression 1 (epsilon) does not transition from q0"
        );
        let [machine_a_q0, machine_a_f] = &machine.automata_refs[machine.automata_refs.len() - 1];
        assert_eq!(
            machine_a_q0, machine_a_f,
            "Expression 1 (epsilon) starts and ends on the same state"
        );
        // run twice to make sure pushing expressions isn't affected by prior pushed expressions
        machine.expr_1().unwrap();
        assert_eq!(
            machine.automata_refs.len(),
            2,
            "(Repeated) Expression 1 (epsilon) pushes one automata ref"
        );
        assert_eq!(
            machine.delta.len(),
            2,
            "(Repeated) Expression 1 (epsilon) pushes one state"
        );
    }

    #[test]
    fn test_expr_a() {
        let mut machine = ANFA::from_expr_a('a').unwrap();

        assert_eq!(
            machine.automata_refs.len(),
            1,
            "Expression 'a' (literal) pushes one automata ref"
        );
        assert_eq!(
            machine.delta.len(),
            2,
            "Expression 'a' (literal) pushes two states"
        );
        assert_eq!(
            machine.delta[0],
            (Some('a'), [Some(1), None]),
            "Expression 'a' (literal) transitions from q0 to f along 'a'"
        );
        assert_eq!(
            machine.delta[1],
            (None, [None, None]),
            "Expression 'a' (literal) cannot transition from f"
        );
        let [machine_a_q0, machine_a_f] = &machine.automata_refs[machine.automata_refs.len() - 1];
        assert_ne!(
            machine_a_q0, machine_a_f,
            "Expression 'a' (literal) starts and ends on different states"
        );
        // run twice to make sure pushing expressions isn't affected by prior pushed expressions
        machine.expr_a('b').unwrap();
        assert_eq!(
            machine.automata_refs.len(),
            2,
            "(Repeated) Expression 'a' (literal) pushes one automata ref"
        );
        assert_eq!(
            machine.delta.len(),
            4,
            "(Repeated) Expression 'a' (literal) pushes two states"
        );
    }

    #[test]
    fn test_concatenate() {
        let mut machine = ANFA::from_expr_a('a').unwrap();
        machine.expr_a('b').unwrap();
        assert_eq!(
            4,
            machine.delta.len(),
            "Expect four states before concatenation"
        );
        assert_eq!(
            2,
            machine.automata_refs.len(),
            "Expect two machines before concatenation"
        );
        machine.concatenate().unwrap();
        assert_eq!(
            4,
            machine.delta.len(),
            "Concatenation does not create new states"
        );
        assert_eq!(
            1,
            machine.automata_refs.len(),
            "Concatenation is a binary operation, i.e. concatenation returns one less machine"
        );
        assert_eq!(
            machine.delta[1],
            (None, [Some(2), None]),
            "Concatenation transitions machine_a to machine_b along epsilon"
        );
    }
}
