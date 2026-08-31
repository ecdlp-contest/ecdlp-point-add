// Generated from formal/ir/frontier-k2-controlled-word-swap-v1.json.
// Generated Rust is output-only.
// ir_sha256=9a92dd0991c4e529c35222873ad36b39cdd6d19f2f061f3cc12e391d6c4b3733
// generator_sha256=53680b979742c48376afd6b5bc636314b3bd4d3146df5b7c672cc457f55c4779

use super::B;
use crate::circuit::QubitId;

pub(super) const FRONTIER_K2_WORD_WIDTH: usize = 256;

pub(super) fn emit_frontier_k2_controlled_word_swap(
    circuit: &mut B,
    control: QubitId,
    x: &mut Vec<QubitId>,
    y: &mut Vec<QubitId>,
) {
    assert_eq!(x.len(), FRONTIER_K2_WORD_WIDTH);
    assert_eq!(y.len(), FRONTIER_K2_WORD_WIDTH);
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        circuit.cx(xi, yi);
        circuit.ccx(control, yi, xi);
        circuit.cx(xi, yi);
    }
}
