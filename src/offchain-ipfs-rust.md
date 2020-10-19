# Using `offchain::ipfs` from your Rust code

## With `substrate-subxt`

In your Cargo.toml file

<!-- markdownlint-disable MD013 --> <!-- long line for codec include -->
```toml
[dependencies]
async-std = { version = "1.6.4", features = ["attributes"] }
codec = { package = "parity-scale-codec", version = "1.3.5", default-features = false, features = ["derive"] }
substrate-subxt = "0.13.0"
sp-keyring = { version = "2.0.0", default-features = false }
```
<!-- markdownlint-restore -->

Then in your main.rs:

```rust
use codec::Encode;
use sp_keyring::AccountKeyring;
use substrate_subxt::{Call, ClientBuilder, EventsDecoder, NodeTemplateRuntime, PairSigner};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Signer for the extrinsic
    let signer = PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Alice.pair());
    // API client, default to connect to 127.0.0.1:9944
    let client = ClientBuilder::<NodeTemplateRuntime>::new().build().await?;

    // Example CID for the example bytes added vec![1, 2, 3, 4]
    let cid = String::from("QmRgctVSR8hvGRDLv7c5H7BCji7m1VXRfEE1vW78CFagD7")
        .into_bytes()
        .to_vec();
    // Example multiaddr to connect IPFS with
    let multiaddr = String::from(
        "/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
    )
    .into_bytes()
    .to_vec();
    // Example Peer Id
    let peer_id = String::from("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ")
        .into_bytes()
        .to_vec();

    // Begin to submit extrinsics
    // ipfs_add_bytes
    let add_bytes = client
        .watch(
            AddBytesCall {
                data: vec![1, 2, 3, 4],
            },
            &signer,
        )
        .await?;
    println!("\nResult for ipfs_add_bytes: {:?}", add_bytes);

    Ok(())
}

#[derive(Encode)]
pub struct AddBytesCall {
    data: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for AddBytesCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_add_bytes";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

```

### With `substrate-api-client`

In your Cargo.toml file:

```toml
[dependencies]
substrate-api-client = { git = "https://github.com/scs/substrate-api-client.git" }
sp-core = { version = "2.0.0", features = ["full_crypto"] }
sp-keyring = { version = "2.0.0", default-features = false }
```

Then in your main.rs:

```rust
use sp_core::crypto::Pair;
use sp_keyring::AccountKeyring;
use std::{convert::TryFrom, string::String};
use substrate_api_client::{
    compose_call, compose_extrinsic_offline, extrinsic::xt_primitives::UncheckedExtrinsicV4,
    node_metadata::Metadata, Api, XtStatus,
};

fn main() {
    // instantiate an Api that connects to the given address
    let url = "127.0.0.1:9944";
    // if no signer is set in the whole program, we need to give to Api a specific
    // type instead of an associated type as during compilation the type needs to be defined.
    let signer = AccountKeyring::Bob.pair();

    // sets up api client and retrieves the node metadata
    let api = Api::new(format!("ws://{}", url)).set_signer(signer.clone());
    // gets the current nonce of Bob so we can increment it manually later
    let mut nonce = api.get_nonce().unwrap();

    // data from the node required in extrinsic
    let meta = Metadata::try_from(api.get_metadata()).unwrap();
    let genesis_hash = api.genesis_hash;
    let spec_version = api.runtime_version.spec_version;
    let transaction_version = api.runtime_version.transaction_version;

    // Example bytes to add
    let bytes_to_add: Vec<u8> = vec![1, 2, 3, 4];
    // Example CID for the example bytes added vec![1, 2, 3, 4]
    let cid = String::from("QmRgctVSR8hvGRDLv7c5H7BCji7m1VXRfEE1vW78CFagD7")
        .into_bytes()
        .to_vec();

    // Create input for all calls
    let calls = vec![
        ("ipfs_add_bytes", bytes_to_add),
        ("ipfs_cat_bytes", cid.clone()),
    ];

    // Create Extinsics and listen for all calls
    for call in calls {
        println!("\n Creating Extrinsic for {}", call.0);
        let _call = compose_call!(meta, "TemplateModule", call.0, call.1);
        let xt: UncheckedExtrinsicV4<_> = compose_extrinsic_offline!(
            signer,
            _call,
            nonce,
            Era::Immortal,
            genesis_hash,
            genesis_hash,
            spec_version,
            transaction_version
        );

        let blockh = api
            .send_extrinsic(xt.hex_encode(), XtStatus::Finalized)
            .unwrap();
        println!("Transaction got finalized in block {:?}", blockh);
        nonce += 1;
    }
}
```

For full demo with all pallet functions, please visit [here](https://github.com/whalelephant/offchain_ipfs_api_clients)
