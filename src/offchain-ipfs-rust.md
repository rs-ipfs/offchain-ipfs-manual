# Using `offchain::ipfs` from your Rust code

## With `substrate-subxt`

In your Cargo.toml file

```toml
[dependencies]
async-std = { version = "1.6.4", features = ["attributes"] }
codec = { package = "parity-scale-codec", version = "1.3.5", default-features = false, features = ["derive"] }
substrate-subxt = "0.13.0"
sp-keyring = { version = "2.0.0", default-features = false }
```

Then in your main.rs:

```rust
use codec::Encode;
use sp_keyring::AccountKeyring;
use substrate_subxt::{Call, ClientBuilder, EventsDecoder, NodeTemplateRuntime, PairSigner};

#[derive(Encode)]
pub struct AddBytesCall {
    data: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for AddBytesCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_add_bytes";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Alice.pair());
    let client = ClientBuilder::<NodeTemplateRuntime>::new().build().await?;
    let metadata = client.metadata();
    let event_decoder = EventsDecoder::<NodeTemplateRuntime>::new(metadata.clone());

    let add_bytes_call = AddBytesCall {
        data: vec![1, 2, 3, 4],
    };

    let signed_extrinsic = client.create_signed(add_bytes_call, &signer).await?;
    let result = client
        .submit_and_watch_extrinsic(signed_extrinsic, event_decoder)
        .await?;

    println!("Result: {:?}", result);

    Ok(())
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
use substrate_api_client::{
    compose_extrinsic, extrinsic::xt_primitives::UncheckedExtrinsicV4, Api, XtStatus,
};

fn main() {
    // instantiate an Api that connects to the given address
    let url = "127.0.0.1:9944";
    // if no signer is set in the whole program, we need to give to Api a specific type instead of an associated type
    // as during compilation the type needs to be defined.
    let signer = AccountKeyring::Bob.pair();

    // this defaults to use 127.0.0.1:9944
    let api = Api::new(format!("ws://{}", url)).set_signer(signer.clone());

    let bytes_to_add: Vec<u8> = vec![1, 2, 3, 4];
    let xt: UncheckedExtrinsicV4<_> = compose_extrinsic!(
        api.clone(),
        "TemplateModule",
        "ipfs_add_bytes",
        bytes_to_add
    );
    println!("Composed Extrinsic:\n {:?}\n", xt);

    let tx_hash = api
        .send_extrinsic(xt.hex_encode(), XtStatus::Finalized)
        .unwrap();
    println!("Transaction got finalized. Hash: {:?}", tx_hash);
}
```
