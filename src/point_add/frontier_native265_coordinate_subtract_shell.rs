// Generated from formal/ir/frontier-native265-coordinate-subtract-shell-v1.json.
// Generated Rust is output-only.
// ir_sha256=64b63a568e3f1ac535b4626898d71df0ef76f4481bdb615185e4d2280ad79a05
// exact_word_rust_sha256=5cf01093d4530a75e8e2610964d0be412f370ba34a5f8be51bcd7821722dfdf2
// generator_sha256=e785fbec6b7d0bc6eeca6a6881c566738ea6d136f1d668f1270141ba4c94cbe5

use super::{B, BExt};
use crate::circuit::{BitId, QubitId};

const WORD_BITS: usize = 256;

fn load_coordinate(circuit: &mut B, coordinate: &[BitId]) -> Vec<QubitId> {
    assert_eq!(coordinate.len(), WORD_BITS);
    let loaded = circuit.alloc_qubits(WORD_BITS);
    for index in 0..WORD_BITS { circuit.x_if_bit(loaded[index], coordinate[index]); }
    loaded
}

fn unload_coordinate(circuit: &mut B, coordinate: &[BitId], loaded: Vec<QubitId>) {
    assert_eq!(coordinate.len(), WORD_BITS);
    assert_eq!(loaded.len(), WORD_BITS);
    for index in 0..WORD_BITS { circuit.x_if_bit(loaded[index], coordinate[index]); }
    for owner in loaded { circuit.zero_and_free(owner); }
}

pub(super) fn emit_frontier_native265_coordinate_subtract(
    circuit: &mut B, destination: &mut Vec<QubitId>, coordinate: &[BitId],
) {
    assert_eq!(destination.len(), WORD_BITS);
    let loaded = load_coordinate(circuit, coordinate);
    super::frontier_k2_exact_word_subtract_negate::emit_frontier_k2_exact_word_subtract(
        circuit, &loaded, destination,
    );
    unload_coordinate(circuit, coordinate, loaded);
}

pub(super) fn emit_frontier_native265_coordinate_reverse_subtract(
    circuit: &mut B, destination: &mut Vec<QubitId>, coordinate: &[BitId],
) {
    assert_eq!(destination.len(), WORD_BITS);
    let loaded = load_coordinate(circuit, coordinate);
    super::frontier_k2_exact_word_subtract_negate::emit_frontier_k2_exact_word_subtract(
        circuit, &loaded, destination,
    );
    super::frontier_k2_exact_word_subtract_negate::emit_frontier_k2_exact_word_negate(
        circuit, destination,
    );
    unload_coordinate(circuit, coordinate, loaded);
}
