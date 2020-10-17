# Substrate core modifications

Every one of `offchain::ipfs`'s functional modifications to the [`paritytech/substrate`] core are
encapsulated in a single commit on the [`offchain_ipfs`] branch of our repo.

In this section we'll walk through these modifications so that you may understand them, and
perhaps improve upon them yourself.

[`paritytech/substrate`]: https://github.com/paritytech/substrate
[`offchain_ipfs`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs

## How Substrate is organized

Substrate, as a modular framework, provides:

1. **Clients** are services that interact with the substrate blockchain, e.g. substrate full and
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
   the [full client] and [light client] to power the IPFS node.
3. The client will use [`IpfsApi`] and [`IpfsWorker`] to expose its functionality
   to any pallets that wish to access it, via [`ipfs_request_start`] and [`ipfs_request_wait`].
   These calls will utilize the types explained below.
4. If your node is configured to processes user input via a pallet, then users will make requests,
   typically in the form of **[extrinsics]** called via JSON-RPC. A typical successful call goes
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

[`IpfsApi`]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs/client/offchain/src/api/ipfs.rs#L69
[`IpfsWorker`]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs/client/offchain/src/api/ipfs.rs#L249
[`ipfs_request_start`]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs/client/offchain/src/api.rs#L189
[`ipfs_request_wait`]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs/client/offchain/src/api.rs#L193
[extrinsics]: hthttps://github.com/rs-ipfs/substrate/blob/offchain_ipfs/client/offchain/src/api/ipfs.rs#L249tps://substrate.dev/docs/en/knowledgebase/learn-substrate/extrinsics
[full client]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs_docker/client/service/src/builder.rs#L266
[light client]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs_docker/client/service/src/builder.rs#L333
[`node`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs_docker/bin/node/cli
[`node-template`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs_docker/bin/node-template

## Primitive Types

Offchain::ipfs adds the following types used in the substrate runtime and the offchain workers. It
is useful to explore some core types as they indicate the existing offchain::ipfs functions and
where to modify to add new functions to interact with IPFS.

> **Development Tip:**
>
> In fact, you can let the `rustc` do a lot of work for you! By simply adding, changing, or
> removing types from the following enums, you will get helpful errors telling you where else you
> need to change your code to satisfy the compiler.

### `IpfsRequest`

An enum that represents a request to the IPFS node.

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

### `IpfsResponse`

An enum that represents a response from the IPFS node.

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

### `IpfsRequestStatus`

An enum that represents the status of an IPFS request.

- `DeadlineReached` - Deadline was reached while we waited for this request to finish.
- `IoError(Vec<u8>)` - An error has occurred during the request, for example a timeout or the remote
  has closed our socket.
- `Invalid` - The passed ID is invalid in this context.
- `Finished(IpfsResponse)` - The request has finished successfully.

### `IpfsError`

An enum that enumerates types of errors returned from the IPFS node.

- `DeadlineReached = 1` - The requested action couldn't been completed within a deadline.
- `IoError = 2` - There was an IO Error while processing the request.
- `Invalid = 3` - The ID of the request is invalid in this context.

We hope this has been a helpful overview of the core modifications that we made to Substrate in
order to enable embedded IPFS functionality. If you have questions, or if you're ready to jump in
and contribute, or if you're ready to build your own `offchain::ipfs`-powered dApp, read on.
