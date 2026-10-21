use crate::circuit::Op;
// The base's gate-list-bound edit tables (row edits, AND cache, product copies, value edits) assume the base's exact
// stream and are not used by this build; the generic exact passes run.
pub(super) fn run(mut v:Vec<Op>)->Vec<Op>{
std::env::set_var("SL_COND","ur");v=super::cond_pass::run(v);v=super::zero_force::run(v);v=super::classical_factors_xor::simplify_classical(v);super::classical_wide::simplify_classical(v)}
