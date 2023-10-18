use iroh::bytes::Hash;
use iroh::sync::AuthorPublicKey;
use crate::identity::Identity;

const STRING_CONTENT_TYPE: &str = "text";

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Content {
    content_type: &'static str,
    size: u64,
    data: Option<Vec<u8>>, // if small enough
    data_hash: Hash,
    author_pk: AuthorPublicKey,
    identity: Hash,
    exchange_doc: Option<Hash>,
}

pub struct ContentStore;

impl ContentStore {
    pub fn new() -> ContentStore {
        ContentStore
    }
    pub fn upload_text_content(text: &str, author: &Identity) -> Content {
        // actually upload the blob and get a hash of it here
        Content {
            content_type: STRING_CONTENT_TYPE,
            size: text.len() as u64,
            data: Some(String::from(text).into_bytes()),
            data_hash: Hash::EMPTY,
            author_pk: author.pk().clone(),
            identity: author.identity_blob().clone(),
            exchange_doc: None,
        }
    }
}