// Generated from formal/ir/frontier-native270-source-materializer-lifecycle-v4.json.
// Generated Rust is output-only; low-owner row cells and zero-future loans are child-owned.
// ir_sha256=007cc7ce82665bd91cdb271bd3bd4905887b4b424aac3474cf835b757deff284
// topology_rust_sha256=2c559a13a983fbbc1ed3461e1f7519bd7d281a9d25fc926bdbb96031014d7586
// store_rust_sha256=7c060410cc5ba3eeef043e1798302d0ffa5b7e063ce05f8e0b4de2c638312a6a
// universal_codec_rust_sha256=b855248da9ff54fbfde44750bbed8b205057fefadaaa8764bc5536f3552edca5
// head_codec_rust_sha256=f68b20ba024c0795c2879e11de154f444761eada36841b86f5f0d2e6b1b4f318
// low_owner_row_rust_sha256=bce89e7f5055eeb8103387af25988d4354779bc706f9bc3e1c1391cbc3bfdd9c
// generator_sha256=ad60e9d01ccd38a5423ce490d10fa1640f586258bae804d0eb023a89cddbcc2f

use super::B;
use crate::circuit::QubitId;

const SECP256K1_P_LIMBS: [u64; 4] = [
    0xfffffffefffffc2f, 0xffffffffffffffff,
    0xffffffffffffffff, 0xffffffffffffffff,
];

pub(super) trait FrontierNative265K2WalkCells {
    fn forward_row(
        &mut self,
        circuit: &mut B,
        step: usize,
        u: &mut Vec<QubitId>,
        factor: &mut Vec<QubitId>,
        raw: &mut [QubitId; 15],
        slot: usize,
        vents: &[QubitId],
    );
    fn reverse_row(
        &mut self,
        circuit: &mut B,
        step: usize,
        u: &mut Vec<QubitId>,
        factor: &mut Vec<QubitId>,
        raw: &mut [QubitId; 15],
        slot: usize,
        vents: &[QubitId],
    );
}

impl<T: FrontierNative265K2WalkCells + ?Sized> FrontierNative265K2WalkCells for &mut T {
    fn forward_row(
        &mut self, circuit: &mut B, step: usize, u: &mut Vec<QubitId>,
        factor: &mut Vec<QubitId>, raw: &mut [QubitId; 15], slot: usize,
        vents: &[QubitId],
    ) { (**self).forward_row(circuit, step, u, factor, raw, slot, vents); }
    fn reverse_row(
        &mut self, circuit: &mut B, step: usize, u: &mut Vec<QubitId>,
        factor: &mut Vec<QubitId>, raw: &mut [QubitId; 15], slot: usize,
        vents: &[QubitId],
    ) { (**self).reverse_row(circuit, step, u, factor, raw, slot, vents); }
}

pub(super) struct FrontierNative265GeneratedSourceMaterializer<C: FrontierNative265K2WalkCells> {
    cells: C,
    terminal_u: Option<Vec<QubitId>>,
    original_factor_owners: Option<Vec<QubitId>>,
}

impl<C: FrontierNative265K2WalkCells> FrontierNative265GeneratedSourceMaterializer<C> {
    pub(super) fn new(cells: C) -> Self {
        Self { cells, terminal_u: None, original_factor_owners: None }
    }

    fn load_modulus(circuit: &mut B, u: &[QubitId]) {
        assert_eq!(u.len(), 256, "native265 u width");
        for index in 0..256 {
            if ((SECP256K1_P_LIMBS[index / 64] >> (index % 64)) & 1) != 0 {
                circuit.x(u[index]);
            }
        }
    }

    fn decode_frame(
        circuit: &mut B,
        block: usize,
        frame: &super::frontier_native265_encoded_block_store::FrontierNative265BlockFrame,
    ) {
        if block == 0 {
            let head = [frame.raw[0], frame.raw[1], frame.raw[2], frame.raw[4]];
            super::frontier_k2_head_pair8_codec::emit_frontier_k2_head_pair8_decode(circuit, &head);
        }
        super::frontier_k2_five_row_universal_codec::emit_frontier_k2_five_row_decode(
            circuit, &frame.raw, &frame.scratch,
        );
    }

    fn encode_frame(
        circuit: &mut B,
        block: usize,
        frame: &super::frontier_native265_encoded_block_store::FrontierNative265BlockFrame,
    ) {
        super::frontier_k2_five_row_universal_codec::emit_frontier_k2_five_row_encode(
            circuit, &frame.raw, &frame.scratch,
        );
        if block == 0 {
            let head = [frame.raw[0], frame.raw[1], frame.raw[2], frame.raw[4]];
            super::frontier_k2_head_pair8_codec::emit_frontier_k2_head_pair8_encode(circuit, &head);
        }
    }

    fn allocate_fresh_prefix(circuit: &mut B, word: &mut [QubitId]) {
        for owner in word { *owner = circuit.alloc_qubit(); }
    }

    fn restore_owner_identity(
        circuit: &mut B, factor: &mut Vec<QubitId>, original: &[QubitId],
    ) {
        assert_eq!(factor.len(), original.len(), "native265 owner identity width");
        for index in 0..factor.len() {
            let desired = original[index];
            if factor[index] == desired { continue; }
            if let Some(position) = factor[index + 1..].iter().position(|owner| *owner == desired) {
                let other = index + 1 + position;
                circuit.swap(factor[index], factor[other]);
                factor.swap(index, other);
            } else {
                circuit.reacquire(desired);
                let displaced = factor[index];
                circuit.swap(displaced, desired);
                circuit.free(displaced);
                factor[index] = desired;
            }
        }
        assert_eq!(factor.as_slice(), original, "native265 original factor owners restored");
    }
}

