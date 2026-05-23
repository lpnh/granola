# diagnostics

Compile-fail tests for the diagnostics emitted by `#[derive(Recipe)]`.

Each fixture in `tests/ui/*.rs` is a program expected to **fail** compilation.
The expected compiler output is captured in the sibling `*.stderr` snapshot.
The test in `tests/ui.rs` drives [`trybuild`](https://docs.rs/trybuild) over
every fixture and compares the actual error against its snapshot.

## What's covered

| Fixture            | Triggers                                                        |
| ------------------ | --------------------------------------------------------------- |
| `missing_from.rs`  | `type Content` without `From<Custom> for Default` (`BakeInto`)  |
| `not_a_recipe.rs`  | composing a non-recipe into a chain, e.g. `(Valid, NotARecipe)` |

## Run the tests

```sh
cargo test -p diagnostics
```

Each fixture must fail to compile with output matching its `.stderr`. A mismatch fails
the test and prints a diff.

## Regenerate the snapshots

The `.stderr` files are nightly-sensitive: a rustc upgrade that reformats diagnostics
will break them. Regenerate after an intended change to the messages (or a toolchain
bump):

```sh
TRYBUILD=overwrite cargo test -p diagnostics
```
