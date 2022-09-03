use crate::ANFA;

pub mod coverage_compiler;
pub mod vanilla_compiler;

pub trait Compiler {
  fn from_expr_0() -> Result<ANFA, &'static str>;
  fn from_expr_1() -> Result<ANFA, &'static str>;
  fn from_expr_a(c: char) -> Result<ANFA, &'static str>;
  fn expr_0(anfa: &mut ANFA) -> Result<(), &'static str>;
  fn expr_1(anfa: &mut ANFA) -> Result<(), &'static str>;
  fn expr_a(anfa: &mut ANFA, c: char) -> Result<(), &'static str>;
  fn concatenate(anfa: &mut ANFA) -> Result<(), &'static str>;
  fn star(anfa: &mut ANFA) -> Result<(), &'static str>;
  fn union(anfa: &mut ANFA) -> Result<(), &'static str>;
}
