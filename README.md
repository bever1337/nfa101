# Regexxx, an implementation of Augmented Non-Deterministic Finite Automata

[Documentation](https://bever1337.github.io/regexxx/regexxx/index.html)

## Preface

- [Optimally Streaming Greedy Regular Expression Parsing](./pubs/grathwohl2014-0-paper.pdf)
- [Bit-coded Regular Expression Parsing](./pubs/fritz-paper.pdf)
- [Regular Expression Search Algorithm](./pubs/Thompson-1968.pdf)

Augmented non-deterministic finite automata are defined by the 4-tuple:

- state, finite set of states
- delta, `δ ⊆ State × T × State` is a labeled transition relation with labels `T = Σ ⊎ {0, 1, ε}`
- q0, initial state
- f, final state

## Implementation

```rust
use regexxx::compilers::forward_compiler::{Compiler, ForwardCompiler};
// RE a(b|c)*d
let mut machine = ForwardCompiler::from_expr_a('a').unwrap();
ForwardCompiler::expr_a(&mut machine, 'b').unwrap();
ForwardCompiler::expr_a(&mut machine, 'c').unwrap();
ForwardCompiler::union(&mut machine).unwrap();
ForwardCompiler::star(&mut machine).unwrap();
ForwardCompiler::expr_a(&mut machine, 'd').unwrap();
ForwardCompiler::concatenate(&mut machine).unwrap();
ForwardCompiler::concatenate(&mut machine).unwrap();
```
