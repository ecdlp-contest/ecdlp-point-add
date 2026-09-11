use quantum_ecc::circuit::*;

pub fn build() -> Vec<Op> {
    let bytes = zstd::stream::decode_all(&include_bytes!("circuit.dat")[..])
        .expect("invalid circuit encoding");
    assert_eq!(bytes.len(), 422355672);
    assert_eq!(bytes.len() % 24, 0);
    let mut result = Vec::with_capacity(26207369);
    let records = bytes.len() / 24;
    for i in 0..records {
        let cell = |j| bytes[j * records + i];
        let word16 = |j| u16::from_le_bytes([cell(j), cell(j + 1)]) as u64;
        let word32 =
            |j| u32::from_le_bytes([cell(j), cell(j + 1), cell(j + 2), cell(j + 3)]) as u64;
        let qubit = |v| if v == 65535 { u64::MAX } else { v };
        let bit = |v| if v == 4294967295 { u64::MAX } else { v };
        let depth = cell(1) as usize;
        assert!(depth <= 2);
        for j in 0..depth {
            let mut o = Op::empty();
            o.kind = OperationType::PushCondition;
            o.c_condition = BitId(word32(12 + 4 * j));
            result.push(o);
        }
        let mut o = Op::empty();
        o.kind = match cell(0) {
            1 => OperationType::Register,
            2 => OperationType::AppendToRegister,
            3 => OperationType::BitInvert,
            4 => OperationType::BitStore0,
            5 => OperationType::BitStore1,
            6 => OperationType::X,
            7 => OperationType::Z,
            8 => OperationType::CX,
            9 => OperationType::CZ,
            12 => OperationType::Hmr,
            13 => OperationType::CCX,
            _ => panic!("invalid circuit gate"),
        };
        o.q_control2 = QubitId(qubit(word16(2)));
        o.q_control1 = QubitId(qubit(word16(4)));
        o.q_target = QubitId(qubit(word16(6)));
        o.c_target = BitId(bit(word32(8)));
        o.r_target = RegisterId(bit(word32(20)));
        result.push(o);
        for _ in 0..depth {
            let mut o = Op::empty();
            o.kind = OperationType::PopCondition;
            result.push(o);
        }
    }
    assert_eq!(result.len(), 26207369);
    result
}
