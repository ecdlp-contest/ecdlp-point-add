//! Generated compact decoder for the frozen QPRT operation stream.

use super::B;
use crate::circuit::{BitId, Op, OperationType, QubitId};

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
        .expect("truncated replay record")
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
        7 => (OperationType::Hmr, 2),
        opcode => panic!("unknown replay opcode {opcode}"),
    };
    assert!(controls <= 2, "unsupported condition arity");
    Header {
        kind,
        quantum_arity,
        controls,
    }
}

fn append_gate(builder: &mut B, kind: OperationType, targets: &[u16], controls: &[u16]) {
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
        builder.push_condition(BitId(u64::from(controls[0])));
    }
    op.validate();
    builder.ops.push(op);
    if controls.len() == 2 {
        builder.pop_condition();
    }
}

pub(super) fn build_replay(
    compressed_stream: &[u8],
    expected_qubits: usize,
    expected_cbits: usize,
    expected_gates: usize,
    quantum_registers: &[(usize, usize)],
    classical_registers: &[(usize, usize)],
) -> Vec<Op> {
    let stream = zstd::stream::decode_all(compressed_stream).expect("embedded replay must be valid");
    let stream = stream.as_slice();
    assert_eq!(&stream[..6], b"QPRT1\0");
    let mut cursor = 6;
    let qubits = usize::from(read_u16(stream, &mut cursor));
    let cbits = usize::from(read_u16(stream, &mut cursor));
    let count = u64::from_le_bytes(stream[cursor..cursor + 8].try_into().unwrap()) as usize;
    cursor += 8;
    assert_eq!(
        (qubits, cbits, count),
        (expected_qubits, expected_cbits, expected_gates)
    );

    let mut builder = B::new();
    let quantum = builder.alloc_qubits(qubits);
    let classical = builder.alloc_bits(cbits);
    for &(start, end) in quantum_registers {
        builder.declare_qubit_register(&quantum[start..end]);
    }
    for &(start, end) in classical_registers {
        builder.declare_bit_register(&classical[start..end]);
    }

    for _ in 0..count {
        let header = decode_header(*stream.get(cursor).expect("truncated replay"));
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
            &mut builder,
            header.kind,
            &targets[..header.quantum_arity],
            &controls[..header.controls],
        );
    }
    assert_eq!(cursor, stream.len(), "trailing replay bytes");
    assert_eq!(builder.peak_qubits as usize, expected_qubits);
    builder.ops
}
