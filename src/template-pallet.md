# Example Pallet

`offchain::ipfs` comes with a showcase **[pallet]**, which is essentially a Rust module that
complies with the requirements to be included within a substrate **runtime**.

> **This pallet is meant only as an example.** We're including it to be helpful for future pallet authors
that want to use the embedded native IPFS node to suit their needs.

If you're familiar with Substrate and the Framework for Runtime Aggregation of Modularized Entities
(FRAME), you can simply view the [source code] for this pallet. Otherwise, read on as we go through
the code step by step.

Please note the order in which these concepts are explained here is not necessarily the order that
they appear in the code.

You can also learn more by following the [Building a Custom Pallet] tutorial.

[Building a Custom Pallet]: https://substrate.dev/docs/en/tutorials/build-a-dapp/pallet
[source code]: https://github.com/rs-ipfs/substrate/blob/offchain_ipfs_bleeding_edge/bin/node-template/pallets/template/src/lib.rs
[pallet]: https://substrate.dev/docs/en/knowledgebase/runtime/pallets

## Prelude

We start by using items from the native runtime. Our pallet is `no_std` since we're targeting `Wasm`

```rust,ignore
#![cfg_attr(not(feature = "std"), no_std)]
// ...
use sp_core::offchain::{
  Duration, IpfsRequest, IpfsResponse, OpaqueMultiaddr, Timestamp
};
// ...
use sp_runtime::offchain::ipfs;
```

## Command Types

When your JSON-RPC calls are received by the pallet, the requests are
expressed as `___Command` enums and stored in the off-chain worker storage as
a queue, to be ingested by your native runtime and passed to IPFS.

Derive attributes are omitted.

```rust,ignore
// Commands involved in peer-to-peer connections
enum ConnectionCommand {
    ConnectTo(OpaqueMultiaddr),
    DisconnectFrom(OpaqueMultiaddr),
}

// Commands that add, remove, pin, unpin, and output data
enum DataCommand {
    AddBytes(Vec<u8>),
    CatBytes(Vec<u8>),
    InsertPin(Vec<u8>),
    RemoveBlock(Vec<u8>),
    RemovePin(Vec<u8>),
}

// Commands that query the distributed hash table (DHT)
// for peers and content
enum DhtCommand {
    FindPeer(Vec<u8>),
    GetProviders(Vec<u8>),
}
```

## The runtime configuration trait

The `system::Trait` trait (not to be confused with the Rust `trait` keyword), allows you to define
which capabilities from the runtime you want to include, and how you want to use them. You can also
"tightly couple" your pallet to other pallets by adding their `Trait`s to your pallet's
inherited trait list.

Here, however, we keep things simple by:

1. Loosely coupling this pallet by leaving out inherited traits
2. Including only the required `Event` type

```rust,ignore
/// The pallet's configuration trait.
pub trait Trait: system::Trait { // Use traits here to tightly couple to runtime
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

Later in the code, we implement some helper functions on the `Module` struct. The function
bodies are omitted for brevity's sake.

```rust,ignore
impl<T: Trait> Module<T> {
    // "Sends" a request to the local IPFS node by adding it to the offchain storage
    fn ipfs_request(req: IpfsRequest, deadline: impl Into<Option<Timestamp>>)
      -> Result<IpfsResponse, Error<T>>

    // Reads from the `ConnectionQueue` and connects / disconnects
    // from desired / undesired peers, respectively
    fn connection_housekeeping() -> Result<(), Error<T>>

    // Reads `FindPeer` and `GetProviders` commands from the `DhtQueue`,
    // and requests their execution from the native runtime
    fn handle_dht_requests() -> Result<(), Error<T>>

    // Reads `AddBytes`, `CatBytes`, `DataCommand`, `RemoveBlock`, `InsertPin`,
    // and `RemovePin` commands from the `DataQueue` and requests their
    // execution from the native runtime.
    fn handle_data_requests() -> Result<(), Error<T>>

