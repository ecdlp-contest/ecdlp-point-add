//! Deterministically generated low-space square.
//! Edit formal IR and regenerate; generated Rust is output-only.
//! ir_sha256=2cd3b68baf0a9593e3a70ab4c615203c55d9cbd1faafe93271503cdc20308a76
//! generator_sha256=0f63e07e3dca7ae1c2c0a5232b5927c2394b4c45f7b39e00a8206797b0441a52

use super::*;

#[derive(Clone, Copy)]
enum SignedOp {
    Add,
    Sub,
}

impl SignedOp {
    fn flipped(self) -> Self {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
        }
    }
}

fn apply_operand(b: &mut B, acc: &[QubitId], operand: &[QubitId], op: SignedOp, p: U256) {
    match op {
        SignedOp::Add => mod_add_qq(b, acc, operand, p),
        SignedOp::Sub => mod_sub_qq(b, acc, operand, p),
    }
}

fn toggle_controlled_lt_const(
    b: &mut B,
    value: &[QubitId],
    control: QubitId,
    target: QubitId,
    constant: U256,
) {
    let constant_register = load_const(b, 256, constant);
    let comparison = b.alloc_qubit();
    cmp_lt_into(b, value, &constant_register, comparison);
    b.ccx(control, comparison, target);
    cmp_lt_into(b, value, &constant_register, comparison);
    b.free(comparison);
    unload_const(b, &constant_register, constant);
}

fn toggle_controlled_ge_const(
    b: &mut B,
    value: &[QubitId],
    control: QubitId,
    target: QubitId,
    constant: U256,
) {
    let constant_register = load_const(b, 256, constant);
    let comparison = b.alloc_qubit();
    cmp_lt_into(b, value, &constant_register, comparison);
    b.x(comparison);
    b.ccx(control, comparison, target);
    b.x(comparison);
    cmp_lt_into(b, value, &constant_register, comparison);
    b.free(comparison);
    unload_const(b, &constant_register, constant);
}

fn apply_controlled_singleton_mod(
    b: &mut B,
    acc: &[QubitId],
    control: QubitId,
    shift: usize,
    op: SignedOp,
    p: U256,
) {
    let term = U256::from(1) << shift;
    let threshold = p.wrapping_sub(term);
    let f = U256::MAX.wrapping_sub(p).wrapping_add(U256::from(1));
    let reduction = b.alloc_qubit();
    match op {
        SignedOp::Add => {
            toggle_controlled_ge_const(b, acc, control, reduction, threshold);
            cadd_nbit_const_direct_fast(b, acc, term, control);
            cadd_nbit_const_direct_fast(b, acc, f, reduction);
            toggle_controlled_lt_const(b, acc, control, reduction, term);
        }
        SignedOp::Sub => {
            toggle_controlled_lt_const(b, acc, control, reduction, term);
            csub_nbit_const_direct_fast(b, acc, term, control);
            csub_nbit_const_direct_fast(b, acc, f, reduction);
            toggle_controlled_ge_const(b, acc, control, reduction, threshold);
        }
    }
    b.free(reduction);
}

// Apply value*2^shift where the represented integer is strictly below p.
// A 256-bit window is split at its top bit, so no modular primitive receives
// an unproved non-canonical operand.
fn apply_fitting_shifted(
    b: &mut B,
    acc: &[QubitId],
    value: &[QubitId],
    shift: usize,
    op: SignedOp,
    p: U256,
) {
    if value.is_empty() {
        return;
    }
    assert!(value.len() + shift <= 256);
    if value.len() == 1 {
        apply_controlled_singleton_mod(b, acc, value[0], shift, op, p);
        return;
    }
    if value.len() + shift == 256 && value.len() > 1 {
        let split = value.len() - 1;
        apply_fitting_shifted(b, acc, &value[..split], shift, op, p);
        apply_fitting_shifted(b, acc, &value[split..], shift + split, op, p);
        return;
    }
    let low_padding = b.alloc_qubits(shift);
    let high_padding = b.alloc_qubits(256 - shift - value.len());
    let mut operand = Vec::with_capacity(256);
    operand.extend_from_slice(&low_padding);
    operand.extend_from_slice(value);
    operand.extend_from_slice(&high_padding);
    apply_operand(b, acc, &operand, op, p);
    b.free_vec(&high_padding);
    b.free_vec(&low_padding);
}

