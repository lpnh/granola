# alloc

Snapshots heap behavior per corpus fixture through a global allocator:

- `allocs`: call count
- `churn`: cumulative bytes
- `peak`: live high-water

## Commands

Check against the snapshot:

```sh
cargo test -p alloc
```

Review the snapshot:

```sh
cargo insta review
```

Prints the table:

```sh
cargo run -p alloc
```
