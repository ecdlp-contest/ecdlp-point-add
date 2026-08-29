//! Generated wrapper for the gate-efficient contest-ABI Qarton epoch.
//! Regenerate this output from the pinned Qarton source and QPRT stream.

use super::qarton_fixed_port::build_replay;
use crate::circuit::Op;

const COMPRESSED_STREAM: &[u8] = include_bytes!("qarton_contest_gate_eff_v1.qprt.zst");
pub(super) const PROFILE_ID: &str = "qarton-gate-efficient-contest-abi-v1";
pub(super) const QPRT_SHA256: &str =
    "5751bc09399bcb3bb921519fb5e004d9f65958d8241e05c2b4662d59798920f3";
pub(super) const QUBITS: usize = 1482;
pub(super) const CLASSICAL_BITS: usize = 768;
pub(super) const LOWERED_RECORDS: usize = 11559597;

pub(super) fn build_contest_gate_eff_port() -> Vec<Op> {
    build_replay(
        COMPRESSED_STREAM,
        QUBITS,
        CLASSICAL_BITS,
        LOWERED_RECORDS,
        // reg0 = P.x qubits, reg1 = P.y qubits
        &[(0, 256), (256, 512)],
        // reg2 = Q.x bits (preserved), reg3 = Q.y bits (preserved)
        &[(0, 256), (256, 512)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::OperationType;

    #[test]
    fn generated_epoch_decodes_with_exact_census() {
        let ops = build_contest_gate_eff_port();
        let mut counts = std::collections::BTreeMap::new();
        for op in &ops {
            op.validate();
            *counts.entry(op.kind as u8).or_insert(0_usize) += 1;
        }
        assert_eq!(counts[&(OperationType::CCX as u8)], 2_013_819);
        assert_eq!(counts[&(OperationType::Hmr as u8)], 940_419);
        assert_eq!(counts[&(OperationType::CZ as u8)], 935_046);
        assert_eq!(QPRT_SHA256.len(), 64);
        assert_eq!(PROFILE_ID.len(), 36);
    }
}
