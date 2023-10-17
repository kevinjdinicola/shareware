use std::process::id;
use crate::identity::{Identity, IdentityStore, MemoryIdentityStore};
use crate::profile::ProfileStore;
use crate::util::PublicKey;

mod identity;
mod util;
mod content;
mod profile;
mod exchange;


fn main() {
    let mut store: MemoryIdentityStore = MemoryIdentityStore::default();

    do_a_thing(store);


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


fn do_a_thing<S: IdentityStore>(mut store: S)
{
    let mut profile_store = ProfileStore::new();
    let profile = profile_store.create_profile();

    let identity = Identity::new("kevin".into(), PublicKey("1234".into()), profile.doc_id().clone());
    // this causes identity to get hash
    store.add_identity(identity);


    let updated_p = profile_store.set_owner(profile, identity.identity_blob().clone());

    println!("profile is {:?}", updated_p);

    // println!("i have {} identities", store.list_all_identities().count());
    // println!("its {:?}", store);
}
