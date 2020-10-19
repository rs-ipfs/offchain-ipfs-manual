# Building from Source

## Building the docker image from source

```bash
git clone https://github.com/rs-ipfs/substrate
cd substrate
docker build --file .maintain/Dockerfile --tag [your-org/]offchain-ipfs .
```

This is a multistage build with the target being an image with name offchain-ipfs:latest
containing the substrate binary and the node-template binary.

Note that this will take forever

## Running the node from source

``` bash
git clone https://github.com/rs-ipfs/substrate
cd substrate
git checkout offchain_ipfs
cargo build --workspace
```

And get a coffee…

Note that this will take forever, but less time than building in alpine
