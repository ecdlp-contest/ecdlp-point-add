// Generated from formal/ir/frontier-native270-rowwise-margin-k2-row-v453.json.
// Generated Rust is output-only. Finite-width approximation events remain explicit.
// ir_sha256=e2ff6925112cbb0f6f28d977f3f03649d52e5e4454ed9d3c765464667034bd82
// generator_sha256=de6a385cd877e85d2ec858fc815810b141b13bb05197685cc65236b3decadca7

use super::B;
use crate::circuit::QubitId;

const ACTIVE_WIDTHS: [usize; 270] = [
    256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
    256, 256, 256, 256, 256, 256, 256, 256, 254, 254, 252, 252, 250, 250, 248, 248,
    246, 246, 244, 244, 242, 242, 240, 240, 238, 238, 236, 236, 234, 234, 232, 232,
    230, 230, 228, 228, 226, 226, 224, 224, 222, 222, 220, 218, 218, 216, 216, 214,
    214, 212, 212, 210, 210, 208, 208, 206, 206, 204, 204, 202, 202, 200, 200, 198,
    198, 196, 196, 194, 194, 192, 192, 190, 190, 188, 188, 186, 186, 184, 184, 182,
    182, 180, 180, 178, 178, 176, 176, 174, 174, 172, 172, 170, 170, 168, 168, 166,
    166, 164, 164, 162, 162, 160, 158, 158, 156, 156, 154, 154, 153, 153, 151, 150,
    148, 148, 146, 146, 144, 144, 142, 142, 140, 140, 138, 138, 136, 136, 134, 134,
    132, 132, 130, 130, 128, 128, 126, 126, 124, 124, 122, 122, 120, 120, 118, 118,
    116, 116, 114, 114, 112, 112, 110, 110, 108, 108, 106, 106, 104, 104, 104, 102,
    100, 99, 98, 96, 96, 94, 94, 92, 92, 92, 90, 88, 88, 87, 87, 86,
    84, 84, 83, 83, 82, 80, 78, 76, 76, 75, 75, 75, 74, 73, 72, 72,
    70, 70, 68, 66, 66, 64, 63, 62, 62, 60, 58, 58, 56, 56, 54, 53,
    53, 52, 50, 48, 48, 48, 46, 45, 44, 44, 42, 41, 40, 38, 37, 36,
    36, 34, 33, 32, 30, 30, 29, 28, 28, 27, 26, 24, 23, 22, 21, 20,
    18, 18, 16, 16, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
];
const BODY_WIDTHS: [usize; 270] = [
    256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
    256, 256, 256, 256, 256, 256, 256, 256, 254, 254, 252, 252, 250, 250, 248, 248,
    246, 246, 244, 244, 242, 242, 240, 240, 238, 238, 236, 236, 234, 234, 232, 232,
    230, 230, 228, 228, 226, 226, 224, 224, 222, 222, 220, 218, 218, 216, 216, 214,
    214, 212, 212, 210, 210, 208, 208, 206, 206, 204, 204, 202, 202, 200, 200, 198,
    198, 196, 196, 194, 194, 192, 192, 190, 190, 188, 188, 186, 186, 184, 184, 182,
    182, 180, 180, 178, 178, 176, 176, 174, 174, 172, 172, 170, 170, 168, 168, 166,
    166, 164, 164, 162, 162, 160, 158, 158, 156, 156, 154, 154, 153, 153, 151, 150,
    148, 148, 146, 146, 144, 144, 142, 142, 140, 140, 138, 138, 136, 136, 134, 134,
    132, 132, 130, 130, 128, 128, 126, 126, 124, 124, 122, 122, 120, 120, 118, 118,
    116, 116, 114, 114, 112, 112, 110, 110, 108, 108, 106, 106, 104, 104, 104, 102,
    100, 99, 98, 96, 96, 94, 94, 92, 92, 92, 90, 88, 88, 87, 87, 86,
    84, 84, 83, 83, 82, 80, 78, 76, 76, 75, 75, 75, 74, 73, 72, 72,
    70, 70, 68, 66, 66, 64, 63, 62, 62, 60, 58, 58, 56, 56, 54, 53,
    53, 52, 50, 48, 48, 48, 46, 45, 44, 44, 42, 41, 40, 38, 37, 36,
    36, 34, 33, 32, 30, 30, 29, 28, 28, 27, 26, 24, 23, 22, 21, 20,
    18, 18, 16, 16, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
];
const COMPARE_WIDTHS: [usize; 270] = [
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 153, 153, 151, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 87, 87, 86,
    84, 84, 83, 83, 82, 80, 78, 76, 76, 75, 75, 75, 74, 73, 72, 72,
    70, 70, 68, 66, 66, 64, 63, 62, 62, 60, 58, 58, 56, 56, 54, 53,
    53, 52, 50, 48, 48, 48, 46, 45, 44, 44, 42, 41, 40, 38, 37, 36,
    36, 34, 33, 32, 30, 30, 29, 28, 28, 27, 26, 24, 23, 22, 21, 20,
    18, 18, 16, 16, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
];

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

