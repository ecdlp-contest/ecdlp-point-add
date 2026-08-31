// Generated from formal/ir/frontier-native265-coordinate-three-x-v1.json.
// Generated Rust is output-only.
// ir_sha256=5aa28168b1b24f0bd233cf07ff62fbae82242b66c4b5003ebdacff9e032eddb8
// exact_word_rust_sha256=5cf01093d4530a75e8e2610964d0be412f370ba34a5f8be51bcd7821722dfdf2
// generator_sha256=0c829597c1ffd86db134f0ea97966d9b663848a73f93bcc006637061c38f7af8

use super::{B, BExt};
use crate::circuit::{BitId, QubitId};

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

fn fresh_zero_bits(circuit: &mut B, count: usize) -> Vec<BitId> {
    let bits = circuit.alloc_bits(count);
    for &bit in &bits { circuit.bit_store0(bit); }
    bits
}

fn clear_bits(circuit: &mut B, bits: &[BitId]) {
    for &bit in bits { circuit.bit_store0(bit); }
}

fn xor_if(circuit: &mut B, target: BitId, condition: BitId) {
    circuit.push_condition(condition);
    circuit.bit_invert(target);
    circuit.pop_condition();
}

fn xor_if_two(circuit: &mut B, target: BitId, first: BitId, second: BitId) {
    circuit.push_condition(first);
    circuit.push_condition(second);
    circuit.bit_invert(target);
    circuit.pop_condition();
    circuit.pop_condition();
}

fn classical_add(circuit: &mut B, left: &[BitId], right: &[BitId], width: usize) -> Vec<BitId> {
    assert!(left.len() <= width && right.len() <= width);
    let output = fresh_zero_bits(circuit, width);
    let carry = fresh_zero_bits(circuit, width + 1);
    for index in 0..width {
        if index < left.len() { xor_if(circuit, output[index], left[index]); }
        if index < right.len() { xor_if(circuit, output[index], right[index]); }
        xor_if(circuit, output[index], carry[index]);
        if index < left.len() && index < right.len() {
            xor_if_two(circuit, carry[index + 1], left[index], right[index]);
        }
        if index < left.len() { xor_if_two(circuit, carry[index + 1], left[index], carry[index]); }
        if index < right.len() { xor_if_two(circuit, carry[index + 1], right[index], carry[index]); }
    }
    clear_bits(circuit, &carry);
    output
}

fn subtract_modulus(circuit: &mut B, input: &[BitId]) -> (Vec<BitId>, Vec<BitId>) {
    let width = input.len();
    assert!(width >= WORD_BITS);
    let difference = fresh_zero_bits(circuit, width);
    let borrow = fresh_zero_bits(circuit, width + 1);
    for index in 0..width {
        xor_if(circuit, difference[index], input[index]);
        let modulus_bit = index < WORD_BITS && MODULUS_BITS[index];
        if modulus_bit { circuit.bit_invert(difference[index]); }
        xor_if(circuit, difference[index], borrow[index]);
        if modulus_bit {
            circuit.bit_store1(borrow[index + 1]);
            xor_if(circuit, borrow[index + 1], input[index]);
            xor_if_two(circuit, borrow[index + 1], input[index], borrow[index]);
        } else {
            xor_if(circuit, borrow[index + 1], borrow[index]);
            xor_if_two(circuit, borrow[index + 1], input[index], borrow[index]);
        }
    }
    (difference, borrow)
}

fn reduce_once(circuit: &mut B, input: Vec<BitId>) -> Vec<BitId> {
    let width = input.len();
    let (difference, borrow) = subtract_modulus(circuit, &input);
    let output = fresh_zero_bits(circuit, width);
    for index in 0..width { xor_if(circuit, output[index], input[index]); }
    let select_difference = borrow[width];
    circuit.bit_invert(select_difference);
    circuit.push_condition(select_difference);
    for index in 0..width {
        xor_if(circuit, output[index], input[index]);
        xor_if(circuit, output[index], difference[index]);
    }
    circuit.pop_condition();
    circuit.bit_invert(select_difference);
    clear_bits(circuit, &input);
    clear_bits(circuit, &difference);
    clear_bits(circuit, &borrow);
    output
}

