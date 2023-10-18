use std::process::id;
use crate::identity::{Identity, MemoryIdentityStore};
use crate::profile::{Profile, ProfileStore};
use iroh::sync::{Author, AuthorPublicKey};
use iroh::sync::store::memory::Store as DocStore;
use iroh::sync::store::Store;
use iroh::baomap::mem::Store as BaoStore;
use iroh::bytes::baomap::ReadableStore;
use iroh::bytes::util::BlobFormat;
use iroh::rpc_protocol::ShareMode;
use tracing_subscriber::{prelude::*, EnvFilter};


mod identity;
mod util;
mod content;
mod profile;
mod exchange;

pub fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();

    let iroh_handle = iroh::bytes::util::runtime::Handle::from_current(1)?;

    let bao_store: BaoStore = BaoStore::new(iroh_handle.clone());

    let store: MemoryIdentityStore = MemoryIdentityStore::new(bao_store.clone(), iroh_handle.clone());
    let doc_store: DocStore = DocStore::default();

    // run iroh in the background.  i dont wanna get fucked into async land
    let node = iroh::node::Node::builder(bao_store.clone(), doc_store.clone())
        .runtime(&iroh_handle)
        .spawn().await?;

    let p = make_a_profile(store, doc_store).await;

    let ticket = node.ticket(p.owner.unwrap(), BlobFormat::HASHSEQ).await?;


    let doc = node.client().docs.get(p.doc_id.clone()).await.unwrap().unwrap();
    let dt = doc.share(ShareMode::Write).await.unwrap();
    println!("i have a DOC TICKET {}", dt);




    bao_store.blobs().for_each(|b| {
       println!("i have a blob {}", b);
    });

    // print some info about the node
    println!("serving hash:    {}", ticket.hash());
    println!("node PeerID:     {}", ticket.node_addr().peer_id);
    println!("node listening addresses:");
    for addr in ticket.node_addr().direct_addresses() {
        println!("\t{:?}", addr);
    }
    println!("\t$ cargo run -- get --ticket {}", ticket);
    node.await?;
    Ok(())

}

struct ContentStore;

impl ContentStore {
    pub fn get_content_for_identity(identity: &Identity) {
        let doc = identity.profile_doc();
        // find doc by id
        // get all keys that start with 'content/'
        // get blobs for each key
    }
    pub fn post_content() {
        // takes identity and content
        // make sure i am the author of the doc referred to by the identity
        // create a blob of the content and wrapping collection
        // add a key to my document referencing collection blob
    }
}


async fn make_a_profile(mut identity_store: MemoryIdentityStore, doc_store: DocStore) -> Profile
{
    let mut rng = rand::thread_rng();

    let mut profile_store = ProfileStore::new(doc_store.clone());

    let author = doc_store.new_author(&mut rng).expect("boom");

    // this profile is empty and un-owned right now, but we have a doc reference to it.
    let profile = profile_store.create_profile().expect("boom");



    let identity = Identity::new("kevin".into(), author.public_key(), profile.doc_id().clone());
    // this causes identity to get hash
    let identity = identity_store.add_identity(identity).await;
    // need to save the actual blob


    let updated_p = profile_store.assign_owner(profile, identity.clone()).unwrap();

    println!("identity is {:?}", identity);
    println!("profile is {:?}", updated_p);

    // println!("i have {} identities", store.list_all_identities().count());
    // println!("its {:?}", store);
    updated_p
}
