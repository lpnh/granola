# cpu

Cachegrind benches via `gungraun` over a subset of corpus fixtures.

Counts `Ir` (retired instructions) and `I1mr` (i-cache misses).
Each bench gates on a soft `Ir` band and a hard ceiling.

## Commands

Run all benches:

```sh
cargo bench -p cpu
```

Run all benches, showing only the `Ir` and `I1mr` rows.

```sh
cargo bench -p cpu -- --cachegrind-metrics i1mr,ir
```
