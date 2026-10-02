# GCD Skywalk point-add block schedule (1,214-qubit reliable configuration)

Circuit: `(x, y) <- (x, y) + (Qx, Qy)` on secp256k1, affine, `Q` classical. Emitted by
`build_point_add` in `mod.rs` with the configuration installed by
`install_skywalk_submission_recipe` (`RELIABLE_OVERRIDES` first, then the recipe). The build
clears the inherited process environment (unless the `PT_EXP` override switch is set), so
the emitted stream depends only on the source.

## Phases (each returns its scratch to |0>)

| phase | operation | main blocks |
|---|---|---|
| `coord_x_sub` | x <- x - Qx | classical-operand subtract with windowed modular fold |
| `coord_y_sub` | y <- y - Qy | same |
| `divide` | y <- y / x (lambda) | forward GCD Skywalk walk, 415 steps, payload cells (`heo.rs`, `heo_carry.rs`) |
| `coord_add3x` | x <- x + 3Qx | classical-operand add |
| `square` | x <- x - lambda^2 | low-space modular square (`square.rs`) |
| `multiply` | y <- y * x | reversed GCD Skywalk walk, 415 steps |
| `coord_y_sub_final` | y <- y - Qy | classical-operand subtract |
| `coord_rsub_final` | x <- Qx - x | classical-operand reverse subtract |

After emission: exact redundant-CCX removal (product, affine, quadratic and truth-table
supports), interned witnesses, measurement absorption, and the inherited 96-gate X identity
tail.

## Walk

- Rails hold the unordered pair {u+v, u} with signs; the even rail is routed to the halving
  position by its low bit (one controlled rail exchange), halving is a wire relabelling, and
  one signed addition forms the other rail's difference.
- History: orientation and sign-flip bits (3 legal joint values), 5 steps per 8 tape wires,
  stored in the rails' freed top bits.
- Payloads: Sigma/Delta frame, Sigma <- (Sigma -/+ Delta)/2 mod p, swap on sign flip;
  multiplication applies the inverse order. At the division endpoint the payloads coincide and
  one is cleared with CNOTs.
- Per-step rail widths follow compiled-in envelope rows:
  `skywalk_data/tw_R415_div.txt`, `tw_R415_mul.txt` (rails) and `tw_proxy_R415.txt` (payload-cell
  proxy), selected through `heo::builtin_table`.

## Reliability configuration (`RELIABLE_OVERRIDES`)

| setting | value | purpose |
|---|---|---|
| walk legs | 415 steps, R415 rows | cover unfinished walks and row exceedances |
| `HEO_PIN_PP_WALK_MAX_QUBITS` | 1214 | qubit cap; room for full late-cell windows |
| `GO_CHUNK` | +20 (rounds >= 675: +24) | chunk-boundary compare widths |
| `GO_FLAG` | +15 (rounds >= 675: +21) | overflow-flag compare widths |
| replay fold windows | 78 (div and mul) | modular fold windows |
| `HEO_PIN_FOLD_GUARD` | 33 | fold guard bits |
| `HEO_FIT_K` | 31 | rail-fit window |
| `HEO_PIN_SQ_ASM_TAIL` | 30 | square carry tail |
| `HEO_PIN_ERASE_COMPARE` | 31 | erase compare width |
| `TERMINAL_FW` | 55 | terminal fold window |
| `GO_PRB` | +12 (rounds 631-698: +11) | retained prebias |
| `HEO_SPLIT_MUL`, `HEO_SPLIT_K` | exact, 28 | multiply-leg rail split |
| `GO_JLB` | +20 | joint low-fold shortcut disabled (low window raised by 20 bits, so the regular fold is used) |

Optional fused constructs present in the source (shared reverse passes, fused y updates,
classical-operand compares, multiply-batch step skip, rewrite rows) are disabled.

## Measured

1,214 qubits; 1,063,436.582 average executed CCX+CCZ (Toffoli depth 1,063,437, serial);
11,487,001 operations; score 1,291,012,518; `ops.bin` sha256
`cd9bad00f51749d15bdb9fb966d364c3eda28f327ae9d667f546b75f02205549`; trusted 102,400-shot
default draw and 26 seeded draws all clean.
