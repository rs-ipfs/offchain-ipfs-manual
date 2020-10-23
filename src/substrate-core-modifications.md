# Substrate core modifications

Every one of `offchain::ipfs`'s functional modifications to the [`paritytech/substrate`] core are
encapsulated in a single commit on the [`offchain_ipfs`] branch of our repo.

In this section we'll walk through these modifications so that you may understand them, and
perhaps improve upon them yourself.

[`paritytech/substrate`]: https://github.com/paritytech/substrate
[`offchain_ipfs`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs

## How Substrate is organized

Substrate, as a modular framework, provides:

1. **Clients** any services that interact with the substrate blockchain, e.g. substrate full and
   light client. Offchain workers are one of those clients
2. Basic **primitives** to compose a blockchain with your desired features
3. Several **binaries**, both necessary and optional, that you can run or build from source

There's definitely a lot more to Substrate, but for the purposes of this explanation we'll only
cover the parts that `offchain::ipfs` augments. At a very high level, we modeled this implementation
after the existing [`offchain::http`] module. You can look that over to get a sense of how it all
works, or read on for more detail.

From here on, most of the links will be to code points within the [`offchain_ipfs`] branch of the
`offchain::ipfs` repo.

[`offchain::ipfs`]: https://github.com/rs-ipfs/substrate
[`offchain::http`]: https://github.com/paritytech/substrate/blob/master/client/offchain/src/api/http.rs

## `offchain::ipfs` lifecycle

1. User runs one of the binaries, [`node`], or [`node-template`] which launches a Substrate runtime.
2. If the offchain worker is enabled in the configuration, a secondary runtime will start in both
   the [full client] and [light client] to power the IPFS node, `IpfsApi` exposes its functionality
   to any pallets that wish to access it.
3. If your node is configured to processes user input via a pallet, then users will make requests,
   typically in the form of **extrinsics** called via JSON-RPC. A typical successful call goes
   something like:

   1. The JSON-RPC server in the substrate node will recieve the call and it will be the dispatched
      to the relevant pallet.
   1. The pallet may store the calls in a queue to be handled later, or immediate create a valid
      `IpfsRequest` with the call argument(s).
   1. An Offchain Worker starts on each block import to handle the request to the IPFS node with its
      exposed APIs.
   1. When the IPFS node respond to the requests, the response is registered at the APIs as a
      `IpfsResponse`. The offchain worker stops.
   1. The response can be used to update a chain state through signed or unsigned transaction or be
      used in the rest of the call's logic.

[full client]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs_docker/client/service/src/builder.rs#L266
[light client]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs_docker/client/service/src/builder.rs#L333
[`node`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs_docker/bin/node/cli
[`node-template`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs_docker/bin/node-template

### Offchain Client

It may help to understand the lifecycle better by understanding how offchain::ipfs adds ot the
existing offchain client.

Offchain::Ipfs provide additional APIs to the offchain worker defined here:

- `client/offchain/src/api.rs`
  - `ipfs_request_start` takes in the `IpfsRequest` and on success, returns the `IpfsRequestId`.
  - `ipfs_response_wait` takes in a vec of `IpfsRequestIds` and deadline and returns the `IpfsRequestStatus`.
  - The asynchronous part of the Offchain worker API `AsyncAPI` also includes the `IpfsWorker` which
    runs the IPFS node.

The actually handling of those APIs are defined here:

- `client/offchain/src/api/ipfs.rs`
  - `IpfsApi` - helper for the offchain worker API, communicates with the `IpfsWorker` and manages
    requests and responses
  - `IpfsWorker` - the ipfs node engine that process the `IpfsRequest` from the `IpfsApi`

Since the IPFS node runs when the chain starts and not on block import, we add the initialisation of
it as part of the creation of offchain workers manager here:

- `client/offchain/src/lib.rs`

## Primitive Types

Offchain::ipfs defines these primitives used in the substrate runtime and the offchain workers

### Core

- `primitives/core/src/offchain/testing.rs`
  - Adds `IpfsRequest`, `IpfsRequestId`, `IpfsRequestStatus`, `IpfsResponse`
  - Defines `IpfsPendingRequest`
  - Adds pending `ipfs_requests` and `expected_ipfs_requests` to `OffchainState`
  - Defines its own `ipfs_request_start` and `ipfs_response_wait`
- `primitives/core/src/offchain/mod.rs`
  - Defines `IpfsRequest` and `IpfsResponse` enums
  - Adds IPFS as a option to be included in the offchain workers capabilities
  - Adds `ipfs_request_start` and `ipfs_response_wait` to the `Externalities` trait
  - Implements `ipfs_request_start` and `ipfs_response_wait` for `LimitedExternalities` for offchain
    workers with `Capability::Ipfs`

#### Runtime

- `primitives/runtime/src/offchain/ipfs.rs`
  - high-level helpers for making IPFS requests from Offchain Workers.
  - Defines `PendingRequest` and `Response` and how the former becomes the latter (or an Error)

#### I/O

- `primitives/io/src/lib.rs`
  - Adds `IpfsRequest`, `IpfsRequestId`
  - Defines the `ipfs_request_start`, and `ipfs_response_wait` functions

### Core Types Details

It is useful to explore some core types as they indicate the existing offchain::ipfs functions and
where to modify to add new functions to interact with IPFS

#### `IpfsRequest`

An enum of request types. _Tip: this is a good starting point to add / modify the functions of
offchain::ipfs as each variant is matched to an ipfs function and response variant in the offchain
worker api_

##### `IpfsRequest` Variants

- `Addrs` - Get the list of node's peerIds and addresses.
- `AddBytes(Vec&lt;u8&gt;)` - Add the given bytes to the IPFS repo
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

##### `IpfsResponse` Variants

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
- `Identity(Vec<u8>, Vec<OpaqueMultiaddr>)` - The local node's public key and the externally
  visible and listened to addresses.
- `LocalAddrs(Vec<OpaqueMultiaddr>)` - A list of local node's externally visible and listened to addresses.
- `LocalRefs(Vec<Vec<u8>>)` - A list of locally available blocks by their Cids.
- `Peers(Vec<OpaqueMultiaddr>)` - The list of currently connected peers.
- `RemoveBlock(Vec<u8>)` - The Cid of the removed block.
- `Success` - A request was processed successfully and there is no extra value to return.

#### IpfsRequestId (pub u16)

Type alias for `pub u16`

#### IpfsError

- `DeadlineReached = 1` - The requested action couldn't been completed within a deadline.
- `IoError = 2` - There was an IO Error while processing the request.
- `Invalid = 3` - The ID of the request is invalid in this context.

#### IpfsRequestStatus

- `DeadlineReached` - Deadline was reached while we waited for this request to finish.
- `IoError(Vec<u8>)` - An error has occurred during the request, for example a timeout or the remote
  has closed our socket.
- `Invalid` - The passed ID is invalid in this context.
- `Finished(IpfsResponse)` - The request has finished successfully.

## Binaries

### `node`

- `bin/node/cli/src/service.rs`
  - integrates `task_manager.ipfs_rt` from client

### `node-template`

- `bin/node-template/node/src/service.rs`
  - integrates `task_manager.ipfs_rt` from client
- `bin/node-template/pallets/template`
  - adds example template
