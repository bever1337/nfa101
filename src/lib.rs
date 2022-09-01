#![no_std]

extern crate alloc;

pub mod anfa;

// for ease of copy+paste:
// ε
// FA = (Q, Σ, δ, q0, F)
