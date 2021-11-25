#

## Preface

See `grathwohl2014-0-paper.pdf` in the docs folder

Augmented non-deterministic finite automata are defined by the 4-tuple:

- state, finite set of states
- delta, `δ ⊆ State × T × State` is a labeled transition relation with labels `T = Σ ⊎ {0, 1, ε}`
- q0, initial state
- f, final state

## Implementation

```rust
use crate::ANFA;
// RE (a ∪ b)*b
let mut anfa = ANFA::new();
let mut ref_a = anfa.expr_a('a').unwrap();
let ref_b = anfa.expr_a('b').unwrap();
let ref_c = anfa.union(&mut ref_a, &ref_b).unwrap();
let ref_d = anfa.star(&ref_c).unwrap();
let ref_e = anfa.expr_a('b').unwrap();
let ref_f = anfa.concatenate(&ref_d, &ref_e).unwrap();
anfa.in_and_fin(&ref_f).unwrap();
println!("{:#?}", anfa);
```
