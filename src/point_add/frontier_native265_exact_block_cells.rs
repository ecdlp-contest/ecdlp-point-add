// Generated from formal/ir/frontier-native265-exact-block-cells-v1.json.
// Generated Rust is output-only; block storage is a child-owned generated cell.
// ir_sha256=40a11090b679f714184de5b2494000e914e311b37c2a82b2af45888577291485
// schedule_rust_sha256=7a14f610aa58eaa161ad105aff67309a982e753d4e8583d67805cc1425a9d9df
// universal_codec_rust_sha256=b855248da9ff54fbfde44750bbed8b205057fefadaaa8764bc5536f3552edca5
// head_codec_rust_sha256=f68b20ba024c0795c2879e11de154f444761eada36841b86f5f0d2e6b1b4f318
// controlled_swap_rust_sha256=523f6fceae7fb3a96780ab8c7d17bc7d3301e8f75e564f90bdb6d46e63c11c4f
// slot_lifecycle_rust_sha256=c07294a3df605c9ca2b02fd09b7ffecb88d1f5ce090aa3445902ba063beeadde
// low73_double_halve_rust_sha256=196d364e510606a8c157073d28cd71d6f064ee9be720f8ba9349466258729e00
// controlled_row_arithmetic_rust_sha256=630d81e2bc0378aa1f78620e13b55d76fae159ebc0ff63d7f22d2850a49d9563
// encoded_block_store_rust_sha256=7c060410cc5ba3eeef043e1798302d0ffa5b7e063ce05f8e0b4de2c638312a6a
// fullwidth_phase_cleanup_ir_sha256=9958f42e7a79a05dcfaf1a37d4527f1a723f4e0fb8bc009256179630fa9ee47d
// generator_sha256=d35e46db486593a70e75928ba36c39e85e63dc6d83fd5fd70978a196676b7ade

use super::B;
use crate::circuit::QubitId;

pub(super) struct FrontierNative265ExactBlockCells<'a> {
    pub store: &'a mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore,
    live: Option<super::frontier_native265_encoded_block_store::FrontierNative265BlockFrame>,
}

impl<'a> FrontierNative265ExactBlockCells<'a> {
    pub fn new(store: &'a mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore) -> Self { Self { store, live: None } }
}

impl super::frontier_native265_schedule::FrontierNative265Cells for FrontierNative265ExactBlockCells<'_> {
    fn begin_block(&mut self, circuit: &mut B, block: usize, start: usize, end: usize) {
        assert!(self.live.is_none(), "nested native265 block frame");
        let frame = self.store.acquire_block(circuit, block, start, end);
        if block == 0 {
            let head = [frame.raw[0], frame.raw[1], frame.raw[2], frame.raw[4]];
            super::frontier_k2_head_pair8_codec::emit_frontier_k2_head_pair8_decode(circuit, &head);
        }
        super::frontier_k2_five_row_universal_codec::emit_frontier_k2_five_row_decode(circuit, &frame.raw, &frame.scratch);
        self.live = Some(frame);
    }

    fn finish_block(&mut self, circuit: &mut B, block: usize, start: usize, end: usize) {
        let frame = self.live.take().expect("native265 block frame is not live");
        super::frontier_k2_five_row_universal_codec::emit_frontier_k2_five_row_encode(circuit, &frame.raw, &frame.scratch);
        if block == 0 {
            let head = [frame.raw[0], frame.raw[1], frame.raw[2], frame.raw[4]];
            super::frontier_k2_head_pair8_codec::emit_frontier_k2_head_pair8_encode(circuit, &head);
        }
        self.store.release_block(circuit, block, start, end, frame);
    }

    fn open_slot(&mut self, circuit: &mut B, block: usize, step: usize, slot: usize) {
        let frame = self.live.as_mut().expect("native265 block frame is not live");
        let _ = (block, step);
        super::frontier_k2_full_frame_slot_lifecycle::emit_frontier_k2_open_full_frame_slot(circuit, slot, &mut frame.raw, &mut frame.scratch);
    }
    fn close_slot(&mut self, circuit: &mut B, block: usize, step: usize, slot: usize) {
        let frame = self.live.as_mut().expect("native265 block frame is not live");
        let _ = (block, step);
        super::frontier_k2_full_frame_slot_lifecycle::emit_frontier_k2_close_full_frame_slot(circuit, slot, &mut frame.raw, &mut frame.scratch);
    }
    fn double_y(&mut self, circuit: &mut B, step: usize, y: &mut Vec<QubitId>) {
        let frame = self.live.as_ref().expect("native265 block frame is not live");
        let slot = step % 5;
        let scale_control = frame.raw[3 * slot + 2];
        let fold_window = match step { 47 => 63, _ => 63 };
        super::trailmix_ludicrous::fused::fused_double_cdouble_window(circuit, &scale_control, y, fold_window);
    }
    fn halve_y(&mut self, circuit: &mut B, step: usize, y: &mut Vec<QubitId>) {
        let frame = self.live.as_ref().expect("native265 block frame is not live");
        let slot = step % 5;
        let scale_control = frame.raw[3 * slot + 2];
        let fold_window = match step { 47 => 63, _ => 63 };
        super::trailmix_ludicrous::fused::fused_double_cdouble_reverse_window(circuit, &scale_control, y, fold_window);
    }
    fn controlled_add(&mut self, circuit: &mut B, step: usize, x: &[QubitId], y: &mut Vec<QubitId>) {
        let frame = self.live.as_ref().expect("native265 block frame is not live");
        let slot = step % 5;
        let arithmetic_control = frame.raw[3 * slot];
        let fold_window = match step { _ => 60 };
        let cleanup_compare_bits = match step { _ => 32 };
        super::trailmix_ludicrous::arith::controlled_mod_add_k_window(circuit, &arithmetic_control, x, y, None, None, fold_window, cleanup_compare_bits);
    }
    fn controlled_subtract(&mut self, circuit: &mut B, step: usize, x: &[QubitId], y: &mut Vec<QubitId>) {
        let frame = self.live.as_ref().expect("native265 block frame is not live");
        let slot = step % 5;
        let arithmetic_control = frame.raw[3 * slot];
        super::trailmix_ludicrous::gcd::controlled_mod_sub_vented(circuit, &arithmetic_control, x, y, None);
    }
    fn controlled_swap(&mut self, circuit: &mut B, step: usize, x: &mut Vec<QubitId>, y: &mut Vec<QubitId>) {
        let frame = self.live.as_ref().expect("native265 block frame is not live");
        let slot = step % 5;
        let swap_control = frame.raw[3 * slot + 1];
        super::frontier_k2_controlled_word_swap::emit_frontier_k2_controlled_word_swap(circuit, swap_control, x, y);
    }
}
