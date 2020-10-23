# The architecture of `offchain::ipfs`

As we explained in the introduction, `offchain::ipfs` is currently a fork of
[`paritytech/substrate`] maintained by [Equilibrium](https://equilibrium.co).

There are three branches of note:

- [`master`] - which will always follow `paritytech/substrate` in lock-step, with no modifications
- [`offchain_ipfs`] - which contains the modifications, and periodically rebases from `master`
- [`offchain_ipfs_docker`] - rebases from `offchain_ipfs` and contains the `Dockerfile` updates

There may be other branches at any given tie for pragmatic purposes, but the three above should
always exist and be suitable for their respective purposes.

In the rest of this chapter, we'll show you how to build the code in these branches, and then we'll
take a closer look at what modifications were made to substrate to achieve `offchain::ipfs`.

[`master`]: https://github.com/rs-ipfs/substrate
[`offchain_ipfs`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs
[`offchain_ipfs_docker`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs_docker
[`paritytech/substrate`]: https://github.com/paritytech/substrate
[`offchain_ipfs_bleeding_edge`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs_bleeding_edge
