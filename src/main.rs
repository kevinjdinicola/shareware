use std::process::id;
use crate::identity::{Identity, IdentityStore, MemoryIdentityStore};
use crate::util::PublicKey;

mod identity;
mod util;


fn main() {
    let mut store: MemoryIdentityStore = MemoryIdentityStore::default();
    let identity = Identity::new("kevin".into(), PublicKey("1234".into()));
    IdentityStore::add_identity(&mut store, identity);
    do_a_thing(store);


}

struct Content {
    text_data: String
}

struct ContentStore;

impl ContentStore {
    pub fn get_content_for_identity(identity: &Identity) {
        let doc = identity.identity_doc();
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


fn do_a_thing<S: IdentityStore>(mut store: S)
{
    let identity = Identity::new("kevin".into(), PublicKey("1234".into()));
    store.add_identity(identity);
    let identity = Identity::new("elena".into(), PublicKey("9382".into()));
    store.add_identity(identity);

    println!("i have {} identities", store.list_all_identities().count());
    println!("its {:?}", store);
}
