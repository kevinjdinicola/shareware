use std::collections::HashMap;
use std::fmt::Debug;
use crate::util::*;

#[derive(Debug)]
pub struct Identity {
    display_name: String,
    display_image: Option<Hash>,
    // what this struct was decoded from.  It's probably a collection blob.  This is just to reference what data I have
    identity_blob: Hash,
    // I can check this doc for the latest content identity blobs.  This shouldn't change?
    identity_doc: Hash,
    // This is a persons ID
    pk: PublicKey,
}
impl Identity {}



impl Identity {
    pub fn new(display_name: String, pk: PublicKey) -> Identity {
        Self {
            display_name,
            display_image: None,
            identity_blob: Hash(String::new()),
            identity_doc: Hash(String::new()),
            pk,
        }
    }
    pub fn identity_doc(&self) -> &Hash {
        &self.identity_doc
    }

}

#[derive(Debug)]
pub struct MemoryIdentityStore {
    known_idens: HashMap<PublicKey, Identity>
}

impl MemoryIdentityStore {
    pub fn default() -> MemoryIdentityStore {
        Self {
            known_idens: HashMap::new()
        }
    }
}

impl IdentityStore for MemoryIdentityStore {
    fn get_by_pk(&self, pk: &PublicKey) -> Option<&Identity> {
        self.known_idens.get(pk)
    }

    fn list_all_identities(&self) -> Box<dyn Iterator<Item=&Identity> + '_> {
        Box::new(self.known_idens.values())
    }

    fn add_identity(&mut self, identity: Identity) {
        self.known_idens.insert(identity.pk.clone(), identity);
    }
}

pub trait IdentityStore: Debug {
    fn get_by_pk(&self, pk: &PublicKey) -> Option<&Identity>;
    fn list_all_identities(&self) -> Box<dyn Iterator<Item = &Identity> + '_>;
    fn add_identity(&mut self, identity: Identity);
}