fn emit_guarded_lt(
    circuit: &mut B, lhs: &[QubitId], rhs: &[QubitId],
    guard: QubitId, target: QubitId,
) {
    assert_eq!(lhs.len(), rhs.len());
    assert!(!lhs.is_empty());
    let carry_in = circuit.alloc_qubit();
    for &qubit in lhs { circuit.x(qubit); }
    emit_maj(circuit, carry_in, rhs[0], lhs[0]);
    for index in 1..lhs.len() {
        emit_maj(circuit, lhs[index - 1], rhs[index], lhs[index]);
    }
    circuit.ccx(guard, lhs[lhs.len() - 1], target);
    for index in (1..lhs.len()).rev() {
        emit_inverse_maj(circuit, lhs[index - 1], rhs[index], lhs[index]);
    }
    emit_inverse_maj(circuit, carry_in, rhs[0], lhs[0]);
    for &qubit in lhs { circuit.x(qubit); }
    circuit.free(carry_in);
}

fn emit_controlled_swap_prefix(
    circuit: &mut B, control: QubitId, lhs: &[QubitId], rhs: &[QubitId], width: usize,
) {
    assert!(width <= lhs.len() && width <= rhs.len());
    // Both low bits are one on the guarded branch, so swapping index zero is identity.
    for index in 1..width {
        circuit.cx(rhs[index], lhs[index]);
        circuit.ccx(control, lhs[index], rhs[index]);
        circuit.cx(rhs[index], lhs[index]);
    }
}

fn emit_vented_controlled_add(
    circuit: &mut B, addend: &[QubitId], acc: &[QubitId],
    control: QubitId, vents: &[QubitId],
) {
    let n = addend.len();
    assert_eq!(n, acc.len());
    if n == 0 { return; }
    if n == 1 { circuit.ccx(control, addend[0], acc[0]); return; }
    assert!(vents.len() >= n - 1, "low-owner K2 row vent width");
    for index in 1..n { circuit.cx(addend[index], acc[index]); }
    for index in (1..n - 1).rev() { circuit.cx(addend[index], addend[index + 1]); }
    for index in 0..n - 1 {
        circuit.ccx(acc[index], addend[index], vents[index]);
        circuit.cx(vents[index], addend[index + 1]);
    }
    for index in (0..n - 1).rev() {
        circuit.ccx(control, addend[index + 1], acc[index + 1]);
        circuit.cx(vents[index], addend[index + 1]);
        let measurement = circuit.alloc_bit();
        circuit.hmr(vents[index], measurement);
        circuit.cz_if(acc[index], addend[index], measurement);
    }
    for index in 1..n - 1 { circuit.cx(addend[index], addend[index + 1]); }
    circuit.ccx(control, addend[0], acc[0]);
    for index in 1..n { circuit.cx(addend[index], acc[index]); }
}

fn emit_vented_controlled_subtract(
    circuit: &mut B, subtrahend: &[QubitId], acc: &[QubitId],
    control: QubitId, vents: &[QubitId],
) {
    for &qubit in acc { circuit.x(qubit); }
    emit_vented_controlled_add(circuit, subtrahend, acc, control, vents);
    for &qubit in acc { circuit.x(qubit); }
}

fn emit_shift_right(circuit: &mut B, word: &[QubitId], width: usize) {
    for index in 0..width - 1 { circuit.swap(word[index], word[index + 1]); }
}

fn emit_unshift_right(circuit: &mut B, word: &[QubitId], width: usize) {
    for index in (0..width - 1).rev() { circuit.swap(word[index], word[index + 1]); }
}

fn emit_controlled_shift_right(
    circuit: &mut B, control: QubitId, word: &[QubitId], width: usize,
) {
    for index in 0..width - 1 {
        circuit.cx(word[index + 1], word[index]);
        circuit.ccx(control, word[index], word[index + 1]);
        circuit.cx(word[index + 1], word[index]);
    }
}