fn modulus_minus_word(circuit: &mut B, input: &[BitId]) -> (Vec<BitId>, Vec<BitId>) {
    assert_eq!(input.len(), WORD_BITS);
    let difference = fresh_zero_bits(circuit, WORD_BITS);
    let borrow = fresh_zero_bits(circuit, WORD_BITS + 1);
    for index in 0..WORD_BITS {
        if MODULUS_BITS[index] { circuit.bit_invert(difference[index]); }
        xor_if(circuit, difference[index], input[index]);
        xor_if(circuit, difference[index], borrow[index]);
        if MODULUS_BITS[index] {
            xor_if_two(circuit, borrow[index + 1], input[index], borrow[index]);
        } else {
            xor_if(circuit, borrow[index + 1], input[index]);
            xor_if(circuit, borrow[index + 1], borrow[index]);
            xor_if_two(circuit, borrow[index + 1], input[index], borrow[index]);
        }
    }
    (difference, borrow)
}

fn canonical_inverse(circuit: &mut B, input: Vec<BitId>) -> Vec<BitId> {
    assert_eq!(input.len(), WORD_BITS);
    let (difference, borrow) = modulus_minus_word(circuit, &input);
    let nonzero = fresh_zero_bits(circuit, WORD_BITS + 1);
    for index in 0..WORD_BITS {
        xor_if(circuit, nonzero[index + 1], nonzero[index]);
        xor_if(circuit, nonzero[index + 1], input[index]);
        xor_if_two(circuit, nonzero[index + 1], nonzero[index], input[index]);
    }
    let output = fresh_zero_bits(circuit, WORD_BITS);
    circuit.push_condition(nonzero[WORD_BITS]);
    for index in 0..WORD_BITS { xor_if(circuit, output[index], difference[index]); }
    circuit.pop_condition();
    clear_bits(circuit, &input);
    clear_bits(circuit, &difference);
    clear_bits(circuit, &borrow);
    clear_bits(circuit, &nonzero);
    output
}

pub(super) fn compute_frontier_native265_three_x_inverse_bits(
    circuit: &mut B, coordinate: &[BitId],
) -> Vec<BitId> {
    assert_eq!(coordinate.len(), WORD_BITS);
    let doubled = classical_add(circuit, coordinate, coordinate, WORD_BITS + 1);
    let tripled = classical_add(circuit, &doubled, coordinate, WORD_BITS + 2);
    clear_bits(circuit, &doubled);
    let reduced_once = reduce_once(circuit, tripled);
    let reduced_twice = reduce_once(circuit, reduced_once);
    assert_eq!(reduced_twice.len(), WORD_BITS + 2);
    let residue = reduced_twice[..WORD_BITS].to_vec();
    clear_bits(circuit, &reduced_twice[WORD_BITS..]);
    canonical_inverse(circuit, residue)
}

pub(super) fn emit_frontier_native265_coordinate_three_x(
    circuit: &mut B, destination: &mut Vec<QubitId>, coordinate: &[BitId],
) {
    assert_eq!(destination.len(), WORD_BITS);
    let inverse = compute_frontier_native265_three_x_inverse_bits(circuit, coordinate);
    let loaded = circuit.alloc_qubits(WORD_BITS);
    for index in 0..WORD_BITS { circuit.x_if_bit(loaded[index], inverse[index]); }
    super::frontier_k2_exact_word_subtract_negate::emit_frontier_k2_exact_word_subtract(
        circuit, &loaded, destination,
    );
    for index in 0..WORD_BITS { circuit.x_if_bit(loaded[index], inverse[index]); }
    for owner in loaded { circuit.zero_and_free(owner); }
    clear_bits(circuit, &inverse);
}
