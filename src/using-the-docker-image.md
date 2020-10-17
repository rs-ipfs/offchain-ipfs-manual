# Using the Docker image

The recommended way to use `offchain::ipfs` is via the [eqlabs/substrate-ipfs] image.

## Installing the image

```bash
# Pull the image from Docker Hub
$ docker pull eqlabs/substrate-ipfs
```

The image comes with two binaries:

1. The default `node-template` serves a preview pallet and reference implementation
2. `substrate` - the real deal, which you can develop your own pallets against

The image exposes ports `9944` for WebSockets, `9933` for RPC, `30333` for p2p, and `9615` for
Prometheus.

## Running the image

The default command for the image is:

`node-template --ws-external --rpc-external --base-path=/substrate-ipfs --dev`

Run the default like so:

```bash
$ docker run \
    -p 9944:9944 \            # websockets
    -p 9933:9933 \            # rpc
    -p 30333:30333 \          # p2p
    -p 9615:9615 \            # prometheus
    -it \
    --rm \
    --name node-template \
    substrate-ipfs
```

To override the default and run `substrate`, for example:

```bash
$ docker run \
    -p 9944:9944 \
    -p 9933:9933 \
    -p 30333:30333 \
    -p 9615:9615 \
    -it \
    --rm \
    --name sub-ipfs \
    substrate-ipfs \
    substrate                 # Override default command
```

This will work with any arguments you'd normally pass to `substrate`

### Persistent Storage

To run with persistent storage volume between containers, first create a volume:

```bash
$ docker volume create substrate-ipfs-vol
```

Then add `-v substrate-ipfs-vol:/substrate-ipfs` to the docker run commands above.
