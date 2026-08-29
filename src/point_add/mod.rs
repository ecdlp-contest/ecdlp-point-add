//! Generated replay entry point for the contextual zero-control child.

use crate::circuit::{BitId, Op, OperationType, QubitId, RegisterId};

struct B {
    ops: Vec<Op>,
    next_qubit: u64,
    next_bit: u64,
    next_register: u64,
    active_qubits: u32,
    peak_qubits: u32,
}

impl B {
    fn new() -> Self {
        Self {
            ops: Vec::new(),
            next_qubit: 0,
            next_bit: 0,
            next_register: 0,
            active_qubits: 0,
            peak_qubits: 0,
        }
    }

    fn alloc_qubits(&mut self, count: usize) -> Vec<QubitId> {
        let start = self.next_qubit;
        self.next_qubit += count as u64;
        self.active_qubits += count as u32;
        self.peak_qubits = self.peak_qubits.max(self.active_qubits);
        (start..self.next_qubit).map(QubitId).collect()
    }

    fn alloc_bits(&mut self, count: usize) -> Vec<BitId> {
        let start = self.next_bit;
        self.next_bit += count as u64;
        (start..self.next_bit).map(BitId).collect()
    }

    fn declare_qubit_register(&mut self, qubits: &[QubitId]) {
        let register = RegisterId(self.next_register);
        self.next_register += 1;
        for &qubit in qubits {
            let mut op = Op::empty();
            op.kind = OperationType::AppendToRegister;
            op.q_target = qubit;
            op.r_target = register;
            self.ops.push(op);
        }
        let mut op = Op::empty();
        op.kind = OperationType::Register;
        op.r_target = register;
        self.ops.push(op);
    }

    fn declare_bit_register(&mut self, bits: &[BitId]) {
        let register = RegisterId(self.next_register);
        self.next_register += 1;
        for &bit in bits {
            let mut op = Op::empty();
            op.kind = OperationType::AppendToRegister;
            op.c_target = bit;
            op.r_target = register;
            self.ops.push(op);
        }
        let mut op = Op::empty();
        op.kind = OperationType::Register;
        op.r_target = register;
        self.ops.push(op);
    }

    fn push_condition(&mut self, condition: BitId) {
        let mut op = Op::empty();
        op.kind = OperationType::PushCondition;
        op.c_condition = condition;
        self.ops.push(op);
    }

    fn pop_condition(&mut self) {
        let mut op = Op::empty();
        op.kind = OperationType::PopCondition;
        self.ops.push(op);
    }
}

mod qarton_fixed_port;
mod gold1283_contextual_zero_control_child;

pub fn build() -> Vec<Op> {
    gold1283_contextual_zero_control_child::build_classical_q_port()
}
