use iroh::bytes::Hash;
use crate::content::Content;

use iroh::sync::{Namespace, NamespaceId, AuthorId, AuthorPublicKey};
use iroh::sync::store::memory::Store as DocStore;
use iroh::sync::store::Store;
use crate::identity::Identity;
use anyhow::Result;


#[derive(Eq, Hash, PartialEq, Debug, Clone)]

pub struct Profile {
    pub doc_id: NamespaceId,
    pub owner: Option<Hash>, // identity cblob
    pub posts: Vec<Content>, // list of content blobs
    pub inbox: Option<NamespaceId>
}

impl Profile {
    pub fn doc_id(&self) -> &NamespaceId {
        &self.doc_id
    }
}

pub struct ProfileStore {
    store: DocStore
}


impl ProfileStore {
    pub fn new(store: DocStore) -> ProfileStore {
        // probably need to accept a store reference
        Self {
            store
        }
    }

    pub fn create_profile(&mut self) -> Result<Profile> {
        // actually make it in iroh first
        let mut rng = rand::thread_rng();
        let n = Namespace::new(&mut rng);
        let p = self.store.new_replica(n)?;

        Ok(Profile {
            doc_id: p.namespace(),
            owner: None,
            posts: vec![],
            inbox: None,
        })
    }

    pub fn assign_owner(&mut self, mut profile: Profile, identity: Identity) -> Result<Profile> {
        // actually set it in iroh
        let doc_id = profile.doc_id().clone();
        let aid: AuthorId = identity.pk().clone().into();
        let author = self.store.get_author(&aid).unwrap().expect("Cant resolve author for identity");

        let r = self.store.open_replica(&doc_id)?.expect("Document doesn't exist");
        let res = r.insert("owner", &author, identity.identity_blob().clone(), identity.identity_blob_size);

        profile.owner = Some(identity.identity_blob().clone());
        Ok(profile)
    }

    pub fn link_content(&mut self, profile: &Profile, content: &Content) {

    }
}