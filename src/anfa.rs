use crate::alloc::vec;

/// Unique state id
pub type QId = usize;

/// A Transition tuple is the 2nd and 3rd arguments `T` and `State` in `δ ⊆ State × T × State`
pub type Transition = Option<(Option<char>, QId)>;

/// DeltaQ represents the function `δ ⊆ State × T × State`, arguments explained:
/// `State` - An index of DeltaQ
/// `T` - An optionally labeled transition
/// `State` - Next index of DeltaQ
pub type DeltaQ = alloc::vec::Vec<[Transition; 2]>;

/// Reference to automata initial and final states
#[derive(Debug, PartialEq)]
pub struct AutomataRef {
    /// initial state
    pub q0: QId,
    /// final state
    pub f: QId,
}

#[derive(Debug)]
pub struct ANFA {
    automata_refs: vec::Vec<AutomataRef>,
    delta: DeltaQ,
}

impl ANFA {
    pub fn new() -> ANFA {
        ANFA {
            automata_refs: vec![],
            delta: vec![],
        }
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
    pub fn expr_0(&mut self) -> Result<(), &'static str> {
        let q0 = self.delta.len();
        let f = q0 + 1;
        let machine_a = AutomataRef { q0, f };
        self.delta.push([None, None]);
        self.delta.push([None, None]);
        self.automata_refs.push(machine_a);
        Ok(())
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
    pub fn expr_1(&mut self) -> Result<(), &'static str> {
        let q0: usize = self.delta.len();
        let machine_a = AutomataRef { q0, f: q0 };
        self.delta.push([None, None]);
        self.automata_refs.push(machine_a);
        Ok(())
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
    pub fn expr_a(&mut self, c: char) -> Result<(), &'static str> {
        let q0 = self.delta.len();
        let f = q0 + 1;
        let machine_a = AutomataRef { q0, f };
        self.delta.push([Some((Some(c), f)), None]);
        self.delta.push([None, None]);
        self.automata_refs.push(machine_a);
        Ok(())
    }

    pub fn concatenate(&mut self) -> Result<(), &'static str> {
        let machine_b = self.automata_refs.pop().unwrap();
        let machine_a = self.automata_refs.pop().unwrap();
        let machine_c = AutomataRef {
            q0: machine_a.q0,
            f: machine_b.f,
        };
        self.delta[machine_a.f] = [Some((None, machine_b.q0)), None];
        self.automata_refs.push(machine_c);
        Ok(())
    }

    pub fn star(&mut self) -> Result<(), &'static str> {
        let q0 = self.delta.len();
        let q_next = q0 + 1;
        let f = q_next + 1;
        let machine_b = AutomataRef { q0, f };
        let machine_a = self.automata_refs.pop().unwrap();
        self.delta.push([Some((None, q_next)), None]);
        self.delta
            .push([Some((None, machine_a.q0)), Some((None, f))]);
        self.delta.push([None, None]);
        self.delta[machine_a.f] = [Some((None, q_next)), None];
        self.automata_refs.push(machine_b);
        Ok(())
    }

    pub fn union(&mut self) -> Result<(), &'static str> {
        let q0 = self.delta.len();
        let f = q0 + 1;
        let machine_c = AutomataRef { q0, f };
        let machine_b = self.automata_refs.pop().unwrap();
        let machine_a = self.automata_refs.pop().unwrap();
        self.delta
            .push([Some((None, machine_a.q0)), Some((None, machine_b.q0))]);
        self.delta.push([None, None]);
        self.delta[machine_a.f] = [Some((None, f)), None];
        self.delta[machine_b.f] = [Some((None, f)), None];
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
        let machine_a = &machine.automata_refs[machine.automata_refs.len() - 1];

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
            machine_a.q0, machine_a.f,
            "Expression 0 (nothing) starts and ends on different states"
        );
    }

    #[test]
    fn test_expr_1() {
        let mut machine = ANFA::new();
        let ref_count_0 = machine.automata_refs.len();
        machine.expr_1().unwrap();
        let ref_count_1 = machine.automata_refs.len();
        let machine_a = &machine.automata_refs[machine.automata_refs.len() - 1];

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
            machine_a.q0, machine_a.f,
            "Expression 1 (epsilon) starts and ends on the same state"
        );
    }

    #[test]
    fn test_expr_a() {
        let mut machine = ANFA::new();
        let ref_count_0 = machine.automata_refs.len();
        machine.expr_a('a').unwrap();
        let ref_count_1 = machine.automata_refs.len();
        let machine_a = &machine.automata_refs[machine.automata_refs.len() - 1];

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
            machine_a.q0, machine_a.f,
            "Expression 'a' (literal) starts and ends on different states"
        );
    }
}
