# Regexxx, an implementation of Augmented Non-Deterministic Finite Automata

[Documentation](https://bever1337.github.io/regexxx/automata/index.html)

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
use crate::regexxx;
// RE (a ∪ b)*b
let mut machine = ANFA::new();
let machine_ref_a = machine.expr_a('a').unwrap();
let machine_ref_b = machine.expr_a('b').unwrap();
let machine_ref_c = machine.union(&machine_ref_a, &machine_ref_b).unwrap();
let machine_ref_d = machine.star(&machine_ref_c).unwrap();
let machine_ref_e = machine.expr_a('b').unwrap();
let machine_ref_f = machine.concatenate(&machine_ref_d, &machine_ref_e).unwrap();
machine.in_and_fin(&ref_f).unwrap();
println!("{:#?}", machine);
```
