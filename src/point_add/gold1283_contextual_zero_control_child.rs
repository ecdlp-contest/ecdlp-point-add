//! Generated wrapper for the immutable Gold-1283 contextual zero-control child epoch.
//! Generated Rust is an output. Regenerate it from the formal profile and QPRT stream.

use super::qarton_fixed_port::build_replay;
use crate::circuit::Op;

const COMPRESSED_STREAM: &[u8] = include_bytes!("gold1283_contextual_zero_control_child.qprt.zst");
pub(super) const PROFILE_ID: &str = "gold1283-contextual-zero-control-child-v1";
pub(super) const SOURCE_STREAM_SHA256: &str = "eb74bd4e2d86ce1d94db69ab89e93d02f77ae12f59f104cd6cf57dcc74adf53e";
pub(super) const LOWERED_RECORD_SHA256: &str = "a7428bb3bf5e58bbc6d52c809a012c605634671ac0515b6b585b68c7991e13f4";
pub(super) const QPRT_SHA256: &str = "430a8f4d6477c5488dd822254a82c566d338b7e68afb1ca57baf64bd425881ec";
pub(super) const QUBITS: usize = 1283;
pub(super) const CLASSICAL_BITS: usize = 768;
pub(super) const LOWERED_RECORDS: usize = 12193785;
pub(super) const TWO_CONTROL_RECORDS: usize = 91428;
pub(super) const REGISTER_DECLARATION_OPS: usize = 1028;
pub(super) const EMITTED_OPERATIONS: usize = 12377669;

pub(super) fn build_classical_q_port() -> Vec<Op> {
    build_replay(
        COMPRESSED_STREAM,
        QUBITS,
        CLASSICAL_BITS,
        LOWERED_RECORDS,
        &[(0, 256), (256, 512)],
        &[(0, 256), (256, 512)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::OperationType;

    #[test]
    fn generated_epoch_decodes_with_exact_census() {
        let ops = build_classical_q_port();
        let mut counts = std::collections::BTreeMap::new();
        for op in &ops {
            op.validate();
            *counts.entry(op.kind as u8).or_insert(0_usize) += 1;
        }
        assert_eq!(ops.len(), EMITTED_OPERATIONS);
        assert_eq!(counts[&(OperationType::CCX as u8)], 2572275);
        assert_eq!(counts[&(OperationType::Hmr as u8)], 704099);
        assert_eq!(counts[&(OperationType::CZ as u8)], 577162);
        assert_eq!(counts[&(OperationType::PushCondition as u8)], TWO_CONTROL_RECORDS);
        assert_eq!(counts[&(OperationType::PopCondition as u8)], TWO_CONTROL_RECORDS);
        assert_eq!(SOURCE_STREAM_SHA256.len(), 64);
        assert_eq!(LOWERED_RECORD_SHA256.len(), 64);
        assert_eq!(QPRT_SHA256.len(), 64);
    }
}
