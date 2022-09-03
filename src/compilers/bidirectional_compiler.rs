// size of QId
// size of label
use crate::compilers::coverage_compiler::CoverageCompiler;
use crate::compilers::forward_compiler::ForwardCompiler;
pub use crate::compilers::Compiler;
use crate::ANFA;

pub trait Compilers {
  fn from_expr_0() -> Result<[ANFA; 2], &'static str>;
  fn from_expr_1() -> Result<[ANFA; 2], &'static str>;
  fn from_expr_a(c: char) -> Result<[ANFA; 2], &'static str>;
  fn expr_0(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str>;
  fn expr_1(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str>;
  fn expr_a(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
    c: char,
  ) -> Result<((), ()), &'static str>;
  fn concatenate(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str>;
  fn star(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str>;
  fn union(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str>;
}

pub struct BidirectionalCompiler {}
impl Compilers for BidirectionalCompiler {
  fn from_expr_0() -> Result<[ANFA; 2], &'static str> {
    let mut forward_machine_a = ANFA::new();
    let mut coverage_machine_a = ANFA::new();
    match [
      ForwardCompiler::expr_0(&mut forward_machine_a),
      CoverageCompiler::expr_0(&mut coverage_machine_a),
    ] {
      [Ok(()), Ok(())] => Ok([forward_machine_a, coverage_machine_a]),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }

  fn from_expr_1() -> Result<[ANFA; 2], &'static str> {
    let mut forward_machine_a = ANFA::new();
    let mut coverage_machine_a = ANFA::new();
    match [
      ForwardCompiler::expr_1(&mut forward_machine_a),
      CoverageCompiler::expr_1(&mut coverage_machine_a),
    ] {
      [Ok(()), Ok(())] => Ok([forward_machine_a, coverage_machine_a]),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }

  fn from_expr_a(c: char) -> Result<[ANFA; 2], &'static str> {
    let mut forward_machine_a = ANFA::new();
    let mut coverage_machine_a = ANFA::new();
    match [
      ForwardCompiler::expr_a(&mut forward_machine_a, c),
      CoverageCompiler::expr_a(&mut coverage_machine_a, c),
    ] {
      [Ok(()), Ok(())] => Ok([forward_machine_a, coverage_machine_a]),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }

  fn expr_0(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str> {
    match [
      ForwardCompiler::expr_0(forward_machine),
      CoverageCompiler::expr_0(coverage_machine),
    ] {
      [Ok(()), Ok(())] => Ok(((), ())),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }

  fn expr_1(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str> {
    match [
      ForwardCompiler::expr_1(forward_machine),
      CoverageCompiler::expr_1(coverage_machine),
    ] {
      [Ok(()), Ok(())] => Ok(((), ())),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }

  fn expr_a(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
    c: char,
  ) -> Result<((), ()), &'static str> {
    match [
      ForwardCompiler::expr_a(forward_machine, c),
      CoverageCompiler::expr_a(coverage_machine, c),
    ] {
      [Ok(()), Ok(())] => Ok(((), ())),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }

  fn concatenate(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str> {
    match [
      ForwardCompiler::concatenate(forward_machine),
      CoverageCompiler::concatenate(coverage_machine),
    ] {
      [Ok(()), Ok(())] => Ok(((), ())),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }

  fn star(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str> {
    match [
      ForwardCompiler::star(forward_machine),
      CoverageCompiler::star(coverage_machine),
    ] {
      [Ok(()), Ok(())] => Ok(((), ())),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }

  fn union(
    forward_machine: &mut ANFA,
    coverage_machine: &mut ANFA,
  ) -> Result<((), ()), &'static str> {
    match [
      ForwardCompiler::union(forward_machine),
      CoverageCompiler::union(coverage_machine),
    ] {
      [Ok(()), Ok(())] => Ok(((), ())),
      [Err(forward_machine_error), _] => Err(forward_machine_error),
      [_, Err(coverage_machine_error)] => Err(coverage_machine_error),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::compilers::bidirectional_compiler::{BidirectionalCompiler, Compilers};

  #[test]
  fn test_expr_0() {
    // RE a(b|c)*d
    let [mut forward_machine, mut coverage_machine] =
      BidirectionalCompiler::from_expr_a('a').unwrap();
    BidirectionalCompiler::expr_a(&mut forward_machine, &mut coverage_machine, 'b').unwrap();
    BidirectionalCompiler::expr_a(&mut forward_machine, &mut coverage_machine, 'c').unwrap();
    BidirectionalCompiler::union(&mut forward_machine, &mut coverage_machine).unwrap();
    BidirectionalCompiler::star(&mut forward_machine, &mut coverage_machine).unwrap();
    BidirectionalCompiler::expr_a(&mut forward_machine, &mut coverage_machine, 'd').unwrap();
    BidirectionalCompiler::concatenate(&mut forward_machine, &mut coverage_machine).unwrap();
    BidirectionalCompiler::concatenate(&mut forward_machine, &mut coverage_machine).unwrap();
  }
}
