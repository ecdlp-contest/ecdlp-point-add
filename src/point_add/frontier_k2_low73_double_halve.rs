// Generated from formal/ir/frontier-k2-low73-double-halve-v1.json.
// Generated Rust is output-only. This is approximate outside two named bad events.
// ir_sha256=96a38cfb7375eb4557c52fc6bf3d63eaa72e00caa64d4bcbf67ad116a08f8944
// generator_sha256=7d6a5dd5ac848f0b0b3368efedf7741bfe28033a68a4635454b4a9107fb8d48c

use super::B;
use crate::circuit::QubitId;

const FRONTIER_K2_WORD_BITS: usize = 256;
const FRONTIER_K2_LOW_BITS: usize = 73;
const FRONTIER_K2_CORRECTION_SET_BITS: [usize; 7] = [0, 4, 6, 7, 8, 9, 32];

fn controlled_swap(circuit: &mut B, control: QubitId, left: QubitId, right: QubitId) {
    circuit.cx(right, left);
    circuit.ccx(control, left, right);
    circuit.cx(right, left);
}

pub(super) fn emit_frontier_k2_low73_add_correction(circuit: &mut B, acc: &[QubitId], control: QubitId) {
    assert_eq!(acc.len(), FRONTIER_K2_LOW_BITS);
    let addend = circuit.alloc_qubits(FRONTIER_K2_LOW_BITS);
    for &index in &FRONTIER_K2_CORRECTION_SET_BITS { circuit.cx(control, addend[index]); }
    let carry_in = circuit.alloc_qubit();
    let carries = circuit.alloc_qubits(FRONTIER_K2_LOW_BITS - 1);

    circuit.cx(addend[0], acc[0]);
    circuit.cx(addend[0], carry_in);
    circuit.ccx(carry_in, acc[0], carries[0]);
    circuit.cx(carries[0], addend[0]);
    for i in 1..FRONTIER_K2_LOW_BITS - 1 {
        circuit.cx(addend[i], acc[i]);
        circuit.cx(addend[i], addend[i - 1]);
        circuit.ccx(addend[i - 1], acc[i], carries[i]);
        circuit.cx(carries[i], addend[i]);
    }
    circuit.cx(addend[FRONTIER_K2_LOW_BITS - 2], acc[FRONTIER_K2_LOW_BITS - 1]);
    circuit.cx(addend[FRONTIER_K2_LOW_BITS - 1], acc[FRONTIER_K2_LOW_BITS - 1]);
    for i in (1..FRONTIER_K2_LOW_BITS - 1).rev() {
        circuit.cx(carries[i], addend[i]);
        let measurement = circuit.alloc_bit();
        circuit.hmr(carries[i], measurement);
        circuit.cz_if(addend[i - 1], acc[i], measurement);
        circuit.cx(addend[i], addend[i - 1]);
        circuit.cx(addend[i - 1], acc[i]);
    }
    circuit.cx(carries[0], addend[0]);
    let measurement = circuit.alloc_bit();
    circuit.hmr(carries[0], measurement);
    circuit.cz_if(carry_in, acc[0], measurement);
    circuit.cx(addend[0], carry_in);
    circuit.cx(carry_in, acc[0]);

    circuit.free_vec(&carries);
    circuit.free(carry_in);
    for &index in &FRONTIER_K2_CORRECTION_SET_BITS { circuit.cx(control, addend[index]); }
    circuit.free_vec(&addend);
}

