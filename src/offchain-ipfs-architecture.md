# The architecture of `offchain::ipfs`

As we explained in the introduction, `offchain::ipfs` is currently a fork of
[`paritytech/substrate`] maintained by [Equilibrium](https://equilibrium.co). This comes
in the form of the [`offchain_ipfs_bleeding_edge`] branch, continually updated and rebased
against the upstream repo.

You can see it for yourself in the branch's latest commit, but it might be better if we break it
down here and explain piece by piece:

[`paritytech/substrate`]: https://github.com/paritytech/substrate
[`offchain_ipfs_bleeding_edge`]: https://github.com/rs-ipfs/substrate/tree/offchain_ipfs_bleeding_edge
