//! Generated wrapper for the immutable Qarton hybrid-4s4s classical-Q epoch.
//! Regenerate this output from the pinned Qarton source and QPRT stream.

use super::qarton_fixed_port::build_replay;
use crate::circuit::Op;

const COMPRESSED_STREAM: &[u8] = include_bytes!("qarton_hybrid_4s4s_v1.qprt.zst");
pub(super) const PROFILE_ID: &str = "qarton-hybrid-gate-walk-space-replay-4s4s";
pub(super) const SOURCE_STREAM_SHA256: &str =
    "18a114949e77ad5f586f58523d8ba261616e0892568b29cfe2d80b69a05a4917";
pub(super) const LOWERED_RECORD_SHA256: &str =
    "a35c38d09fe8777333b15cf93c05964a564544990f42f79a8b1feb21472bb888";
pub(super) const QPRT_SHA256: &str =
    "a4f6f22453d26643df63e977e79292068cc5b143f9705d585ae93bb3677354f7";
pub(super) const QUBITS: usize = 1283;
pub(super) const CLASSICAL_BITS: usize = 768;
pub(super) const LOWERED_RECORDS: usize = 12193937;
pub(super) const TWO_CONTROL_RECORDS: usize = 91428;
pub(super) const REGISTER_DECLARATION_OPS: usize = 1028;
pub(super) const EMITTED_OPERATIONS: usize = 12377821;

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
        assert_eq!(counts[&(OperationType::CCX as u8)], 2572427);
        assert_eq!(counts[&(OperationType::Hmr as u8)], 704099);
        assert_eq!(counts[&(OperationType::CZ as u8)], 577162);
        assert_eq!(
            counts[&(OperationType::PushCondition as u8)],
            TWO_CONTROL_RECORDS
        );
        assert_eq!(
            counts[&(OperationType::PopCondition as u8)],
            TWO_CONTROL_RECORDS
        );
        assert_eq!(SOURCE_STREAM_SHA256.len(), 64);
        assert_eq!(LOWERED_RECORD_SHA256.len(), 64);
        assert_eq!(QPRT_SHA256.len(), 64);
    }
}
