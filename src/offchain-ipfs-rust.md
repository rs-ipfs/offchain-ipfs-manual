# Using `offchain::ipfs` from your Rust code

## With `substrate-subxt`

In your Cargo.toml file

<!-- markdownlint-disable MD013 --> <!-- long line for codec include -->
```toml
{{#include ../code/subxt-ipfs/Cargo.toml:9:13}}

```
<!-- markdownlint-restore -->

Then in your main.rs:

```rust, no_run, noplayground
{{#include ../code/subxt-ipfs/src/main.rs:1:37}}

{{#include ../code/subxt-ipfs/src/main.rs:80:92}}
```

### With `substrate-api-client`

In your Cargo.toml file:

```toml
{{#include ../code/sub-api-ipfs/Cargo.toml:9:12}}
```

Then in your main.rs:

```rust, no_run, noplayground
{{#include ../code/sub-api-ipfs/src/main.rs:1:46}}
{{#include ../code/sub-api-ipfs/src/main.rs:54:77}}
```

For full demo with all pallet functions, please visit [here](https://github.com/rs-ipfs/offchain-ipfs-manual/tree/main/code)
