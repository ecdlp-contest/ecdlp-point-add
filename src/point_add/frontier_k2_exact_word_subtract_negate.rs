// Generated from formal/ir/frontier-k2-exact-word-subtract-negate-v1.json.
// Generated Rust is output-only.
// ir_sha256=4382649d1f2e0077e3f7c4439754d1e6c90a7b858e89dc0cd26d955582f274f1
// controlled_row_rust_sha256=630d81e2bc0378aa1f78620e13b55d76fae159ebc0ff63d7f22d2850a49d9563
// low73_rust_sha256=196d364e510606a8c157073d28cd71d6f064ee9be720f8ba9349466258729e00
// generator_sha256=fdd719bfcbe3da737871687cb7e7f3378c3989b48f64426315e59277b4d5ebff

use super::B;
use crate::circuit::QubitId;

const WORD_BITS: usize = 256;
const MODULUS_BITS: [bool; WORD_BITS] = [
    true, true, true, true, false, true, false, false, false, false, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
];

fn emit_zero_toggle(circuit: &mut B, word: &[QubitId], target: QubitId) {
    assert_eq!(word.len(), WORD_BITS);
    for &qubit in word { circuit.x(qubit); }
    let chain = circuit.alloc_qubits(WORD_BITS - 2);
    circuit.ccx(word[0], word[1], chain[0]);
    for index in 2..WORD_BITS - 1 {
        circuit.ccx(chain[index - 2], word[index], chain[index - 1]);
    }
    circuit.ccx(chain[WORD_BITS - 3], word[WORD_BITS - 1], target);
    for index in (2..WORD_BITS - 1).rev() {
        circuit.ccx(chain[index - 2], word[index], chain[index - 1]);
    }
    circuit.ccx(word[0], word[1], chain[0]);
    circuit.free_vec(&chain);
    for &qubit in word { circuit.x(qubit); }
}

pub(super) fn emit_frontier_k2_exact_word_subtract(
    circuit: &mut B, source: &[QubitId], target: &mut Vec<QubitId>,
) {
    assert_eq!(source.len(), WORD_BITS);
    assert_eq!(target.len(), WORD_BITS);
    let source_overflow = circuit.alloc_qubit();
    let target_overflow = circuit.alloc_qubit();
    let carry_in = circuit.alloc_qubit();
    let mut source_ext = source.to_vec();
    source_ext.push(source_overflow);
    let mut target_ext = target.clone();
    target_ext.push(target_overflow);
    super::frontier_k2_controlled_pseudomersenne::emit_fast_subtract(
        circuit, &source_ext, &target_ext, carry_in,
    );
    circuit.free(carry_in);
    circuit.free(source_overflow);
    super::frontier_k2_low73_double_halve::emit_frontier_k2_low73_subtract_correction(
        circuit, &target[..73], target_overflow,
    );
    circuit.x(target_overflow);
    super::frontier_k2_controlled_pseudomersenne::emit_modular_negate_selected(circuit, source);
    super::frontier_k2_controlled_pseudomersenne::emit_unsigned_lt_toggle(
        circuit, target, source, target_overflow,
    );
    super::frontier_k2_controlled_pseudomersenne::emit_modular_negate_selected(circuit, source);
    circuit.free(target_overflow);
}

pub(super) fn emit_frontier_k2_exact_word_negate(
    circuit: &mut B, word: &mut Vec<QubitId>,
) {
    assert_eq!(word.len(), WORD_BITS);
    let was_zero = circuit.alloc_qubit();
    emit_zero_toggle(circuit, word, was_zero);
    super::frontier_k2_controlled_pseudomersenne::emit_modular_negate_selected(circuit, word);
    for (index, &set) in MODULUS_BITS.iter().enumerate() {
        if set { circuit.cx(was_zero, word[index]); }
    }
    emit_zero_toggle(circuit, word, was_zero);
    circuit.free(was_zero);
}
