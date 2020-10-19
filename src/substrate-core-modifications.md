# Substrate core modifications

Every one of `offchain::ipfs`'s functional modifications to the [`paritytech/substrate`] core are
encapsulated in a single commit on the [`offchain_ipfs`] branch of our repo.

In this section we'll walk through these modifications so that you may understand them, and
perhaps improve upon them yourself.

[`paritytech/substrate`]: https://github.com/paritytech/substrate
[`offchain_ipfs`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs

## How is Substrate itself organized?

Substrate, as a modular framework, provides:

1. Basic **primitives** to compose a blockchain with your desired features
2. **Clients** (full and light) that allows you to interact with your Substrate blockchain
3. Several **binaries**, both necessary and optional, that you can run or build from source

There's definitely a lot more to Substrate, but for the purposes of this explanation we'll only
cover the parts that `offchain::ipfs` augments. At a very high level, we modeled this implementation
after the existing [`offchain::http`] module. You can look that over to get a sense of how it all
works, or read on for more detail.

[`offchain::http`]: https://github.com/paritytech/substrate/blob/master/client/offchain/src/api/http.rs

## `offchain::ipfs` lifecycle

1. User runs one of the binaries, `node`, or `node-template`
    1. That binary includes, in both the full and light clients, an extra `tokio` runtime that
runs a Rust IPFS node alongside your typical Substrate node runtime.
2. If the offchain worker is enabled in the configuration, the IPFS runtime will start
3. At the same time, the Offchain worker starts, with its own IPFS API embedded within it.

### Primitives

#### Core

- `primitives/core/src/offchain/testing.rs`
    - Adds `IpfsRequest`, `IpfsRequestId`, `IpfsRequestStatus`, `IpfsResponse`
    - Defines `IpfsPendingRequest`
    - Adds pending `ipfs_requests` and `expected_ipfs_requests` to `OffchainState`
    - Defines its own `ipfs_request_start` and `ipfs_response_wait`
- `primitives/core/src/offchain/mod.rs`
    - Defines `IpfsRequest` and `IpfsResponse` enums
    - What is IPFS = 254, Capability, ok.. what does it mean to have 255?
    - Defines its own `ipfs_request_start` and `ipfs_response_wait`
        - On Externalities trait of Box<T> and LimitedExternalities
        - uses `Capability::Ipfs`

#### Runtime

- `primitives/runtime/src/offchain/ipfs.rs`
    - high-level helpers for making IPFS requests from Offchain Workers.
    - Defines `PendingRequest` and `Response` and how the former becomes the latter (or an Error)

#### I/O

- `primitives/io/src/lib.rs`
    - Adds `IpfsRequest`, `IpfsRequestId`
    - Defines the `ipfs_request_start`, and `ipfs_response_wait` functions

### Client

- `client/service/src/builder.rs`
    - Adds a new async runtime here for ipfs to both the full and light clients
- `client/service/src/task_manager/mod.rs`
    - adds public `ipfs_rt` field to the task manager

#### Offchain Client

Lots of stuff here too

- `client/offchain/Cargo.toml`
    - Adds tokio
- `client/offchain/src/api/ipfs.rs`
    - `IpfsApi` and `IpfsWorker`
    - Big part of the implementation here
- `client/offchain/src/api.rs`
    - uses `IpfsRequest`, `IpfsRequestId`, and `IpfsRequestStatus`
    - defines `ipfs_request_start`
    - Includes ipfs
    - Runs the node API
- `client/offchain/src/lib.rs`

### Binaries

#### `node`

- `bin/node/cli/src/service.rs`
    - integrates `task_manager.ipfs_rt` from client

#### `node-template`

- `bin/node-template/node/src/service.rs`
    - integrates `task_manager.ipfs_rt` from client

### Types

#### IpfsRequestId (pub u16)

Type alias for `pub u16`

#### IpfsError

- `DeadlineReached = 1` - The requested action couldn't been completed within a deadline.
- `IoError = 2` - There was an IO Error while processing the request.
- `Invalid = 3` - The ID of the request is invalid in this context.

#### IpfsRequestStatus
pf
/// Deadline was reached while we waited for this request to finish.
	///
	/// Note the deadline is controlled by the calling part, it not necessarily
	/// means that the request has timed out.
	DeadlineReached,
	/// An error has occurred during the request, for example a timeout or the
	/// remote has closed our socket.
	///
	/// The request is now considered destroyed. To retry the request you need
	/// to construct it again.
	IoError(Vec<u8>),
	/// The passed ID is invalid in this context.
- `Invalid`
	/// The request has finished successfully.
- `Finished(IpfsResponse)`

#### `IpfsRequest`

An enum of request types. Note that some of these are not yet implemented as
callables or exposed functionality.

##### Variants

- `Addrs` - Get the list of node's peerIds and addresses.
- `AddBytes(Vec<u8>)` - Add the given bytes to the IPFS repo
- `AddListeningAddr(OpaqueMultiaddr)` - Add an address to listen on.
- `BitswapStats` - Get the bitswap stats of the node.
- `CatBytes(Vec<u8>)` - Get bytes with the given Cid from the IPFS repo and display them.
- `Connect(OpaqueMultiaddr)` - Connect to an external IPFS node with the specified Multiaddr.
- `Disconnect(OpaqueMultiaddr)` - Disconnect from an external IPFS node with the specified Multiaddr.
- `GetBlock(Vec<u8>)` - Obtain an IPFS block.
- `FindPeer(Vec<u8>)` - Find the addresses related to the given PeerId.
- `GetClosestPeers(Vec<u8>)` - Get a list of PeerIds closest to the given PeerId.
- `GetProviders(Vec<u8>)` - Find the providers for the given Cid.
- `Identity` - Get the node's public key and dedicated external addresses.
- `InsertPin(Vec<u8>, bool)` - Pins a given Cid recursively or directly (non-recursively)
- `LocalAddrs` - Get the list of node's local addresses.
- `LocalRefs` - Get the list of `Cid`s of blocks known to a node.
- `Peers` - Obtain the list of node's peers.
- `Publish` - Publish a given message to a topic.
    - `topic: Vec<u8>` - The topic to publish the message to.
    - `message: Vec<u8>` - The message to publish.
- `RemoveBlock(Vec<u8>)` - Remove a block from the ipfs repo. A pinned block cannot be removed.
- `RemoveListeningAddr(OpaqueMultiaddr)` - Remove an address that is listened on.
- `RemovePin(Vec<u8>, bool)` - Unpins a given Cid recursively or only directly.
- `Subscribe(Vec<u8>)` - Subscribe to a given topic.
- `SubscriptionList` - Obtain the list of currently subscribed topics.
- `Unsubscribe(Vec<u8>)` - Unsubscribe from a given topic.


#### `IpfsResponse`

##### Variants

- `Addrs(Vec<(Vec<u8>, Vec<OpaqueMultiaddr>)>)` - A list of pairs of node's peers and
their known addresses.
- `AddBytes(Vec<u8>)` - The Cid of the added bytes.
- `BitswapStats` - A collection of node stats related to the bitswap protocol.
    - `blocks_sent: u64` - The number of blocks sent.
    - `data_sent: u64` - The number of bytes sent.
    - `blocks_received: u64` - The number of blocks received.
    - `data_received: u64` - The number of bytes received.
    - `dup_blks_received: u64` - The number of duplicate blocks received.
    - `dup_data_received: u64` - The number of duplicate bytes received.
    - `peers: Vec<Vec<u8>>` - The list of peers.
    - `wantlist: Vec<(Vec<u8>, i32)>` - The list of wanted CIDs and their bitswap priorities.
- `CatBytes(Vec<u8>)` - The data received from IPFS.
- `FindPeer(Vec<OpaqueMultiaddr>)` - A list of addresses known to be related to a PeerId.
- `GetClosestPeers(Vec<Vec<u8>>)` - The list of PeerIds closest to the given PeerId.
- `GetProviders(Vec<Vec<u8>>)` - A list of PeerIds known to provide the given Cid.
- `Identity(Vec<u8>, Vec<OpaqueMultiaddr>)` - The local node's public key and the externally visible and listened to addresses.
- `LocalAddrs(Vec<OpaqueMultiaddr>)` - A list of local node's externally visible and listened to addresses.
- `LocalRefs(Vec<Vec<u8>>)` - A list of locally available blocks by their Cids.
- `Peers(Vec<OpaqueMultiaddr>)` - The list of currently connected peers.
- `RemoveBlock(Vec<u8>)` - The Cid of the removed block.
- `Success` - A request was processed successfully and there is no extra value to return.
