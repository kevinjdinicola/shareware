use std::collections::HashMap;
use iroh::sync::AuthorPublicKey;
use crate::content::Content;


struct Exchange {
    peer_data: HashMap<AuthorPublicKey, Vec<Content>>,
}