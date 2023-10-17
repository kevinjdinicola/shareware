use crate::content::Content;
use crate::util::Hash;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]

struct Profile {
    doc_id: Hash,
    owner_id: Option<Hash>, // identity cblob
    posts: Vec<Content>, // list of content blobs
    inbox: Option<Hash>
}

impl Profile {
    pub fn doc_id(&self) -> &Hash {
        &self.doc_id
    }
}

pub struct ProfileStore;


impl ProfileStore {
    pub fn new() -> ProfileStore {
        // probably need to accept a store reference
        ProfileStore
    }

    pub fn create_profile(&mut self) -> Profile {
        // actually make it in iroh first
        Profile {
            doc_id: Hash(String::from("hi")),
            owner_id: None,
            posts: vec![],
            inbox: None,
        }
    }

    pub fn set_owner(&mut self, mut profile: Profile, identity_blob: Hash) -> Profile {
        // actually set it in iroh
        profile.owner_id = Some(identity_blob);
        profile
    }

    pub fn link_content(&mut self, profile: &Profile, content: &Content) {

    }
}