impl<C: FrontierNative265K2WalkCells>
    super::frontier_native265_source_materializer_topology::FrontierNative265SourceMaterializer
    for FrontierNative265GeneratedSourceMaterializer<C>
{
    fn begin(
        &mut self,
        circuit: &mut B,
        factor: &mut Vec<QubitId>,
        store: &mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore,
    ) {
        assert!(self.terminal_u.is_none(), "native265 materializer already live");
        assert!(self.original_factor_owners.is_none(), "native265 factor owner snapshot already live");
        assert_eq!(factor.len(), 256, "native265 factor width");
        self.original_factor_owners = Some(factor.clone());
        let mut u = circuit.alloc_qubits(256);
        Self::load_modulus(circuit, &u);
        let mut live_width = 256usize;
        for block in 0..54 {
            let start = block * 5;
            let end = start + 5;
            store.activate_zero_block(circuit, block);
            let mut frame = store.acquire_block(circuit, block, start, end);
            for slot in 0..5 {
                let step = start + slot;
                let active = super::frontier_native265_low_owner_k2_row::FrontierNative265LowOwnerK2Row::active_width(step);
                assert_eq!(live_width, active, "native265 forward live width");
                let need = super::frontier_native265_low_owner_k2_row::FrontierNative265LowOwnerK2Row::vent_count(step);
                let mut vents = Vec::with_capacity(need);
                vents.extend(frame.scratch.iter().copied().take(need));
                let loan_count = need.saturating_sub(vents.len());
                let loan = store.take_zero_future_loan(circuit, block, loan_count);
                vents.extend(loan.owners.iter().copied());
                assert_eq!(vents.len(), need, "native265 forward vent pool");
                self.cells.forward_row(
                    circuit, step, &mut u, factor, &mut frame.raw, slot, &vents,
                );
                store.return_zero_future_loan(circuit, loan);
                let next_live = if step + 1 < 270 {
                    super::frontier_native265_low_owner_k2_row::FrontierNative265LowOwnerK2Row::active_width(step + 1)
                } else {
                    active
                };
                assert!(next_live <= active, "native265 forward width monotonicity");
                circuit.free_vec(&u[next_live..active]);
                circuit.free_vec(&factor[next_live..active]);
                live_width = next_live;
            }
            Self::encode_frame(circuit, block, &frame);
            store.release_block(circuit, block, start, end, frame);
        }
        Self::allocate_fresh_prefix(circuit, &mut factor[live_width..]);
        circuit.x(u[0]);
        circuit.free_vec(&u[..live_width]);
        self.terminal_u = Some(u);
    }

    fn finish(
        &mut self,
        circuit: &mut B,
        factor: &mut Vec<QubitId>,
        store: &mut super::frontier_native265_encoded_block_store::FrontierNative265EncodedBlockStore,
    ) {
        assert_eq!(factor.len(), 256, "native265 factor width");
        let mut u = self.terminal_u.take().expect("native265 materializer is not live");
        let mut live_width = super::frontier_native265_low_owner_k2_row::FrontierNative265LowOwnerK2Row::active_width(269);
        circuit.free_vec(&factor[live_width..]);
        Self::allocate_fresh_prefix(circuit, &mut u[..live_width]);
        circuit.x(u[0]);
        for block in (0..54).rev() {
            let start = block * 5;
            let end = start + 5;
            let mut frame = store.acquire_block(circuit, block, start, end);
            Self::decode_frame(circuit, block, &frame);
            for slot in (0..5).rev() {
                let step = start + slot;
                let active = super::frontier_native265_low_owner_k2_row::FrontierNative265LowOwnerK2Row::active_width(step);
                assert!(live_width <= active, "native265 reverse width monotonicity");
                Self::allocate_fresh_prefix(circuit, &mut u[live_width..active]);
                Self::allocate_fresh_prefix(circuit, &mut factor[live_width..active]);
                live_width = active;
                let need = super::frontier_native265_low_owner_k2_row::FrontierNative265LowOwnerK2Row::vent_count(step);
                let mut vents = Vec::with_capacity(need);
                vents.extend(frame.scratch.iter().copied().take(need));
                let loan_count = need.saturating_sub(vents.len());
                let loan = store.take_zero_future_loan(circuit, block, loan_count);
                vents.extend(loan.owners.iter().copied());
                assert_eq!(vents.len(), need, "native265 reverse vent pool");
                self.cells.reverse_row(
                    circuit, step, &mut u, factor, &mut frame.raw, slot, &vents,
                );
                store.return_zero_future_loan(circuit, loan);
            }
            store.release_zero_block(circuit, block, start, end, frame);
        }
        assert_eq!(live_width, 256, "native265 reverse final live width");
        Self::load_modulus(circuit, &u);
        circuit.free_vec(&u);
        let original = self.original_factor_owners.take().expect("native265 factor owner snapshot is not live");
        Self::restore_owner_identity(circuit, factor, &original);
    }
}
