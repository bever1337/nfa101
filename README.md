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
use regexxx::compilers::vanilla_compiler::{Compiler, VanillaCompiler};
// RE a(b|c)*d
let mut machine = VanillaCompiler::from_expr_a('a').unwrap();
VanillaCompiler::expr_a(&mut machine, 'b').unwrap();
VanillaCompiler::expr_a(&mut machine, 'c').unwrap();
VanillaCompiler::union(&mut machine).unwrap();
VanillaCompiler::star(&mut machine).unwrap();
VanillaCompiler::expr_a(&mut machine, 'd').unwrap();
VanillaCompiler::concatenate(&mut machine).unwrap();
VanillaCompiler::concatenate(&mut machine).unwrap();
```
