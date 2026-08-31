// Generated from formal/ir/frontier-k2-full-frame-slot-lifecycle-v1.json.
// Generated Rust is output-only.
// ir_sha256=2d1eb6408c703d3119a3364fd06b309cd2bd953ded67143896f802011282c5ff
// generator_sha256=e92b20d91a98213ee94fe4ee5c071cd4a2f45bf78c472fb282c8a4ac3da9e239

use super::B;
use crate::circuit::QubitId;

pub(super) fn emit_frontier_k2_open_full_frame_slot(
    _circuit: &mut B,
    slot: usize,
    _raw: &mut [QubitId; 15],
    _scratch: &mut [QubitId; 13],
) {
    assert!(slot < 5);
    assert!(3 * slot + 2 < 15);
}

pub(super) fn emit_frontier_k2_close_full_frame_slot(
    _circuit: &mut B,
    slot: usize,
    _raw: &mut [QubitId; 15],
    _scratch: &mut [QubitId; 13],
) {
    assert!(slot < 5);
    assert!(3 * slot + 2 < 15);
}