    // Logs metadata (the number of connected peers) to the console at the DEBUG log level
    fn print_metadata() -> Result<(), Error<T>>
```

## The `decl_` macros

Pallets included in Substrate runtimes must adhere to the conventions of [FRAME].
In practice, this means you must implement `decl_` macros:

- `decl_module!`
- `decl_event!`
- `decl_storage!`
- `decl_error!`

[FRAME]: https://substrate.dev/docs/en/knowledgebase/runtime/frame

### `decl_storage!`

Here, we define the data that will actually be stored on-chain as **extrinsics**.

Since the offchain-worker can't perform I/O outside of the wasm context, we store our requests as
queues, to be processed on a periodic basis, consumed, and ultimately performed by the native
runtime.

This is where we use the `ConnectionQueue`, `DataQueue`, and `DhtQueue` command types from above.

```rust,ignore
// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        // A list of addresses to connect to and disconnect from.
        pub ConnectionQueue: Vec<ConnectionCommand>;
        // A queue of data to publish or obtain on IPFS.
        pub DataQueue: Vec<DataCommand>;
        // A list of requests to the DHT.
        pub DhtQueue: Vec<DhtCommand>;
    }
}
```

### `decl_event!`

This is where we define what those events are, when they happen, and what they contain.

Once a command is received by the off-chain worker, the corresponding handler from the `decl_module!`
section will "deposit" the event in its storage, to be processed by the native runtime.

```rust,ignore
// The pallet's events
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        ConnectionRequested(AccountId),
        DisconnectRequested(AccountId),
        QueuedDataToAdd(AccountId),
        QueuedDataToCat(AccountId),
        QueuedDataToPin(AccountId),
        QueuedDataToRemove(AccountId),
        QueuedDataToUnpin(AccountId),
        FindPeerIssued(AccountId),
        FindProvidersIssued(AccountId),
    }
);
```

### `decl_module!`

This section, perhaps the most critical section of any given pallet, is where you can define
functions that are exposed via JSON-RPC to client libraries and, by proxy, your users.

In practice, the bulk of what these functions do is to modify the `DataQueue`, `DhtQueue`, and
`ConnectionQueue` storage objects by pushing signed command requests to their respective queues.

Some default types in the functions are omitted, but we've kept the `#[weight]` attributes around.

> The Substrate docs define one unit of weight as "one picosecond of execution time on
fixed reference hardware." These are essentially _time limits_ for block creation, and can be
(indirectly) mapped to transaction fees analogous to something like "gas fees." Read more about
[weights] if you're curious.

[weights]: https://substrate.dev/docs/en/knowledgebase/learn-substrate/weight

```rust,ignore
// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Called at the beginning of every block before any extrinsics. Clears
        // `ConnectionQueue` and `DhtQueue` values every block, and clears
        // `DataQueue` every other block, since they should have been processed
        // Returns a weight of 0
        fn on_initialize(block_number: T::BlockNumber) -> Weight

        // Called at the beginning of every block to create extrinsics.
        // - `connection_housekeeping` and `handle_dht_requests` called every block
        // - `handle_data_requests` is called on every other block
        // - `print_metadata` is called every 5 blocks
        // blocks to alleviate some bandwidth and storage congestion
        fn offchain_worker(block_number: T::BlockNumber)

        /// Mark a `Multiaddr` as a desired connection target.
        #[weight = 100_000]
        pub fn ipfs_connect(origin, addr: Vec<u8>)

        /// Queues a `Multiaddr` to be disconnected
        #[weight = 500_000]
        pub fn ipfs_disconnect(origin, addr: Vec<u8>)

        /// Add arbitrary bytes to the IPFS repository.
        #[weight = 200_000]
        pub fn ipfs_add_bytes(origin, data: Vec<u8>)

        /// Find and output IPFS data pointed to by the given `Cid`
        #[weight = 100_000]
        pub fn ipfs_cat_bytes(origin, cid: Vec<u8>)

        /// Remove bytes from IPFS by `Cid`
        #[weight = 300_000]
        pub fn ipfs_remove_block(origin, cid: Vec<u8>)

        /// Pins a given `Cid` non-recursively.
        #[weight = 100_000]
        pub fn ipfs_insert_pin(origin, cid: Vec<u8>)

        /// Unpins a given `Cid` non-recursively.
        #[weight = 100_000]
        pub fn ipfs_remove_pin(origin, cid: Vec<u8>)

        /// Find addresses associated with the given `PeerId`.
        #[weight = 100_000]
        pub fn ipfs_dht_find_peer(origin, peer_id: Vec<u8>)

        /// Find the list of `PeerId`s known to be hosting the given `Cid`.
        #[weight = 100_000]
        pub fn ipfs_dht_find_providers(origin, cid: Vec<u8>)
    }
}
```

### `decl_error!`

This is where we can define the myriad ways things can go wrong, as an enum.

```rust,ignore
// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        CantCreateRequest,
        RequestTimeout,
        RequestFailed,
    }
}
```

Read on to see examples of how you can make calls to the this example pallet from your application.
