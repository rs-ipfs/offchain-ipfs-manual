# Example pallet callable reference

The following callable functions are exposed via JSON-RPC by the example pallet.
These specific functions were chosen due to popularity, to give you a familiar experience.

These functions are not the only ones available in the native runtime, and therefore
do not represent the full extent of functionality available to you.

Regardless, we still detail the example template functions here as a helpful reference. If you have
feedback about these existing functions, or would like to request new functions, please open an issue
at [rs-ipfs/substrate].

[rs-ipfs/substrate]: https://github.com/rs-ipfs/substrate

## Callables

What follows is a list of callables, their frequency in terms of block creation, their weights,
their arguments and return values. As explained in the section on the example pallet,
weights are essentially picosecond representations of a time limit for a given transaction,
and can be loosely correlated to transaction fees.

> **Rust's `snake_case` is used here.** However, as you'll see in the upcoming polkadot.js example,
JavaScript will use `camelCase` for function and variable names. Generally, the exposed JSON-RPC
functions will adhere to the conventions of the programming language that the client code is
written in.

Also, the weights here and block frequencies here are chosen rather arbitrarily without tokenomics
in mind. You will probably need to tune these values in your own custom pallet.

Finally, while we list "return" values for simplicity's sake, the template pallet does not actually
return any values from the RPC calls. What this really means in the context of Substrate is that the
values will be _eventually_ emitted returned in the runtime logs, which you'll need to monitor.

### `ipfs_add_bytes`

Adds the given bytes to the IPFS repository.

**Frequency:** Every other block<br />
**Weight**: 200,000<br />

#### Arguments

- `bytes` - The bytes that you want to add to IPFS, e.g. `vec![1, 2, 3, 4]` or `b"1234"`

#### Returns

A Content ID (CID) string, e.g. `QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa`

### `ipfs_cat_bytes`

Displays the bytes behind a given CID.

**Frequency:** Every other block<br />
**Weight**: 100,000

#### Arguments

- `cid`: The CID of your desired content, e.g. `QmY7Yh4UquoXHLPFo2XbhXkhBvFoPwmQUSa92pxnxjQuPU`

#### Returns

The requested bytes - UTF-8 as a string, non-UTF-8 as hexidecimal string

### `ipfs_connect`

Connects the embedded node to the given Multiaddr.

**Frequency:** Every block<br />
**Weight**: 100,000

#### Arguments

- `multiaddr`: A valid multiaddress with peer ID a the end, e.g. `/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ`

#### Returns

Nothing, or an error.

### `ipfs_disconnect`

Disconnects from the given Multiaddr.

**Frequency:** Every block<br />
**Weight**: 500,000

#### Arguments

- `multiaddr`: A valid multiaddress with peer ID a the end, e.g. `/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ`

#### Returns

Nothing, or an error.

### `ipfs_dht_findpeer`

Performs a search for the addresses associated with the provided PeerId.

**Frequency:** Every block<br />
**Weight**: 100,000

#### Arguments

- `peerID`: A `PeerId` hash, e.g. `QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ`

#### Returns

A multiaddr, such as `/ip4/104.131.131.82/tcp/4001`.

### `ipfs_dht_findproviders`

Search for PeerIds known to be providing the given Cid. You must be connected to at least one peer.

**Frequency:** Every block<br />
**Weight**: 100,000

#### Arguments

- `cid`: The CID of your desired content, e.g. `QmY7Yh4UquoXHLPFo2XbhXkhBvFoPwmQUSa92pxnxjQuPU`

#### Returns

- `peerID`: An array `PeerId` string, e.g. `[QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ]`

### `ipfs_insert_pin`

Non-recursively pins a block with the specified Cid, protecting it from removal.

**Frequency:** Every other block<br />
**Weight**: 100,000

#### Arguments

- `cid`: The CID of the data you want to pin, e.g. `QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa`

#### Returns

Nothing, or an error.

### `ipfs_remove_pin`

Removes a pin from a block, so that it is no longer persistent and can be removed.

**Frequency:** Every other block<br />
**Weight**: 100,000

#### Arguments

- `cid`: The CID of the data you want to unpin, e.g. `QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa`

#### Returns

### `ipfs_remove_block`

Removes a block from the node’s repository.

**Frequency:** Every other block<br />
**Weight**: 300,000

#### Arguments

- `cid`: The CID of the block you want to remove, e.g. `QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa`

#### Returns

Nothing, or an error.
