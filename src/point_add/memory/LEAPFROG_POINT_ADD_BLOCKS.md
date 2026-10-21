# Leapfrog point-add block schedule (low-error configuration, 1,360 qubits)

Circuit: `(x, y) <- (x, y) + (Qx, Qy)` on secp256k1, affine, `Q` classical. Emitted by `build_point_add` in
`mod.rs` with the configuration installed by `install_skywalk_submission_recipe` and then
`leapfrog::install_recipe` (whose last entries are the low-error settings). The build clears the inherited
process environment first, so the emitted stream depends only on the source.

## Phases (each returns its scratch to |0>)

| phase | operation | main blocks |
|---|---|---|
| `coord_y_sub` | y <- y - Qy | classical-operand subtract |
| `divide` | y <- y / x (lambda) | `x - Qx` fused into the seed; forward Leapfrog walk, 156 ticks, payload cells (`leapfrog.rs`, `y15_onefold.rs`, `pingpong.rs` cells) |
| `square` | x <- x + 3Qx - lambda^2 | `3Qx` fused into the seam; chained-row square in the 2^128 frame (`square.rs`, `sqc.rs`) |
| `multiply` | y <- y * x | reversed Leapfrog walk, 156 ticks; `Qx - x` fused into the unseed |
| `coord_y_sub_final` | y <- y - Qy | classical-operand subtract |

After emission: exact redundant-CCX removal (product, affine, quadratic and truth-table supports),
interned witnesses, measurement absorption, the parity scan, the conditional pass and the classical-factor
passes, then the inherited 96-gate X identity tail.

## Walk

- Two odd signed rails held without their constant bit 0. Per tick: forced add-halve, forced add-halve with
  the choice logic inside its rail add (window compare of the rails' top bits, deep or flat), choice
  add-halve, barrel of 0-3 halvings. Letter: 2 forced signs + choice + 2 barrel bits.
- Widths per tick: `leapfrog_data/hard_table.txt` (`W n0 n1 n2 nb`), 156 rows, the last a parked row.
- Payload: `P_i = mu R_i mod p`; payload-only passes for the early ticks, fused one-fold ticks later, a tail
  batch after the rails park; the multiplication mirrors it in reverse.

## Low-error settings (end of `leapfrog::install_recipe`)

Walk table and length; qubit cap 1,360; chunk-boundary compares +12 (`E008_CHUNK_DIR`), narrowed-compare table
off (`LF_NO_NARROW`); flag compares +12 and boundary shift from tick 0 (`LF_CMP_SHIFT`, `LF_CMP_FROM`);
merged fold windows 70 (`LF_MERGED_WIN`, `LF_MERGED_LATE_WIN`); cell fold floor 72 (`LF_CELL_FOLD_MIN`);
one-fold fold window +12 (`LF_Y15_WIN_ADD`); erase compare 40; fold guard 32; square: exact assembly tail,
wrap witness 33 top bits (`SQC_T`), correction lines and REDC guard 36 (`SQC_G`); stream-bound tables,
recorded allocator states and tick loans off (`LF_UNBIND`, `LF_NO_Y42_ALLOC`, `LF_NO_TICK_LOANS`); the
multiplication's joint low-fold shortcut off (`PP_JOINT_GUARD` unset through its pin).
