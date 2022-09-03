// size of QId
// size of label
pub use crate::compilers::Compiler;
use crate::ANFA;

pub struct ForwardCompiler {}
impl Compiler for ForwardCompiler {
    /// Returns a new ANFA that never transitions to a final state.
    ///
    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let machine = ForwardCompiler::from_expr_0().unwrap(); // always safe!
    /// ```
    fn from_expr_0() -> Result<ANFA, &'static str> {
        let mut machine_a = ANFA::new();
        match ForwardCompiler::expr_0(&mut machine_a) {
            Ok(()) => Ok(machine_a),
            Err(e) => Err(e),
        }
    }

    /// Returns a new ANFA in its final state.
    ///
    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let machine = ForwardCompiler::from_expr_1().unwrap(); // always safe!
    /// ```
    fn from_expr_1() -> Result<ANFA, &'static str> {
        let mut machine_a = ANFA::new();
        match ForwardCompiler::expr_1(&mut machine_a) {
            Ok(()) => Ok(machine_a),
            Err(e) => Err(e),
        }
    }

    /// Returns a new ANFA that transitions to a final state on 'a'.
    ///
    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let mut machine = ForwardCompiler::from_expr_a('a').unwrap(); // always safe!
    /// ```
    fn from_expr_a(c: char) -> Result<ANFA, &'static str> {
        let mut machine_a = ANFA::new();
        match ForwardCompiler::expr_a(&mut machine_a, c) {
            Ok(()) => Ok(machine_a),
            Err(e) => Err(e),
        }
    }

    /// Pushes an acceptor that never transitions, i.e. accept nothing
    ///
    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let mut machine = ForwardCompiler::from_expr_a('a').unwrap(); // always safe!
    /// match ForwardCompiler::expr_0(&mut machine) {
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
    fn expr_0(anfa: &mut ANFA) -> Result<(), &'static str> {
        let q0 = anfa.delta.len();
        let f = q0 + 1;
        let machine_a = [q0, f];
        anfa.delta.push((
            // push non-transitioning state
            None,
            [None, None],
        ));
        anfa.delta.push((
            // push final state
            None,
            [None, None],
        ));
        anfa.automata_refs.push(machine_a);
        Ok(())
    }

    /// Pushes an acceptor in final state, i.e. accept anything, AKA epsilon acceptor
    ///
    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let mut machine = ForwardCompiler::from_expr_a('a').unwrap(); // always safe!
    /// match ForwardCompiler::expr_1(&mut machine) {
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
    fn expr_1(anfa: &mut ANFA) -> Result<(), &'static str> {
        let q0 = anfa.delta.len();
        let f = q0;
        let machine_a = [q0, f];
        anfa.delta.push((
            // push final state
            None,
            [None, None],
        ));
        anfa.automata_refs.push(machine_a);
        Ok(())
    }

    /// Pushes an automaton that transitions to a final state on 'a'
    ///
    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let mut machine = ForwardCompiler::from_expr_a('a').unwrap(); // always safe!
    /// match ForwardCompiler::expr_a(&mut machine, 'b') {
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
    fn expr_a(anfa: &mut ANFA, c: char) -> Result<(), &'static str> {
        let q0 = anfa.delta.len();
        let f = q0 + 1;
        let machine_a = [q0, f];
        anfa.delta.push((
            // push transition to Q `f` along Label `c`
            Some(c),
            [Some(f), None],
        ));
        anfa.delta.push((
            // push final state
            None,
            [None, None],
        ));
        anfa.automata_refs.push(machine_a);
        Ok(())
    }

    /// Concatenate machines 'a' and 'b'
    ///
    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let mut machine = ForwardCompiler::from_expr_a('a').unwrap(); // always safe
    /// ForwardCompiler::expr_a(&mut machine, 'b').unwrap(); // (should be) safe
    /// match ForwardCompiler::concatenate(&mut machine) {
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
    fn concatenate(anfa: &mut ANFA) -> Result<(), &'static str> {
        match anfa.automata_refs.len() {
            0 | 1 => {
                return Err("Concatenation requires two operands.");
            }
            _ => {}
        };
        let [machine_b_q0, machine_b_f] = match anfa.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Concatenation requires two operands. (Race condition.)");
            }
            Some(machine_b) => machine_b,
        };
        let [machine_a_q0, machine_a_f] = match anfa.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Concatenation requires two operands. (Race condition.)");
            }
            Some(machine_a) => machine_a,
        };
        let machine_c = [machine_a_q0, machine_b_f];
        anfa.delta[machine_a_f] = (
            // point 'a' at 'b'
            None,
            [Some(machine_b_q0), None],
        );
        anfa.automata_refs.push(machine_c);
        Ok(())
    }

    /// Star is a unary operation so that the last machine may be repeated 0 or more times.
    ///
    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let mut machine = ForwardCompiler::from_expr_a('a').unwrap(); // always safe!
    /// match ForwardCompiler::star(&mut machine) {
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
    fn star(anfa: &mut ANFA) -> Result<(), &'static str> {
        match anfa.automata_refs.len() {
            0 => {
                return Err("Star requires one operand.");
            }
            _ => {}
        };
        let [machine_a_q0, machine_a_f] = match anfa.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Star requires one operand. (Race condition.)");
            }
            Some(machine_a) => machine_a,
        };
        let machine_b_q0 = anfa.delta.len();
        let machine_b_q = machine_b_q0 + 1;
        let machine_b_f = machine_b_q0 + 2;
        let machine_b = [machine_b_q0, machine_b_f];
        anfa.delta.push((
            // push epsilon transition to union
            None,
            [Some(machine_b_q), None],
        ));
        anfa.delta.push((
            // push union of machine_a and final state
            None,
            [Some(machine_a_q0), Some(machine_b_f)],
        ));
        anfa.delta.push((
            // push final state
            None,
            [None, None],
        ));
        anfa.delta[machine_a_f] = (
            // point machine_a at union
            None,
            [Some(machine_b_q), None],
        );
        anfa.automata_refs.push(machine_b);
        Ok(())
    }

    /// ```rust
    /// use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
    /// let mut machine = ForwardCompiler::from_expr_a('a').unwrap(); // always safe!
    /// ForwardCompiler::expr_a(&mut machine, 'b').unwrap(); // (should be) always safe!
    /// match ForwardCompiler::union(&mut machine) {
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
    fn union(anfa: &mut ANFA) -> Result<(), &'static str> {
        let machine_c_q0 = anfa.delta.len();
        match machine_c_q0 {
            0 | 1 => {
                return Err("Union requires two operands.");
            }
            _ => {}
        };
        let machine_c_f = machine_c_q0 + 1;
        let machine_c = [machine_c_q0, machine_c_f];
        let [machine_b_q0, machine_b_f] = match anfa.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Union requires two operands. (Race condition.)");
            }
            Some(machine_b) => machine_b,
        };
        let [machine_a_q0, machine_a_f] = match anfa.automata_refs.pop() {
            None => {
                // exhaustive sanity check, should be impossible
                return Err("Union requires two operands. (Race condition.)");
            }
            Some(machine_a) => machine_a,
        };
        anfa.delta.push((
            // push union transition
            None,
            [Some(machine_a_q0), Some(machine_b_q0)],
        ));
        anfa.delta.push((
            // push final state
            None,
            [None, None],
        ));
        anfa.delta[machine_a_f] = (
            // point machine_a at machine_c
            None,
            [Some(machine_c_f), None],
        );
        anfa.delta[machine_b_f] = (
            // point machine_b at machine_c
            None,
            [Some(machine_c_f), None],
        );
        anfa.automata_refs.push(machine_c);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::compilers::forward_compiler::{Compiler, ForwardCompiler};

    #[test]
    fn test_expr_0() {
        let mut machine = ForwardCompiler::from_expr_0().unwrap();
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
        let [machine_a_q0, machine_a_f] = machine.automata_refs[0];
        assert_eq!(
            machine.delta[machine_a_q0],
            (None, [None, None]),
            "Expression 0 (nothing) cannot transition from q0"
        );
        assert_eq!(
            machine.delta[machine_a_f],
            (None, [None, None]),
            "Expression 0 (nothing) cannot transition from f"
        );
        assert_ne!(
            machine_a_q0, machine_a_f,
            "Expression 0 (nothing) starts and ends on different states"
        );

        // run twice to make sure pushing expressions isn't affected by prior pushed expressions
        ForwardCompiler::expr_0(&mut machine).unwrap();
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
        let mut machine = ForwardCompiler::from_expr_1().unwrap();

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
        ForwardCompiler::expr_1(&mut machine).unwrap();
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
        let mut machine = ForwardCompiler::from_expr_a('a').unwrap();

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
        let [machine_a_q0, machine_a_f] = machine.automata_refs[0];
        assert_eq!(
            machine.delta[machine_a_q0],
            (Some('a'), [Some(machine_a_f), None]),
            "Expression 'a' (literal) transitions from q0 to f along 'a'"
        );
        assert_eq!(
            machine.delta[machine_a_f],
            (None, [None, None]),
            "Expression 'a' (literal) cannot transition from f"
        );
        assert_ne!(
            machine_a_q0, machine_a_f,
            "Expression 'a' (literal) starts and ends on different states"
        );
        // run twice to make sure pushing expressions isn't affected by prior pushed expressions
        ForwardCompiler::expr_a(&mut machine, 'b').unwrap();
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
        let mut machine = ForwardCompiler::from_expr_a('a').unwrap();
        let [_machine_a_q0, machine_a_f] = machine.automata_refs[0];
        ForwardCompiler::expr_a(&mut machine, 'b').unwrap();
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
        let [machine_b_q0, _machine_b_f] = machine.automata_refs[1];
        ForwardCompiler::concatenate(&mut machine).unwrap();
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
        let [_machine_c_q0, _machine_c_f] = machine.automata_refs[0];
        assert_eq!(
            machine.delta[machine_a_f],
            (None, [Some(machine_b_q0), None]),
            "Concatenation transitions machine_a to machine_b along epsilon"
        );
    }

    #[test]
    fn test_star() {
        let mut machine = ForwardCompiler::from_expr_a('a').unwrap();
        let [machine_a_q0, machine_a_f] = machine.automata_refs[0];
        assert_eq!(
            machine.automata_refs.len(),
            1,
            "Star is unary, length of automatons won't change (pre-assertion)"
        );
        assert_eq!(
            machine.delta.len(),
            2,
            "Star pushes three new states (pre-assertion)"
        );
        ForwardCompiler::star(&mut machine).unwrap();
        assert_eq!(
            machine.automata_refs.len(),
            1,
            "Star is unary, length of automatons won't change"
        );
        assert_eq!(machine.delta.len(), 5, "Star pushes three new states");
        let [machine_b_q0, machine_b_f] = machine.automata_refs[0];
        let machine_b_intermediary_q = machine.delta[machine_a_f].1[0].unwrap();
        assert_eq!(
            machine.delta[machine_b_intermediary_q],
            (None, [Some(machine_a_q0), Some(machine_b_f)]),
            "(1) New intermediary state is a union of machine_a's q0 and new f, \
             (2) machine_a's f transtions to new intermediary state along epsilon"
        );
        assert_ne!(machine_a_q0, machine_b_q0, "Star pushes new initial state");
        assert_ne!(machine_a_f, machine_b_f, "Star pushes new final state");
    }

    #[test]
    fn test_union() {
        let mut machine = ForwardCompiler::from_expr_a('a').unwrap();
        let [machine_a_q0, machine_a_f] = machine.automata_refs[0];
        ForwardCompiler::expr_a(&mut machine, 'b').unwrap();
        let [machine_b_q0, machine_b_f] = machine.automata_refs[1];
        assert_eq!(
            machine.automata_refs.len(),
            2,
            "Union removes one automaton (pre-assertion)"
        );
        assert_eq!(
            machine.delta.len(),
            4,
            "Union pushes two states (pre-assertion)"
        );
        ForwardCompiler::union(&mut machine).unwrap();
        assert_eq!(
            machine.automata_refs.len(),
            1,
            "Union is a binary operation, consuming one automaton"
        );
        assert_eq!(
            machine.delta.len(),
            6,
            "Union requires two additional states"
        );
        let [machine_c_q0, machine_c_f] = machine.automata_refs[0];
        assert_eq!(
            machine.delta[machine_c_q0],
            (None, [Some(machine_a_q0), Some(machine_b_q0)]),
            "q0 of machine_c transitions to q0 of machine_a and machine_b along epsilon"
        );
        assert_eq!(
            machine.delta[machine_a_f],
            (None, [Some(machine_c_f), None]),
            "f of machine_a transitions to f of machine_c along epsilon"
        );
        assert_eq!(
            machine.delta[machine_b_f],
            (None, [Some(machine_c_f), None]),
            "f of machine_b transitions to f of machine_c along epsilon"
        );
    }
}
