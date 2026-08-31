//! Generated from formal/source/frontier-native270-rowwise-margin-repair-v4.json.
//! Generated Rust is output-only; imported primitive cells remain explicit.
//! source_sha256=8fa2202153206a09afa906c2b2a5f17501f4286bb6e63183f4db97813776d3c0
//! frontier_native265_schedule_rust_sha256=7a14f610aa58eaa161ad105aff67309a982e753d4e8583d67805cc1425a9d9df
//! frontier_native265_encoded_block_store_rust_sha256=7c060410cc5ba3eeef043e1798302d0ffa5b7e063ce05f8e0b4de2c638312a6a
//! frontier_native265_exact_block_cells_rust_sha256=6bee85ee753f2a6c3234b69c16fb2630f240edb54d1a7c71f58bb148aa3e51a7
//! frontier_native265_source_materializer_topology_rust_sha256=2c559a13a983fbbc1ed3461e1f7519bd7d281a9d25fc926bdbb96031014d7586
//! frontier_native265_source_materializer_lifecycle_rust_sha256=f3703f105e68c3f93fdd91b54a2fb03b23a6a6a4744c1fd1f9c131ba67ac2f90
//! frontier_native265_low_owner_k2_row_rust_sha256=bce89e7f5055eeb8103387af25988d4354779bc706f9bc3e1c1391cbc3bfdd9c
//! frontier_native265_coordinate_subtract_shell_rust_sha256=24e5e94c43cd7d65d110be3cda593b66cedbb67ad4b1b43dc76d666a4b8c4345
//! frontier_native265_coordinate_three_x_rust_sha256=6c60bd3a17eac4bbbddfbcecc98bf742a40c0fb69a5359990e27e3daea65db1a
//! frontier_low_space_square_rust_sha256=b11cb6fb4cf34cb34c15d934a38781c459025970a75632da29a0be939ba83722
//! generator_sha256=d5147aee8873984ba1561ab199f943b4be5f7f2eb59d329613bcf77779bbb3a5

use super::B;
use crate::circuit::{BitId, QubitId};

pub(super) const NATIVE265_CANDIDATE_CONDITIONAL_PEAK: usize = 1282;
pub(super) const NATIVE265_CANDIDATE_CONDITIONAL_TOFFOLI_UPPER: usize = 2100000;

pub(super) fn emit_frontier_native265_candidate(
    circuit: &mut B,
    x: &mut Vec<QubitId>,
    y: &mut Vec<QubitId>,
    ox: &[BitId],
    oy: &[BitId],
    store: &mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore,
) {
    let mut walk_cells = super::frontier_native265_low_owner_k2_row::FrontierNative265LowOwnerK2Row;
    super::frontier_native265_coordinate_subtract_shell::emit_frontier_native265_coordinate_subtract(circuit, x, ox);
    super::frontier_native265_coordinate_subtract_shell::emit_frontier_native265_coordinate_subtract(circuit, y, oy);
    {
        let mut materializer = super::frontier_native265_source_materializer_lifecycle::FrontierNative265GeneratedSourceMaterializer::new(&mut walk_cells);
        super::frontier_native265_source_materializer_topology::emit_frontier_native265_quotient(
            circuit, x, y, store, &mut materializer,
        );
    }
    super::frontier_native265_coordinate_three_x::emit_frontier_native265_coordinate_three_x(circuit, x, ox);
    super::frontier_low_space_square::frontier_low_space_square_subtract(
        circuit, x, y, super::SECP256K1_P,
    );
    {
        let mut materializer = super::frontier_native265_source_materializer_lifecycle::FrontierNative265GeneratedSourceMaterializer::new(&mut walk_cells);
        super::frontier_native265_source_materializer_topology::emit_frontier_native265_product(
            circuit, x, y, store, &mut materializer,
        );
    }
    super::frontier_native265_coordinate_subtract_shell::emit_frontier_native265_coordinate_subtract(circuit, y, oy);
    super::frontier_native265_coordinate_subtract_shell::emit_frontier_native265_coordinate_reverse_subtract(circuit, x, ox);
}
