# nfa101

I naively wanted to pattern match an arbitrary stream of data using regular expressions and I discovered implementations are lacking or non-existent. Now I'm working from the ground up and prototyping a language engine that matches on arbitrary streams of bytes. The first goal is a formal NFA implementation. It is up to the developer to enforce specific compilation algorithms like "Thompson's Construction."

### 1. Defines an NFA implementation by the 5-tuple (Q, Σ, δ, q0, F)

1. Q: Set of all possible states,
2. Epsilon Σ: The finite alphabet
3. Delta δ: A function where (State X Input Symbol) => Set of states
4. q0: The initial Q
5. F: The set of all accepting/'finished' states

Input:

```rust
println!("Literal 'A':\n{}", FA::from_literal('A').unwrap());
println!(
    "Concatenate {{a}}, {{p}}, {{p}}, {{l}}, {{e}}:\n{}",
    FA::from_concatenation(vec![
        FA::from_literal('A').unwrap(),
        FA::from_literal('P').unwrap(),
        FA::from_literal('P').unwrap(),
        FA::from_literal('L').unwrap(),
        FA::from_literal('E').unwrap(),
    ])
    .unwrap()
);
```

Output:

```
Literal 'A':
Finite Automata = (
   Q: { 0, 1 },
   Σ: { 0..255 },
   δ: (Q, Σ) -> [Q],
     0: A -> { 1 }
     1:
   q0: 0,
   F: { 1 }
)

Concatenate {a}, {p}, {p}, {l}, {e}:
Finite Automata = (
   Q: { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 },
   Σ: { 0..255 },
   δ: (Q, Σ) -> [Q],
     0: A -> { 1 }
     1: ε -> { 2 }
     2: P -> { 3 }
     3: ε -> { 4 }
     4: P -> { 5 }
     5: ε -> { 6 }
     6: L -> { 7 }
     7: ε -> { 8 }
     8: E -> { 9 }
     9:
   q0: 0,
   F: { 9 }
)
```

### 2. NFA construction

1. From literal ✓

Input:

```rust
// Simplest NFA, a single literal
println!("{:#?}", FA::from_literal('A'));
// An epsilon transition is an edge with no value
println!("{:#?}", Edge(0, None, 1));
```

Output:

```
Ok(
    FA {
        matches: [1],
        states: [
            [Edge(0, Some('A', ), 1)],
            [],
        ],
    },
)
Edge(0, None, 1)
```

2. Union
3. Intersection
4. Complement
5. Difference
6. Concatenation ✓

```rust
// Concatenation is a binary operation composed left-to-right
println!(
    "Debug: concatenate {{a}}, {{p}}, {{p}}, {{l}}, {{e}}:\n{:#?}",
    FA::from_concatenation(vec![
        FA::from_literal('A').unwrap(),
        FA::from_literal('P').unwrap(),
        FA::from_literal('P').unwrap(),
        FA::from_literal('L').unwrap(),
        FA::from_literal('E').unwrap(),
    ])
    .unwrap()
);
/**
Debug: concatenate {a}, {p}, {p}, {l}, {e}:
FA {
    matches: [9],
    states: [
        [Edge(0, Some('A'), 1)],
        [Edge(1, None, 2)],
        [Edge(2, Some('P'), 3)],
        [Edge(3, None, 4)],
        [Edge(4, Some('P'), 5)],
        [Edge(5, None, 6)],
        [Edge(6, Some('L'), 7)],
        [Edge(7, None, 8)],
        [Edge(8, Some('E'), 9)],
        [],
    ],
}
**/
```

7. klenne star
