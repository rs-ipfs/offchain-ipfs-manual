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
    let cid = String::from("QmRgctVSR8hvGRDLv7c5H7BCji7m1VXRfEE1vW78CFagD7").into_bytes();
    // Example multiaddr to connect IPFS with
    let multiaddr = String::from(
        "/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
    )
    .into_bytes();
    // Example Peer Id
    let peer_id = String::from("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ").into_bytes();

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

    // ipfs_cat_bytes
    let cat_bytes = client
        .watch(CatBytesCall { cid: cid.clone() }, &signer)
        .await?;
    println!("\nResult for ipfs_cat_bytes: {:?}", cat_bytes);

    // ipfs_connect
    let connect = client
        .watch(
            ConnectCall {
                multiaddr: multiaddr.clone(),
            },
            &signer,
        )
        .await?;
    println!("\nResult for ipfs_connect: {:?}", connect);

    // ipfs_insert_pin
    let insert_pin = client
        .watch(InsertPinCall { cid: cid.clone() }, &signer)
        .await?;
    println!("\nResult for ipfs_insert_pin: {:?}", insert_pin);

    // ipfs_dht_find_peer
    let find_peer = client.watch(FindPeerCall { peer_id }, &signer).await?;
    println!("\nResult for ipfs_dth_find_peer: {:?}", find_peer);

    // ipfs_dht_find_providers
    let find_providers = client
        .watch(FindProvidersCall { cid: cid.clone() }, &signer)
        .await?;
    println!("\nResult for ipfs_dth_find_providers: {:?}", find_providers);

    // ipfs_remove_pin
    let remove_pin = client.watch(RemovePinCall { cid }, &signer).await?;
    println!("\nResult for ipfs_remove_pin: {:?}", remove_pin);

    // ipfs_disconnect
    let disconnect = client.watch(DisconnectCall { multiaddr }, &signer).await?;
    println!("\nResult for ipfs_disconnect: {:?}", disconnect);

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

#[derive(Encode)]
pub struct CatBytesCall {
    cid: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for CatBytesCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_cat_bytes";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

#[derive(Encode)]
pub struct InsertPinCall {
    cid: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for InsertPinCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_insert_pin";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

#[derive(Encode)]
pub struct RemovePinCall {
    cid: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for RemovePinCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_remove_pin";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

#[derive(Encode)]
pub struct ConnectCall {
    multiaddr: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for ConnectCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_connect";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

#[derive(Encode)]
pub struct DisconnectCall {
    multiaddr: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for DisconnectCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_disconnect";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

#[derive(Encode)]
pub struct FindPeerCall {
    peer_id: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for FindPeerCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_dht_find_peer";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}

#[derive(Encode)]
pub struct FindProvidersCall {
    cid: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for FindProvidersCall {
    const MODULE: &'static str = "TemplateModule";
    const FUNCTION: &'static str = "ipfs_dht_find_providers";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}
