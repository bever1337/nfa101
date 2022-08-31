#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

// for ease of copy+paste:
// ε
// FA = (Q, Σ, δ, q0, F)

pub mod anfa;
