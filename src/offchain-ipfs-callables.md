# offchain::ipfs callables

The following callable functions are implemented within the embedded IPFS node and available to you
via our example `templateModule` pallet. These functions were specifically chosen due to popularity
and to give Substrate uses the most cohesive and familiar experience using IPFS within their node.

If you have feedback about these existing functions, or want to request new functions, please
open an issue at [rs-ipfs/substrate].

[rs-ipfs/substrate]: https://github.com/rs-ipfs/substrate

## Notes

For function and argument names Rust will uses `snake_case` and JavaScript will use `camelCase`.
The examples here are given as Rust code, and you should be able to use the simplified JavaScript equivalents.

Also, while we list "return" values, what this means in the context of substrate is that the values
will be returned as chain events, which you will need to listen for.

## The callables

### `ipfs_add_bytes` / `ipfsAddBytes`

Adds the given bytes to the IPFS repository; the off-chain worker interval for this activity is
every block with an odd BlockNumber (i.e. every other block).

#### Arguments

| name | description | example |
| ---- | ----- | ----|
| `bytes` |  Byte array or byte string | `b"1234"` / `vec![1, 2, 3, 4]` |

#### Returns

A string representation of a Content ID (CID), e.g. `QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa`

## `ipfs_cat_bytes` / `ipfsCatBytes`

Displays the bytes (UTF-8 is displayed as a string, while non-UTF-8 bytes are displayed in their
hexadecimal representation) with the given Cid; the off-chain worker interval for this activity
is also every block with an odd BlockNumber.

### Arguments

`cid`: String representation of a CID, e.g. `QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa`

### Returns

In the inverse of the last item, requesting the CID above will return 1234

## `ipfs_connect` / `ipfsConnect`

Connects the embedded node to the given Multiaddr. The off-chain worker interval for this activity
is every block (the queue of connection requests is processed with every block in the chain).

### Arguments

`multiaddr`: String representation, such as `/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ`

### Returns

???

## `ipfs_disconnect` / `ipfsDisconnect`

Disconnects from the given Multiaddr; the off-chain worker interval for this activity is also every
block. You can try disconnecting from multiaddr from the previous item.

### Arguments

### Returns

## `ipfs_dht_findpeer` / `ipfsDhtFindPeer`

Performs a search for the addresses associated with the provided PeerId; the off-chain worker 
interval for this activity is every block.

### Arguments

If you use the peerID from two items prior, QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ, it will return /ip4/104.131.131.82/tcp/4001.

### Returns

## `ipfs_dht_findproviders` / `ipfsDhtFindProviders`

Search for PeerIds known to be providing the given Cid; the off-chain worker interval for this
activity is every block. Try it with the hash of the IPFS CID Inspector,
QmY7Yh4UquoXHLPFo2XbhXkhBvFoPwmQUSa92pxnxjQuPU.

Make sure you’re connected to at least one peer though!

### Arguments

### Returns

## `ipfs_insert_pin` / `ipfsInsertPin`

Pins a block with the specified Cid, making it persistent; a pinned block can’t be removed.
Try pinning the 1234 CID above: QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa

### Arguments

### Returns

## `ipfs_remove_pin` / `ipfsRemovePin`

Removes a pin from a block, i.e. unpin it, so that it is no longer persistent and can be removed
too. Try removing the pin from the previous item.

### Arguments

### Returns

## `ipfs_remove_block` / `ipfsRemoveBlock`

Removes a block from the node’s repository

### Arguments

### Returns
