// Generated from formal/ir/frontier-k2-controlled-pseudomersenne-add-sub-v1.json.
// Generated Rust is output-only. The forward value is approximate only at its named bad event.
// ir_sha256=7148b9507b0e0c60ff09e22d77a255850ab4a47234e3b3d666594b0947233450
// low73_rust_sha256=196d364e510606a8c157073d28cd71d6f064ee9be720f8ba9349466258729e00
// generator_sha256=572dbe255971d4fd3776ecff420555b33eea9e68aea6a1922ec2817d2d6978a9

use super::B;
use crate::circuit::QubitId;

const FRONTIER_K2_FIELD_BITS: usize = 256;
const FRONTIER_K2_P_PLUS_ONE_BITS: [bool; FRONTIER_K2_FIELD_BITS] = [
    false, false, false, false, true, true, false, false, false, false, true, true, true, true, true, true,
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

fn emit_fast_add(circuit: &mut B, addend: &[QubitId], acc: &[QubitId], carry_in: QubitId) {
    let n = addend.len();
    assert_eq!(n, acc.len());
    assert!(n >= 2);
    let carries = circuit.alloc_qubits(n - 1);
    circuit.cx(addend[0], acc[0]);
    circuit.cx(addend[0], carry_in);
    circuit.ccx(carry_in, acc[0], carries[0]);
    circuit.cx(carries[0], addend[0]);
    for i in 1..n - 1 {
        circuit.cx(addend[i], acc[i]);
        circuit.cx(addend[i], addend[i - 1]);
        circuit.ccx(addend[i - 1], acc[i], carries[i]);
        circuit.cx(carries[i], addend[i]);
    }
    circuit.cx(addend[n - 2], acc[n - 1]);
    circuit.cx(addend[n - 1], acc[n - 1]);
    for i in (1..n - 1).rev() {
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
}

pub(super) fn emit_fast_subtract(circuit: &mut B, subtrahend: &[QubitId], acc: &[QubitId], carry_in: QubitId) {
    let n = subtrahend.len();
    assert_eq!(n, acc.len());
    assert!(n >= 2);
    let carries = circuit.alloc_qubits(n - 1);
    circuit.cx(carry_in, acc[0]);
    circuit.cx(subtrahend[0], carry_in);
    circuit.ccx(carry_in, acc[0], carries[0]);
    circuit.cx(carries[0], subtrahend[0]);
    for i in 1..n - 1 {
        circuit.cx(subtrahend[i - 1], acc[i]);
        circuit.cx(subtrahend[i], subtrahend[i - 1]);
        circuit.ccx(subtrahend[i - 1], acc[i], carries[i]);
        circuit.cx(carries[i], subtrahend[i]);
    }
    circuit.cx(subtrahend[n - 1], acc[n - 1]);
    circuit.cx(subtrahend[n - 2], acc[n - 1]);
    for i in (1..n - 1).rev() {
        circuit.cx(carries[i], subtrahend[i]);
        let measurement = circuit.alloc_bit();
        circuit.hmr(carries[i], measurement);
        circuit.cz_if(subtrahend[i - 1], acc[i], measurement);
        circuit.cx(subtrahend[i], subtrahend[i - 1]);
        circuit.cx(subtrahend[i], acc[i]);
    }
    circuit.cx(carries[0], subtrahend[0]);
    let measurement = circuit.alloc_bit();
    circuit.hmr(carries[0], measurement);
    circuit.cz_if(carry_in, acc[0], measurement);
    circuit.cx(subtrahend[0], carry_in);
    circuit.cx(subtrahend[0], acc[0]);
    circuit.free_vec(&carries);
}

fn emit_maj(circuit: &mut B, x: QubitId, y: QubitId, carry: QubitId) {
    circuit.cx(carry, y);
    circuit.cx(carry, x);
    circuit.ccx(x, y, carry);
}

fn emit_inverse_maj(circuit: &mut B, x: QubitId, y: QubitId, carry: QubitId) {
    circuit.ccx(x, y, carry);
    circuit.cx(carry, x);
    circuit.cx(carry, y);
}

pub(super) fn emit_unsigned_lt_toggle(circuit: &mut B, lhs: &[QubitId], rhs: &[QubitId], target: QubitId) {
    assert_eq!(lhs.len(), FRONTIER_K2_FIELD_BITS);
    assert_eq!(rhs.len(), FRONTIER_K2_FIELD_BITS);
    let carry_in = circuit.alloc_qubit();
    for &qubit in lhs { circuit.x(qubit); }
    emit_maj(circuit, carry_in, rhs[0], lhs[0]);
    for i in 1..FRONTIER_K2_FIELD_BITS { emit_maj(circuit, lhs[i - 1], rhs[i], lhs[i]); }
    circuit.cx(lhs[FRONTIER_K2_FIELD_BITS - 1], target);
    for i in (1..FRONTIER_K2_FIELD_BITS).rev() { emit_inverse_maj(circuit, lhs[i - 1], rhs[i], lhs[i]); }
    emit_inverse_maj(circuit, carry_in, rhs[0], lhs[0]);
    for &qubit in lhs { circuit.x(qubit); }
    circuit.free(carry_in);
}

fn emit_add_p_plus_one_direct(circuit: &mut B, acc: &[QubitId]) {
    assert_eq!(acc.len(), FRONTIER_K2_FIELD_BITS);
    let known_one = circuit.alloc_qubit();
    circuit.x(known_one);
    let carries = circuit.alloc_qubits(FRONTIER_K2_FIELD_BITS - 1);
    if FRONTIER_K2_P_PLUS_ONE_BITS[0] { circuit.ccx(acc[0], known_one, carries[0]); }
    for i in 1..FRONTIER_K2_FIELD_BITS - 1 {
        if FRONTIER_K2_P_PLUS_ONE_BITS[i] {
            circuit.cx(carries[i - 1], carries[i]);
            circuit.cx(carries[i - 1], acc[i]);
            circuit.cx(carries[i - 1], known_one);
            circuit.ccx(acc[i], known_one, carries[i]);
            circuit.cx(carries[i - 1], known_one);
            circuit.cx(carries[i - 1], acc[i]);
        } else {
            circuit.ccx(acc[i], carries[i - 1], carries[i]);
        }
    }
    for i in 0..FRONTIER_K2_FIELD_BITS {
        if FRONTIER_K2_P_PLUS_ONE_BITS[i] { circuit.cx(known_one, acc[i]); }
        if i > 0 { circuit.cx(carries[i - 1], acc[i]); }
    }
    for i in (0..FRONTIER_K2_FIELD_BITS - 1).rev() {
        let measurement = circuit.alloc_bit();
        circuit.hmr(carries[i], measurement);
        if FRONTIER_K2_P_PLUS_ONE_BITS[i] {
            circuit.x(acc[i]);
            circuit.cz_if(acc[i], known_one, measurement);
            if i > 0 {
                circuit.cz_if(acc[i], carries[i - 1], measurement);
                circuit.x(acc[i]);
                circuit.cz_if(known_one, carries[i - 1], measurement);
            } else {
                circuit.x(acc[i]);
            }
        } else if i > 0 {
            circuit.x(acc[i]);
            circuit.cz_if(acc[i], carries[i - 1], measurement);
            circuit.x(acc[i]);
        }
    }
    circuit.free_vec(&carries);
    circuit.x(known_one);
    circuit.free(known_one);
}

pub(super) fn emit_modular_negate_selected(circuit: &mut B, selected: &[QubitId]) {
    for &qubit in selected { circuit.x(qubit); }
    emit_add_p_plus_one_direct(circuit, selected);
}

fn emit_selected_source(circuit: &mut B, control: QubitId, source: &[QubitId]) -> Vec<QubitId> {
    let selected = circuit.alloc_qubits(FRONTIER_K2_FIELD_BITS);
    for i in 0..FRONTIER_K2_FIELD_BITS { circuit.ccx(control, source[i], selected[i]); }
    selected
}

fn emit_measurement_unload_selected(circuit: &mut B, control: QubitId, source: &[QubitId], selected: &[QubitId]) {
    for i in 0..FRONTIER_K2_FIELD_BITS {
        let measurement = circuit.alloc_bit();
        circuit.hmr(selected[i], measurement);
        circuit.cz_if(control, source[i], measurement);
    }
    circuit.free_vec(selected);
}

pub(super) fn emit_frontier_k2_controlled_pseudomersenne_add(
    circuit: &mut B, control: QubitId, source: &[QubitId], target: &mut Vec<QubitId>,
) {
    assert_eq!(source.len(), FRONTIER_K2_FIELD_BITS);
    assert_eq!(target.len(), FRONTIER_K2_FIELD_BITS);
    let selected = emit_selected_source(circuit, control, source);
    let selected_overflow = circuit.alloc_qubit();
    let target_overflow = circuit.alloc_qubit();
    let carry_in = circuit.alloc_qubit();
    let mut selected_ext = selected.clone();
    selected_ext.push(selected_overflow);
    let mut target_ext = target.clone();
    target_ext.push(target_overflow);
    emit_fast_add(circuit, &selected_ext, &target_ext, carry_in);
    circuit.free(carry_in);
    circuit.free(selected_overflow);
    super::frontier_k2_low73_double_halve::emit_frontier_k2_low73_add_correction(
        circuit, &target[..73], target_overflow,
    );
    emit_unsigned_lt_toggle(circuit, target, &selected, target_overflow);
    circuit.free(target_overflow);
    emit_measurement_unload_selected(circuit, control, source, &selected);
}

pub(super) fn emit_frontier_k2_controlled_pseudomersenne_subtract(
    circuit: &mut B, control: QubitId, source: &[QubitId], target: &mut Vec<QubitId>,
) {
    assert_eq!(source.len(), FRONTIER_K2_FIELD_BITS);
    assert_eq!(target.len(), FRONTIER_K2_FIELD_BITS);
    let selected = emit_selected_source(circuit, control, source);
    let selected_overflow = circuit.alloc_qubit();
    let target_overflow = circuit.alloc_qubit();
    let carry_in = circuit.alloc_qubit();
    let mut selected_ext = selected.clone();
    selected_ext.push(selected_overflow);
    let mut target_ext = target.clone();
    target_ext.push(target_overflow);
    emit_fast_subtract(circuit, &selected_ext, &target_ext, carry_in);
    circuit.free(carry_in);
    circuit.free(selected_overflow);
    super::frontier_k2_low73_double_halve::emit_frontier_k2_low73_subtract_correction(
        circuit, &target[..73], target_overflow,
    );
    circuit.x(target_overflow);
    emit_modular_negate_selected(circuit, &selected);
    emit_unsigned_lt_toggle(circuit, target, &selected, target_overflow);
    emit_modular_negate_selected(circuit, &selected);
    circuit.free(target_overflow);
    emit_measurement_unload_selected(circuit, control, source, &selected);
}