// F = 2^32 + 977 = 2^0 + 2^4 - 2^6 + 2^10 + 2^32.
const F_NAF: [(usize, bool); 5] = [(0, false), (4, false), (6, true), (10, false), (32, false)];

fn apply_narrow_times_f(b: &mut B, acc: &[QubitId], value: &[QubitId], op: SignedOp, p: U256) {
    assert!(value.len() + 32 <= 256);
    for (shift, negative) in F_NAF {
        let term_op = if negative { op.flipped() } else { op };
        apply_fitting_shifted(b, acc, value, shift, term_op, p);
    }
}

// Reduce one shifted exact product with 2^256 = F (mod p). The overflow
// tail is narrow for every reviewed call, so one fold is sufficient.
fn apply_shifted_product(
    b: &mut B,
    acc: &[QubitId],
    product: &[QubitId],
    shift: usize,
    op: SignedOp,
    p: U256,
) {
    assert!(shift < 256);
    let cutoff = 256 - shift;
    let low_len = product.len().min(cutoff);
    apply_fitting_shifted(b, acc, &product[..low_len], shift, op, p);
    if product.len() > cutoff {
        apply_narrow_times_f(b, acc, &product[cutoff..], op, p);
    }
}

// The reviewed caller supplies b^2 < p. Walk that canonical product through
// the NAF powers in place, then apply the exact inverse walk back to b^2.
fn apply_product_times_f(b: &mut B, acc: &[QubitId], product: &[QubitId], op: SignedOp, p: U256) {
    assert_eq!(product.len(), 256);
    apply_operand(b, acc, product, op, p);
    for _ in 0..4 {
        mod_double_inplace_direct_const_fast(b, product, p);
    }
    apply_operand(b, acc, product, op, p);
    for _ in 0..2 {
        mod_double_inplace_direct_const_fast(b, product, p);
    }
    apply_operand(b, acc, product, op.flipped(), p);
    for _ in 0..4 {
        mod_double_inplace_direct_const_fast(b, product, p);
    }
    apply_operand(b, acc, product, op, p);
    for _ in 0..22 {
        mod_double_inplace_direct_const_fast(b, product, p);
    }
    apply_operand(b, acc, product, op, p);
    for _ in 0..32 {
        mod_halve_inplace_direct_const_fast(b, product, p);
    }
}

pub(crate) fn frontier_low_space_square_subtract(
    b: &mut B,
    acc: &[QubitId],
    x: &[QubitId],
    p: U256,
) {
    assert_eq!(acc.len(), 256);
    assert_eq!(x.len(), 256);
    let a = &x[..128];
    let high = &x[128..];

    b.set_phase("frontier_square_low_product");
    let product = b.alloc_qubits(256);
    trailmix_ludicrous::square::product_register::tri_square(b, a, &product, false);
    apply_shifted_product(b, acc, &product, 0, SignedOp::Sub, p);
    trailmix_ludicrous::square::product_register::tri_square(b, a, &product, true);
    b.free_vec(&product);

    b.set_phase("frontier_square_cross_product");
    let product = b.alloc_qubits(256);
    schoolbook_mul_into_addsub_lowq(b, a, high, &product);
    apply_shifted_product(b, acc, &product, 129, SignedOp::Sub, p);
    schoolbook_mul_into_addsub_lowq_inverse(b, a, high, &product);
    b.free_vec(&product);

    b.set_phase("frontier_square_high_product");
    let product = b.alloc_qubits(256);
    trailmix_ludicrous::square::product_register::tri_square(b, high, &product, false);
    apply_product_times_f(b, acc, &product, SignedOp::Sub, p);
    trailmix_ludicrous::square::product_register::tri_square(b, high, &product, true);
    b.free_vec(&product);
}