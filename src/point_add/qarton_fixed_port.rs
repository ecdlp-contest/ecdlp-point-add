//! Operation-preserving port of Schrottenloher's Qarton fixed-point circuit.
//!
//! Source: ec-point-addition commit 9b23c9170a636a7097a02afb3a3d6cbb6425c9f4.
//! The source circuit has a one-qubit lookup selector followed by a 513-qubit
//! affine point (x, y, not_infinity). It is not the contest's arbitrary-Q ABI;
//! callers must not present it as such.

use super::B;
use crate::circuit::{BitId, Op, OperationType, QubitId};

const COMPRESSED_STREAM: &[u8] = include_bytes!("qarton_fixed_v1.qprt.zst");
pub(super) const SOURCE_STREAM_SHA256: &str =
    "5a2451bc0cc1f56143d860a39097b8bdd48bfb584fa70b414b635e66531d8e51";
pub(super) const LOWERED_RECORD_SHA256: &str =
    "ceaeaa8bd4065500818ad3995cd1173d514af8051fe05f10b57ddfa36e03826c";
pub(super) const QUBITS: usize = 1443;
pub(super) const CLASSICAL_BITS: usize = 256;
pub(super) const LOWERED_GATES: usize = 10_515_375;

#[derive(Clone, Copy)]
struct Header {
    kind: OperationType,
    quantum_arity: usize,
    controls: usize,
}

fn read_u16(data: &[u8], cursor: &mut usize) -> u16 {
    let end = *cursor + 2;
    let bytes: [u8; 2] = data
        .get(*cursor..end)
        .expect("truncated Qarton replay record")
        .try_into()
        .unwrap();
    *cursor = end;
    u16::from_le_bytes(bytes)
}

fn decode_header(byte: u8) -> Header {
    let controls = usize::from(byte >> 6);
    let (kind, quantum_arity) = match byte & 0x3f {
        1 => (OperationType::X, 1),
        2 => (OperationType::Z, 1),
        3 => (OperationType::CX, 2),
        4 => (OperationType::CZ, 2),
        5 => (OperationType::CCX, 3),
        6 => (OperationType::Swap, 2),
        // HMR stores one qubit and one classical result in its two target slots.
        7 => (OperationType::Hmr, 2),
        opcode => panic!("unknown Qarton replay opcode {opcode}"),
    };
    assert!(controls <= 2, "unsupported Qarton condition arity");
    Header { kind, quantum_arity, controls }
}

fn append_gate(b: &mut B, kind: OperationType, targets: &[u16], controls: &[u16]) {
    let mut op = Op::empty();
    op.kind = kind;
    match kind {
        OperationType::X | OperationType::Z => op.q_target = QubitId(u64::from(targets[0])),
        OperationType::CX | OperationType::CZ | OperationType::Swap => {
            op.q_control1 = QubitId(u64::from(targets[0]));
            op.q_target = QubitId(u64::from(targets[1]));
        }
        OperationType::CCX => {
            op.q_control2 = QubitId(u64::from(targets[0]));
            op.q_control1 = QubitId(u64::from(targets[1]));
            op.q_target = QubitId(u64::from(targets[2]));
        }
        OperationType::Hmr => {
            op.q_target = QubitId(u64::from(targets[0]));
            op.c_target = BitId(u64::from(targets[1]));
        }
        _ => unreachable!(),
    }
    if let Some(&condition) = controls.last() {
        op.c_condition = BitId(u64::from(condition));
    }
    if controls.len() == 2 {
        b.push_condition(BitId(u64::from(controls[0])));
    }
    op.validate();
    b.ops.push(op);
    if controls.len() == 2 {
        b.pop_condition();
    }
}

/// Build the exact fixed-point Qarton primitive after semantics-preserving gate
/// lowering. This intentionally remains separate from `build()`: it has the paper
/// signature, not the four-register point-add contest signature.
pub(super) fn build_fixed_point_port() -> Vec<Op> {
    build_replay(
        COMPRESSED_STREAM,
        QUBITS,
        CLASSICAL_BITS,
        LOWERED_GATES,
        &[(0, 1), (1, 514)],
        &[(0, CLASSICAL_BITS)],
    )
}

pub(super) fn build_replay(
    compressed_stream: &[u8],
    expected_qubits: usize,
    expected_cbits: usize,
    expected_gates: usize,
    quantum_registers: &[(usize, usize)],
    classical_registers: &[(usize, usize)],
) -> Vec<Op> {
    let stream = zstd::stream::decode_all(compressed_stream)
        .expect("embedded Qarton replay must be valid zstd");
    let stream = stream.as_slice();
    assert_eq!(&stream[..6], b"QPRT1\0");
    let mut cursor = 6;
    let qubits = usize::from(read_u16(stream, &mut cursor));
    let cbits = usize::from(read_u16(stream, &mut cursor));
    let count = u64::from_le_bytes(stream[cursor..cursor + 8].try_into().unwrap()) as usize;
    cursor += 8;
    assert_eq!((qubits, cbits, count), (expected_qubits, expected_cbits, expected_gates));

    let mut b = B::new();
    let q = b.alloc_qubits(qubits);
    let c = b.alloc_bits(cbits);
    for &(start, end) in quantum_registers {
        b.declare_qubit_register(&q[start..end]);
    }
    for &(start, end) in classical_registers {
        b.declare_bit_register(&c[start..end]);
    }

    for _ in 0..count {
        let header = decode_header(*stream.get(cursor).expect("truncated Qarton replay"));
        cursor += 1;
        let mut targets = [0_u16; 3];
        for target in targets.iter_mut().take(header.quantum_arity) {
            *target = read_u16(stream, &mut cursor);
        }
        let mut controls = [0_u16; 2];
        for control in controls.iter_mut().take(header.controls) {
            *control = read_u16(stream, &mut cursor);
        }
        append_gate(
            &mut b,
            header.kind,
            &targets[..header.quantum_arity],
            &controls[..header.controls],
        );
    }
    assert_eq!(cursor, stream.len(), "trailing bytes in Qarton replay");
    assert_eq!(b.peak_qubits as usize, expected_qubits);
    b.ops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_port_decodes_and_validates() {
        let ops = build_fixed_point_port();
        let mut counts = std::collections::BTreeMap::new();
        for op in &ops {
            op.validate();
            *counts.entry(op.kind as u8).or_insert(0_usize) += 1;
        }
        assert_eq!(counts[&(OperationType::CCX as u8)], 1_842_771);
        assert_eq!(counts[&(OperationType::Hmr as u8)], 857_383);
        assert_eq!(counts[&(OperationType::CZ as u8)], 854_058);
        assert_eq!(SOURCE_STREAM_SHA256.len(), 64);
        assert_eq!(LOWERED_RECORD_SHA256.len(), 64);
    }
}
