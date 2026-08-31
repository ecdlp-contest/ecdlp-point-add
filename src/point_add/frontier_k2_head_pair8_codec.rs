// Generated from formal/ir/frontier-k2-head-pair8-codec-v1.json.
// Generated Rust is output-only.
// ir_sha256=f950821f4a7b8f24ee8b16bc9a4771e42498c18a35b0db09386f6dd5dc38139f
// generator_sha256=6021ec3dc64d3ab51d216cb83c073b7b57df0812cfabf9857db866d6a28c2808

use super::B;
use crate::circuit::QubitId;

pub(super) fn emit_frontier_k2_head_pair8_encode(circuit: &mut B, w: &[QubitId; 4]) {
    circuit.x(w[2]);
    circuit.x(w[3]);
    circuit.ccx(w[2], w[3], w[1]);
    circuit.x(w[3]);
    circuit.x(w[2]);
    circuit.ccx(w[0], w[1], w[2]);
    circuit.cx(w[1], w[0]);
    circuit.cx(w[2], w[0]);
}

pub(super) fn emit_frontier_k2_head_pair8_decode(circuit: &mut B, w: &[QubitId; 4]) {
    circuit.cx(w[2], w[0]);
    circuit.cx(w[1], w[0]);
    circuit.ccx(w[0], w[1], w[2]);
    circuit.x(w[2]);
    circuit.x(w[3]);
    circuit.ccx(w[2], w[3], w[1]);
    circuit.x(w[3]);
    circuit.x(w[2]);
}
