# nfa101

I naively wanted to pattern match an arbitrary stream of data using regular expressions and I discovered implementations are lacking or non-existent. Now I'm working from the ground up and prototyping a language engine that matches on arbitrary streams of bytes. The first goal is a formal NFA implementation. It is up to the developer to enforce specific compilation algorithms like "Thompson's Construction."

### 1. Defines an NFA implementation by the 5-tuple (Q, Σ, δ, q0, F)

1. Q: Set of all possible states,
2. Epsilon Σ: The finite alphabet
3. Delta δ: A function where (State X Input Symbol) => Set of states
4. q0: The initial Q
5. F: The set of all accepting/'finished' states

### 2. NFA construction

1. From literal ✓

Input:

```rust
// Simplest NFA, a single literal
println!("{:#?}", FA::from_literal('a').unwrap());
println!("{}", FA::from_literal('a').unwrap());
```

Output:

```
Ok(
    FA {
        delta: [
            {
                Some(
                    'a',
                ): [
                    1,
                ],
            },
            {},
        ],
        q0: 0,
        f: [
            1,
        ],
    }
)
```

2. Union ✓

Input:

```rust
println!(
    "The machine 'orange|apple':\n{}",
    FA::from_union(
        FA::from_concatenation(
            FA::from_concatenation(
                FA::from_concatenation(
                    FA::from_concatenation(
                        FA::from_concatenation(
                            FA::from_literal('o').unwrap(),
                            FA::from_literal('r').unwrap(),
                        )
                        .unwrap(),
                        FA::from_literal('a').unwrap(),
                    )
                    .unwrap(),
                    FA::from_literal('n').unwrap(),
                )
                .unwrap(),
                FA::from_literal('g').unwrap(),
            )
            .unwrap(),
            FA::from_literal('e').unwrap()
        )
        .unwrap(),
        FA::from_concatenation(
            FA::from_concatenation(
                FA::from_concatenation(
                    FA::from_concatenation(
                        FA::from_literal('a').unwrap(),
                        FA::from_literal('p').unwrap(),
                    )
                    .unwrap(),
                    FA::from_literal('p').unwrap(),
                )
                .unwrap(),
                FA::from_literal('l').unwrap(),
            )
            .unwrap(),
            FA::from_literal('e').unwrap(),
        )
        .unwrap()
    )
    .unwrap()
);
```

Output:

```
The machine 'orange|apple':
Finite Automata = (
   Q: { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22 },
   Σ: { 0..255 },
   δ: (Q, Σ) -> [Q],
     0: Q, o: Σ -> { 1 }
     1: Q, ε: Σ -> { 2 }
     2: Q, r: Σ -> { 3 }
     3: Q, ε: Σ -> { 4 }
     4: Q, a: Σ -> { 5 }
     5: Q, ε: Σ -> { 6 }
     6: Q, n: Σ -> { 7 }
     7: Q, ε: Σ -> { 8 }
     8: Q, g: Σ -> { 9 }
     9: Q, ε: Σ -> { 10 }
     10: Q, e: Σ -> { 11 }
     11: Q
     12: Q, ε: Σ -> { 0, 13 }
     13: Q, a: Σ -> { 14 }
     14: Q, ε: Σ -> { 15 }
     15: Q, p: Σ -> { 16 }
     16: Q, ε: Σ -> { 17 }
     17: Q, p: Σ -> { 18 }
     18: Q, ε: Σ -> { 19 }
     19: Q, l: Σ -> { 20 }
     20: Q, ε: Σ -> { 21 }
     21: Q, e: Σ -> { 22 }
     22: Q
   q0: 12,
   F: { 11, 22 }
)
```

3. Intersection
4. Complement
5. Difference
6. Concatenation ✓

Input:

```rust
println!(
    "The concatenation of 'apple':\n{}",
    FA::from_concatenation(
        FA::from_concatenation(
            FA::from_concatenation(
                FA::from_concatenation(
                    FA::from_literal('a').unwrap(),
                    FA::from_literal('p').unwrap(),
                )
                .unwrap(),
                FA::from_literal('p').unwrap(),
            )
            .unwrap(),
            FA::from_literal('l').unwrap(),
        )
        .unwrap(),
        FA::from_literal('e').unwrap(),
    )
    .unwrap()
);
```

Output:

```
The concatenation of 'apple':
Finite Automata = (
   Q: { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 },
   Σ: { 0..255 },
   δ: (Q, Σ) -> [Q],
     0: Q, a: Σ -> { 1 }
     1: Q, ε: Σ -> { 2 }
     2: Q, p: Σ -> { 3 }
     3: Q, ε: Σ -> { 4 }
     4: Q, p: Σ -> { 5 }
     5: Q, ε: Σ -> { 6 }
     6: Q, l: Σ -> { 7 }
     7: Q, ε: Σ -> { 8 }
     8: Q, e: Σ -> { 9 }
     9: Q
   q0: 0,
   F: { 9 }
)
```

7. klenne star
