//! Four-register classical-Q adaptation of Schrottenloher's Qarton schedule.
//!
//! The selector is specialized to true, finite-point flags are removed, fixed
//! lookups are replaced by classically conditioned loads, and the `3*Q.x`
//! lookup becomes three modular additions from one recyclable loaded Q.x word.

use super::qarton_fixed_port::build_replay;
use crate::circuit::Op;

const COMPRESSED_STREAM: &[u8] = include_bytes!("qarton_classical_q_v1.qprt.zst");
pub(super) const SOURCE_STREAM_SHA256: &str =
    "2edf1a06c4c285b20fce52eef6c28783ccb9563a0f4bd4dd241d9bc144031f2e";
pub(super) const LOWERED_RECORD_SHA256: &str =
    "97c0329cdb77b80458972682a56dadd83565037cdd1dc04648b49e13c1cb8dda";
pub(super) const QUBITS: usize = 1536;
pub(super) const CLASSICAL_BITS: usize = 768;
pub(super) const LOWERED_GATES: usize = 13_476_916;

pub(super) fn build_classical_q_port() -> Vec<Op> {
    build_replay(
        COMPRESSED_STREAM,
        QUBITS,
        CLASSICAL_BITS,
        LOWERED_GATES,
        &[(0, 256), (256, 512)],
        &[(0, 256), (256, 512)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::OperationType;

    #[test]
    fn adapted_port_decodes_with_expected_gate_counts() {
        let ops = build_classical_q_port();
        let mut counts = std::collections::BTreeMap::new();
        for op in &ops {
            op.validate();
            *counts.entry(op.kind as u8).or_insert(0_usize) += 1;
        }
        assert_eq!(counts[&(OperationType::CCX as u8)], 2_313_618);
        assert_eq!(counts[&(OperationType::Hmr as u8)], 1_097_858);
        assert_eq!(counts[&(OperationType::CZ as u8)], 1_093_509);
        assert_eq!(SOURCE_STREAM_SHA256.len(), 64);
        assert_eq!(LOWERED_RECORD_SHA256.len(), 64);
    }
}
