// Generated from formal/ir/frontier-native270-encoded-block-store-v4.json.
// Generated Rust is output-only.
// ir_sha256=92c37bb900b5c04445add5a0fa6a07345d252587a67a93d4c3a3a871628bef93
// generator_sha256=8e42e676e715163c3f64feff63e273ba3d0ad5cd3f19ee871616f66ad1d7d6d3

use super::B;
use crate::circuit::QubitId;

pub(super) const NATIVE265_ENCODED_OWNER_COUNT: usize = 647;
pub(super) const NATIVE265_MAX_STORE_TRANSIENT: usize = 17;
const ORDINARY_CODE_WIRES: [usize; 12] = [0, 1, 2, 4, 5, 6, 7, 8, 9, 11, 12, 14];
const ORDINARY_RELEASED_WIRES: [usize; 3] = [3, 10, 13];
const HEAD_CODE_WIRES: [usize; 11] = [1, 2, 4, 5, 6, 7, 8, 9, 11, 12, 14];
const HEAD_RELEASED_WIRES: [usize; 4] = [0, 3, 10, 13];

pub(super) struct FrontierNative265BlockFrame {
    pub raw: [QubitId; 15],
    pub scratch: [QubitId; 13],
}

pub(super) struct FrontierNative265EncodedBlockStore {
    owners: Vec<Option<QubitId>>,
    active_blocks: [bool; 54],
}

pub(super) struct FrontierNative265ZeroFutureLoan {
    indices: Vec<usize>,
    pub owners: Vec<QubitId>,
}

impl FrontierNative265EncodedBlockStore {
    pub(super) fn new_lazy() -> Self {
        Self { owners: vec![None; NATIVE265_ENCODED_OWNER_COUNT], active_blocks: [false; 54] }
    }

    fn block_layout(block: usize) -> (usize, &'static [usize], &'static [usize]) {
        assert!(block < 54, "native270 block index");
        if block == 0 {
            (0, &HEAD_CODE_WIRES, &HEAD_RELEASED_WIRES)
        } else {
            (11 + (block - 1) * 12, &ORDINARY_CODE_WIRES, &ORDINARY_RELEASED_WIRES)
        }
    }

    pub(super) fn activate_zero_block(&mut self, _circuit: &mut B, block: usize) {
        assert!(!self.active_blocks[block], "native265 block already active");
        let (offset, code_wires, _) = Self::block_layout(block);
        assert!((0..code_wires.len()).all(|index| self.owners[offset + index].is_none()),
            "native265 lazy slot occupied");
        self.active_blocks[block] = true;
    }

    pub(super) fn take_zero_future_loan(
        &mut self, circuit: &mut B, current_block: usize, count: usize,
    ) -> FrontierNative265ZeroFutureLoan {
        assert!(current_block < 54, "native270 current block index");
        let owners = circuit.alloc_qubits(count);
        FrontierNative265ZeroFutureLoan { indices: Vec::new(), owners }
    }

    pub(super) fn return_zero_future_loan(
        &mut self, circuit: &mut B, loan: FrontierNative265ZeroFutureLoan,
    ) {
        assert!(loan.indices.is_empty(), "native265 lazy loan has no persistent slots");
        circuit.free_vec(&loan.owners);
    }

    pub(super) fn acquire_block(
        &mut self, circuit: &mut B, block: usize, start: usize, end: usize,
    ) -> FrontierNative265BlockFrame {
        assert_eq!(start, block * 5, "native265 block start");
        assert_eq!(end, start + 5, "native270 block end");
        assert!(self.active_blocks[block], "native265 acquire requires active block");
        let (offset, code_wires, released_wires) = Self::block_layout(block);
        let encoded_count = (0..code_wires.len())
            .filter(|index| self.owners[offset + index].is_some()).count();
        assert!(encoded_count == 0 || encoded_count == code_wires.len(),
            "native265 partial encoded block");
        if encoded_count == 0 {
            return FrontierNative265BlockFrame {
                raw: circuit.alloc_qubits(15).try_into().ok().expect("native265 raw width"),
                scratch: circuit.alloc_qubits(13).try_into().ok().expect("native265 scratch width"),
            };
        }
        let mut raw = Vec::with_capacity(15);
        let mut code_index = 0usize;
        for wire in 0..15 {
            if released_wires.contains(&wire) {
                raw.push(circuit.alloc_qubit());
            } else {
                assert_eq!(code_wires[code_index], wire, "native265 code-wire order");
                raw.push(self.owners[offset + code_index].take().expect("native265 block already acquired"));
                code_index += 1;
            }
        }
        assert_eq!(code_index, code_wires.len(), "native265 acquired code width");
        let scratch: [QubitId; 13] = circuit.alloc_qubits(13).try_into().ok().expect("native265 scratch width");
        FrontierNative265BlockFrame {
            raw: raw.try_into().ok().expect("native265 raw width"),
            scratch,
        }
    }

    pub(super) fn release_block(
        &mut self, circuit: &mut B, block: usize, start: usize, end: usize,
        frame: FrontierNative265BlockFrame,
    ) {
        assert_eq!(start, block * 5, "native265 block start");
        assert_eq!(end, start + 5, "native270 block end");
        let (offset, code_wires, released_wires) = Self::block_layout(block);
        for (code_index, wire) in code_wires.iter().copied().enumerate() {
            assert!(self.owners[offset + code_index].is_none(), "native265 code slot still occupied");
            self.owners[offset + code_index] = Some(frame.raw[wire]);
        }
        for wire in released_wires.iter().copied() {
            circuit.free(frame.raw[wire]);
        }
        circuit.free_vec(&frame.scratch);
    }

    pub(super) fn release_zero_block(
        &mut self, circuit: &mut B, block: usize, start: usize, end: usize,
        frame: FrontierNative265BlockFrame,
    ) {
        assert_eq!(start, block * 5, "native265 block start");
        assert_eq!(end, start + 5, "native270 block end");
        assert!(self.active_blocks[block], "native265 zero release requires active block");
        let (offset, code_wires, _) = Self::block_layout(block);
        assert!((0..code_wires.len()).all(|index| self.owners[offset + index].is_none()),
            "native265 zero release retains encoded owners");
        circuit.free_vec(&frame.raw);
        circuit.free_vec(&frame.scratch);
        self.active_blocks[block] = false;
    }

    pub(super) fn assert_all_parked(self) {
        assert!(self.active_blocks.iter().all(|active| !active), "native265 store still active");
        assert!(self.owners.iter().all(Option::is_none), "native265 lazy store retains owners");
    }
}
