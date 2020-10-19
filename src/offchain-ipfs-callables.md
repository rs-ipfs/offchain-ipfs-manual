# offchain::ipfs callables

The following callables are available to you. These were chosen due to popularity and to try
and give substrate uses the most cohesive and familiar experience using IPFS within their node.

- api.tx.templateModule.ipfsAddBytes - add the given bytes to the IPFS repository; the off-chain
worker interval for this activity is every block with an odd BlockNumber (i.e. every other
block). For example adding 1234 gives you the CID QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa
- api.tx.templateModule.ipfsCatBytes - display the bytes (UTF-8 is displayed as a string, while
non-UTF-8 bytes are displayed in their hexadecimal representation) with the given Cid;
the off-chain worker interval for this activity is also every block with an odd BlockNumber.
In the inverse of the last item, requesting the CID above will return 1234
- api.tx.templateModule.ipfsConnect - connect to the given Multiaddr; the off-chain worker
interval for this activity is every block (the queue of connection requests is processed with
every block in the chain). Try it with
/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ.
- api.tx.templateModule.ipfsDisconnect - disconnect from the given Multiaddr; the off-chain worker
interval for this activity is also every block. You can try disconnecting from multiaddr from the
previous item.
- api.tx.templateModule.ipfsDhtFindPeer - perform a search for the addresses associated with the
provided PeerId; the off-chain worker interval for this activity is every block. If you use the
peerID from two items prior, QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ, it will return
/ip4/104.131.131.82/tcp/4001.
- api.tx.templateModule.ipfsDhtFindProviders - search for PeerIds known to be providing the given
Cid; the off-chain worker interval for this activity is every block. Try it with the hash of the
IPFS CID Inspector, QmY7Yh4UquoXHLPFo2XbhXkhBvFoPwmQUSa92pxnxjQuPU. Make sure you’re connected to
at least one peer though!
- api.tx.templateModule.ipfsInsertPin - pin a block with the specified Cid, making it persistent;
a pinned block can’t be removed. Try pinning the 1234 CID above:
QmU1f6ngsoHvwtViihzLQPXCA8j3sagmvY9GJJDY7Ao7Aa
- api.tx.templateModule.ipfsRemovePin - remove a pin from a block, i.e. unpin it, so that it is
no longer persistent and can be removed too. Trey removing the pin from the previous item.
- api.tx.templateModule.ipfsRemoveBlock - remove a block from the node’s repository
