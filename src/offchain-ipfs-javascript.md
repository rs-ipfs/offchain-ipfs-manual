# In JavaScript

This should work in both node.js and the browser via a bundler like Webpack or Parcel.

## With polkadot.js

```bash
yarn init
yarn add @polkadot/api
```

```javascript
// Import
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

;(async () => {
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({
    provider,
    types: {
      ConnectionCommand: 'ConnectionCommand',
      DataCommand: 'DataCommand',
      DhtCommand: 'DhtCommand',
      Address: 'AccountId',
      LookupSource: 'AccountId'
    }
  });

  await api.isReady;

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const module = api.tx.templateModule

  const PEER_ID = 'QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ'
  const PEER_MULTIADDR = `/ip4/104.131.131.82/tcp/4001/p2p/${PEER_ID}`
  const CID_1234 = 'QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa'

  // Connect to a peer
  // await module.ipfsConnect(PEER_MULTIADDR).signAndSend(alice, logEvents)

  // Add data as bytes (string or raw) to IPFS
  // await module.ipfsAddBytes('1234').signAndSend(alice, logEvents)

  // Cat (retrieve) data from IPFS
  // await module.ipfsCatBytes(CID_1234).signAndSend(alice, logEvents)

  // Disconnect from a peer
  // await module.ipfsDisconnect(PEER_MULTIADDR).signAndSend(alice, logEvents)

  // Locate a peer via the distributed hash table
  // await module.ipfsDhtFindPeer(PEER_ID).signAndSend(alice, logEvents)

  // Locate a peer that has the content you're seeing via the DHT
  // await module.ipfsDhtFindPeer(CID_1234).signAndSend(alice, logEvents)
})()

const logEvents = ({ status, events }) => {
  if (status.isInBlock) {
    console.log(`included in ${status.asInBlock}`);
  }

  if (status.isInBlock || status.isFinalized) {
    events.forEach((record) => {
      const { event, phase } = record;
      const types = event.typeDef;

      console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
      console.log(`\t\t${event.meta.documentation.toString()}`);

      event.data.forEach((data, index) => {
        console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
      });
    })
  }
}
```

### Debugging JSON-RPC in the browser

You will connect to the blockchain node via JSON-RPC over WebSockets., On page load it will
connect to port 9944 via WebSockets.

One good way to monitor the streaming results in the browser is to use devtools:
