# Exact generated target fanout

The candidate keeps the full conservative affine/Kaliski construction and applies one narrow
post-emission identity at five reviewed parent-stream windows:

```text
CCX(a, b, t); CX(t, u); CCX(a, b, t)
    =>
CX(t, u); CCX(a, b, u)
```

On a computational-basis state, the first source CCX toggles `t` by `a & b`; the CX therefore
toggles `u` by the original `t` and by `a & b`; the last source CCX restores `t`. The replacement
applies those two effects directly. Four distinct qubit lanes and identical condition context are
required.

`generated_exact_rewrites.rs` is fail closed. It checks the reviewed absolute index, both CCX
controls and targets, the CX control and target, inline conditions, condition-stack depth, and
non-quantum fields before changing the stream. It removes five CCX gates with no allocator change.

The finalized stream passes all 102,400 verifier shots with `0/0/0` failures. It emits 40,922,095
operations, uses 2,841 peak qubits, averages 5,180,781 executed Toffolis, and scores
14,718,598,821. The verifier was an acceptance gate, not the source of the transformation.
