use std::collections::HashMap;
use crate::content::Content;
use crate::util::PublicKey;

struct Exchange {
    peer_data: HashMap<PublicKey, Vec<Content>>,
}