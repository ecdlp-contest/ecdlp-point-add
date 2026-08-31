//! Generated from formal/ir/frontier-native270-schedule-v4.json.
//! Generated Rust is output-only; all block and per-row circuit cells remain typed imports.
//! ir_sha256=69d8a001c54d56c97db78efcd3e4de101ff8aa5e5f28dd95d7f0eab60b2363ef
//! generator_sha256=aa994744b624c2e732788af2114eb16494eecc1617a26a921c303cea165ba09c

use super::B;
use crate::circuit::QubitId;

pub(super) const NATIVE265_STEPS: usize = 270;
pub(super) const NATIVE265_BLOCK_WIDTH: usize = 5;
pub(super) const NATIVE265_BLOCK_COUNT: usize = 54;
pub(super) const NATIVE265_FULL_BLOCKS: usize = 54;
pub(super) const NATIVE265_TAIL_STEPS: usize = 0;
pub(super) const NATIVE265_CONDITIONAL_PEAK: usize = 1282;
pub(super) const NATIVE265_CONDITIONAL_TOFFOLI: usize = 2100000;

pub(super) trait FrontierNative265Cells {
    fn begin_block(&mut self, circuit: &mut B, block: usize, start: usize, end: usize);
    fn finish_block(&mut self, circuit: &mut B, block: usize, start: usize, end: usize);
    fn open_slot(&mut self, circuit: &mut B, block: usize, step: usize, slot: usize);
    fn close_slot(&mut self, circuit: &mut B, block: usize, step: usize, slot: usize);
    fn double_y(&mut self, circuit: &mut B, step: usize, y: &mut Vec<QubitId>);
    fn halve_y(&mut self, circuit: &mut B, step: usize, y: &mut Vec<QubitId>);
    fn controlled_add(&mut self, circuit: &mut B, step: usize, x: &[QubitId], y: &mut Vec<QubitId>);
    fn controlled_subtract(&mut self, circuit: &mut B, step: usize, x: &[QubitId], y: &mut Vec<QubitId>);
    fn controlled_swap(&mut self, circuit: &mut B, step: usize, x: &mut Vec<QubitId>, y: &mut Vec<QubitId>);
}

pub(super) fn emit_native265_apply<C: FrontierNative265Cells>(
    circuit: &mut B, x: &mut Vec<QubitId>, y: &mut Vec<QubitId>, cells: &mut C,
) {
    for block in (0..NATIVE265_BLOCK_COUNT).rev() {
        let start = block * NATIVE265_BLOCK_WIDTH;
        let end = (start + NATIVE265_BLOCK_WIDTH).min(NATIVE265_STEPS);
        cells.begin_block(circuit, block, start, end);
        for step in (start..end).rev() {
            let slot = step - start;
            cells.open_slot(circuit, block, step, slot);
            cells.double_y(circuit, step, y);
            cells.controlled_add(circuit, step, x, y);
            cells.controlled_swap(circuit, step, x, y);
            cells.close_slot(circuit, block, step, slot);
        }
        cells.finish_block(circuit, block, start, end);
    }
}

pub(super) fn emit_native265_unapply<C: FrontierNative265Cells>(
    circuit: &mut B, x: &mut Vec<QubitId>, y: &mut Vec<QubitId>, cells: &mut C,
) {
    for block in 0..NATIVE265_BLOCK_COUNT {
        let start = block * NATIVE265_BLOCK_WIDTH;
        let end = (start + NATIVE265_BLOCK_WIDTH).min(NATIVE265_STEPS);
        cells.begin_block(circuit, block, start, end);
        for step in start..end {
            let slot = step - start;
            cells.open_slot(circuit, block, step, slot);
            cells.controlled_swap(circuit, step, x, y);
            cells.controlled_subtract(circuit, step, x, y);
            cells.halve_y(circuit, step, y);
            cells.close_slot(circuit, block, step, slot);
        }
        cells.finish_block(circuit, block, start, end);
    }
}