pub(super) fn emit_frontier_k2_low73_subtract_correction(circuit: &mut B, acc: &[QubitId], control: QubitId) {
    assert_eq!(acc.len(), FRONTIER_K2_LOW_BITS);
    let addend = circuit.alloc_qubits(FRONTIER_K2_LOW_BITS);
    for &index in &FRONTIER_K2_CORRECTION_SET_BITS { circuit.cx(control, addend[index]); }
    let carry_in = circuit.alloc_qubit();
    let carries = circuit.alloc_qubits(FRONTIER_K2_LOW_BITS - 1);

    circuit.cx(carry_in, acc[0]);
    circuit.cx(addend[0], carry_in);
    circuit.ccx(carry_in, acc[0], carries[0]);
    circuit.cx(carries[0], addend[0]);
    for i in 1..FRONTIER_K2_LOW_BITS - 1 {
        circuit.cx(addend[i - 1], acc[i]);
        circuit.cx(addend[i], addend[i - 1]);
        circuit.ccx(addend[i - 1], acc[i], carries[i]);
        circuit.cx(carries[i], addend[i]);
    }
    circuit.cx(addend[FRONTIER_K2_LOW_BITS - 1], acc[FRONTIER_K2_LOW_BITS - 1]);
    circuit.cx(addend[FRONTIER_K2_LOW_BITS - 2], acc[FRONTIER_K2_LOW_BITS - 1]);
    for i in (1..FRONTIER_K2_LOW_BITS - 1).rev() {
        circuit.cx(carries[i], addend[i]);
        let measurement = circuit.alloc_bit();
        circuit.hmr(carries[i], measurement);
        circuit.cz_if(addend[i - 1], acc[i], measurement);
        circuit.cx(addend[i], addend[i - 1]);
        circuit.cx(addend[i], acc[i]);
    }
    circuit.cx(carries[0], addend[0]);
    let measurement = circuit.alloc_bit();
    circuit.hmr(carries[0], measurement);
    circuit.cz_if(carry_in, acc[0], measurement);
    circuit.cx(addend[0], carry_in);
    circuit.cx(addend[0], acc[0]);

    circuit.free_vec(&carries);
    circuit.free(carry_in);
    for &index in &FRONTIER_K2_CORRECTION_SET_BITS { circuit.cx(control, addend[index]); }
    circuit.free_vec(&addend);
}

pub(super) fn emit_frontier_k2_low73_double(circuit: &mut B, value: &mut Vec<QubitId>) {
    assert_eq!(value.len(), FRONTIER_K2_WORD_BITS);
    let overflow = circuit.alloc_qubit();
    circuit.swap(value[FRONTIER_K2_WORD_BITS - 1], overflow);
    for i in (0..FRONTIER_K2_WORD_BITS - 1).rev() { circuit.swap(value[i], value[i + 1]); }
    emit_frontier_k2_low73_add_correction(circuit, &value[..FRONTIER_K2_LOW_BITS], overflow);
    circuit.cx(value[0], overflow);
    circuit.free(overflow);
}

pub(super) fn emit_frontier_k2_low73_halve(circuit: &mut B, value: &mut Vec<QubitId>) {
    assert_eq!(value.len(), FRONTIER_K2_WORD_BITS);
    let overflow = circuit.alloc_qubit();
    circuit.cx(value[0], overflow);
    emit_frontier_k2_low73_subtract_correction(circuit, &value[..FRONTIER_K2_LOW_BITS], overflow);
    for i in 0..FRONTIER_K2_WORD_BITS - 1 { circuit.swap(value[i], value[i + 1]); }
    circuit.swap(value[FRONTIER_K2_WORD_BITS - 1], overflow);
    circuit.free(overflow);
}

pub(super) fn emit_frontier_k2_low73_controlled_double(
    circuit: &mut B, value: &mut Vec<QubitId>, control: QubitId,
) {
    assert_eq!(value.len(), FRONTIER_K2_WORD_BITS);
    let overflow = circuit.alloc_qubit();
    controlled_swap(circuit, control, value[FRONTIER_K2_WORD_BITS - 1], overflow);
    for i in (0..FRONTIER_K2_WORD_BITS - 1).rev() {
        controlled_swap(circuit, control, value[i], value[i + 1]);
    }
    emit_frontier_k2_low73_add_correction(circuit, &value[..FRONTIER_K2_LOW_BITS], overflow);
    circuit.ccx(control, value[0], overflow);
    circuit.free(overflow);
}

pub(super) fn emit_frontier_k2_low73_controlled_halve(
    circuit: &mut B, value: &mut Vec<QubitId>, control: QubitId,
) {
    assert_eq!(value.len(), FRONTIER_K2_WORD_BITS);
    let overflow = circuit.alloc_qubit();
    circuit.ccx(control, value[0], overflow);
    emit_frontier_k2_low73_subtract_correction(circuit, &value[..FRONTIER_K2_LOW_BITS], overflow);
    for i in 0..FRONTIER_K2_WORD_BITS - 1 {
        controlled_swap(circuit, control, value[i], value[i + 1]);
    }
    controlled_swap(circuit, control, value[FRONTIER_K2_WORD_BITS - 1], overflow);
    circuit.free(overflow);
}
