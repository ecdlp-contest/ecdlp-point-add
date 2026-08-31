// Generated from formal/ir/frontier-native265-source-materializer-topology-v1.json.
// Generated Rust is output-only; materializer begin/finish remains the one typed-open family.
// ir_sha256=12ceedb361561c73bac81060a14a575ab59a604388707db2932e70b094add00d
// schedule_rust_sha256=a7ffa00f8023536d1dbc3ca030ea4c70a230bbc90e0c69496fa269c2af5be3e0
// exact_block_rust_sha256=5c2ddcc6473f20214638c9275ca08a461bbde8b62f93b5e91e11021b0f20d4a1
// store_rust_sha256=d5af84e50fdc40883c2de5d2604c32d88483699386eb8c143f4774c449f1aa83
// generator_sha256=aac75e63519b014f55957ca00ed27f0f3002d4eb69f1b37b533c4e2e97b5bdec

use super::B;
use crate::circuit::QubitId;

pub(super) trait FrontierNative265SourceMaterializer {
    fn begin(
        &mut self,
        circuit: &mut B,
        factor: &mut Vec<QubitId>,
        store: &mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore,
    );
    fn finish(
        &mut self,
        circuit: &mut B,
        factor: &mut Vec<QubitId>,
        store: &mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore,
    );
}

fn swap_words(circuit: &mut B, left: &mut Vec<QubitId>, right: &mut Vec<QubitId>) {
    assert_eq!(left.len(), 256);
    assert_eq!(right.len(), 256);
    for index in 0..256 {
        circuit.swap(left[index], right[index]);
    }
}

fn toggle_replay_p_representative(circuit: &mut B, word: &[QubitId]) {
    assert_eq!(word.len(), 256);
    const REPLAY_P_SET_BITS: &[usize] = &[0, 1, 2, 3, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255];
    for &index in REPLAY_P_SET_BITS {
        circuit.x(word[index]);
    }
}

pub(super) fn emit_frontier_native265_quotient<M: FrontierNative265SourceMaterializer>(
    circuit: &mut B,
    factor: &mut Vec<QubitId>,
    target: &mut Vec<QubitId>,
    store: &mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore,
    materializer: &mut M,
) {
    materializer.begin(circuit, factor, store);
    swap_words(circuit, target, factor);
    toggle_replay_p_representative(circuit, target);
    {
        let mut cells = super::frontier_native265_exact_block_cells::FrontierNative265ExactBlockCells::new(store);
        super::frontier_native265_schedule::emit_native265_unapply(circuit, target, factor, &mut cells);
    }
    materializer.finish(circuit, factor, store);
}

pub(super) fn emit_frontier_native265_product<M: FrontierNative265SourceMaterializer>(
    circuit: &mut B,
    factor: &mut Vec<QubitId>,
    target: &mut Vec<QubitId>,
    store: &mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore,
    materializer: &mut M,
) {
    materializer.begin(circuit, factor, store);
    {
        let mut cells = super::frontier_native265_exact_block_cells::FrontierNative265ExactBlockCells::new(store);
        super::frontier_native265_schedule::emit_native265_apply(circuit, target, factor, &mut cells);
    }
    toggle_replay_p_representative(circuit, target);
    swap_words(circuit, target, factor);
    materializer.finish(circuit, factor, store);
}
