// Generated construction entrypoint. Do not edit.
#![allow(dead_code,unused_variables,unused_mut,unused_parens,unreachable_code)]
mod runtime;
mod arithmetic;
pub fn build()->Vec<quantum_ecc::circuit::Op>{let mut c=runtime::B::new();arithmetic::construct(&mut c);c.finish()}
