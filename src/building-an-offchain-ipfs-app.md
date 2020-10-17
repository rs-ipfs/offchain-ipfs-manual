# Building an `offchain::ipfs` app

In this section we’ll cover the essentials on what you'll need to start building your
application with `offchain::ipfs`. We do so by way of:

1. Walking you through the example Substrate pallet that's included with `offchain::ipfs`
2. Providing example code from two Rust clients (`substrate-subxt` and
`substrat-api-client`), and one JavaScript client (`polkadot.js`).

We expect feedback on this pallet, but also we hope that the reference implementation will inspire
builders to create their own pallets, expose their own JSON-RPC endpoints, and call them from
their applications similarly.

## How it all works

It helps, first, to have a basic understanding of how a request flows from a user of your
application, through the Substrate offchain-worker, to the native runtime, over to IPFS,
and then all the way back up to your application again.

1. Once the chain is initialized or blocks are synced, the embedded Rust IPFS
node is launched and connected to the offchain worker runtime. It will stay running
in the background.
2. The user makes a JSON-RPC call to submit an extrinsic to the node's runtime, using
the callable functions exposed from the custom pallet.
3. The request is added to the relevant queue in the Substrate storage database.
This is also defined in the custom pallet.
4. Upon import of specified blocks, the node's runtime passes the requests from the queues to an
offchain worker.
5. The offchain worker relays the desired requests to the Rust IPFS node, and
the node returns futures resolving to results
6. The offchain worker registers the results and relays them to the substrate
runtime, which processes them and acts upon them as specified in the custom pallet.
7. The offchain worker stops running