fn emit_controlled_unshift_right(
    circuit: &mut B, control: QubitId, word: &[QubitId], width: usize,
) {
    for index in (0..width - 1).rev() {
        circuit.cx(word[index + 1], word[index]);
        circuit.ccx(control, word[index], word[index + 1]);
        circuit.cx(word[index + 1], word[index]);
    }
}

pub(super) struct FrontierNative265LowOwnerK2Row;

impl FrontierNative265LowOwnerK2Row {
    pub(super) fn active_width(step: usize) -> usize { ACTIVE_WIDTHS[step] }
    pub(super) fn body_width(step: usize) -> usize { BODY_WIDTHS[step] }
    pub(super) fn vent_count(step: usize) -> usize { BODY_WIDTHS[step].saturating_sub(2) }

    pub(super) fn emit_forward_row(
        circuit: &mut B, step: usize, u: &mut Vec<QubitId>, v: &mut Vec<QubitId>,
        raw: &mut [QubitId; 15], slot: usize, vents: &[QubitId],
    ) {
        assert!(step < 270 && slot < 5 && u.len() == 256 && v.len() == 256);
        let active = ACTIVE_WIDTHS[step];
        let body = BODY_WIDTHS[step];
        let compare = COMPARE_WIDTHS[step];
        assert!(vents.len() >= body.saturating_sub(2));
        let b0 = raw[3 * slot];
        let branch = raw[3 * slot + 1];
        let s2 = raw[3 * slot + 2];
        circuit.cx(v[0], b0);
        let compare_start = active - compare;
        emit_guarded_lt(
            circuit, &v[compare_start..active], &u[compare_start..active], b0, branch,
        );
        emit_controlled_swap_prefix(circuit, branch, u, v, active);
        circuit.cx(b0, v[0]);
        emit_vented_controlled_subtract(
            circuit, &u[1..body], &v[1..body], b0, &vents[..body.saturating_sub(2)],
        );
        emit_shift_right(circuit, v, active);
        circuit.cx(v[0], s2);
        circuit.x(s2);
        emit_controlled_shift_right(circuit, s2, v, active);
    }

    pub(super) fn emit_reverse_row(
        circuit: &mut B, step: usize, u: &mut Vec<QubitId>, v: &mut Vec<QubitId>,
        raw: &mut [QubitId; 15], slot: usize, vents: &[QubitId],
    ) {
        assert!(step < 270 && slot < 5 && u.len() == 256 && v.len() == 256);
        let active = ACTIVE_WIDTHS[step];
        let body = BODY_WIDTHS[step];
        let compare = COMPARE_WIDTHS[step];
        assert!(vents.len() >= body.saturating_sub(2));
        let b0 = raw[3 * slot];
        let branch = raw[3 * slot + 1];
        let s2 = raw[3 * slot + 2];
        emit_controlled_unshift_right(circuit, s2, v, active);
        circuit.x(s2);
        circuit.cx(v[0], s2);
        emit_unshift_right(circuit, v, active);
        emit_vented_controlled_add(
            circuit, &u[1..body], &v[1..body], b0, &vents[..body.saturating_sub(2)],
        );
        circuit.cx(b0, v[0]);
        emit_controlled_swap_prefix(circuit, branch, u, v, active);
        let compare_start = active - compare;
        emit_guarded_lt(
            circuit, &v[compare_start..active], &u[compare_start..active], b0, branch,
        );
        circuit.cx(v[0], b0);
    }
}

impl super::frontier_native265_source_materializer_lifecycle::FrontierNative265K2WalkCells
    for FrontierNative265LowOwnerK2Row
{
    fn forward_row(
        &mut self, circuit: &mut B, step: usize, u: &mut Vec<QubitId>,
        factor: &mut Vec<QubitId>, raw: &mut [QubitId; 15], slot: usize,
        vents: &[QubitId],
    ) { Self::emit_forward_row(circuit, step, u, factor, raw, slot, vents); }

    fn reverse_row(
        &mut self, circuit: &mut B, step: usize, u: &mut Vec<QubitId>,
        factor: &mut Vec<QubitId>, raw: &mut [QubitId; 15], slot: usize,
        vents: &[QubitId],
    ) { Self::emit_reverse_row(circuit, step, u, factor, raw, slot, vents); }
}
