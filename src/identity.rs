use std::collections::HashMap;
use std::fmt::Debug;
use std::future::IntoFuture;
use std::io::Read;
use std::process::id;
use iroh::bytes::Hash;
use iroh::sync::{AuthorPublicKey, NamespaceId};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use tokio::runtime::{Runtime};
use iroh::bytes::util::runtime::Handle;


use iroh::baomap::mem::Store as BaoStore;
use iroh::bytes::baomap::{PartialMap, Store, TempTag};
use iroh::bytes::util::BlobFormat;
use iroh::collection::{Blob, Collection};
use tokio_util::bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Identity {
    pub name: String,
    pub image: Option<Hash>,
    // I can check this doc for the latest content identity blobs.  This shouldn't change?
    pub profile_doc: NamespaceId,
    // what this struct was decoded from.  It's probably a collection blob.  This is just to reference what data I have
    pub identity_blob: Hash,
    pub identity_blob_size: u64,
    // This is a persons ID
    pub pk: AuthorPublicKey,
}

#[derive(Serialize, Deserialize)]
struct IdentityMetadata {
    pub name: String,
    pub image: String,
    pub profile_doc: String,
    pub pk: String,
    // probably want something in here signing this data with the pk
}

impl Identity {
    pub fn new(name: String, pk: AuthorPublicKey, profile_doc: NamespaceId) -> Identity {
        Self {
            name,
            image: None,
            identity_blob: Hash::EMPTY,
            profile_doc,
            pk,
            identity_blob_size: 0,
        }
    }

    pub fn identity_blob(&self) -> &Hash { &self.identity_blob }
    pub fn pk(&self) -> &AuthorPublicKey { &self.pk }
    pub fn profile_doc(&self) -> &NamespaceId {
        &self.profile_doc
    }

}

#[derive(Debug)]
pub struct MemoryIdentityStore {
    known_idens: HashMap<AuthorPublicKey, Identity>,
    bao_store: BaoStore,
    rt: Handle,
}

impl MemoryIdentityStore {
    pub fn new(bao_store: BaoStore, rt: Handle) -> MemoryIdentityStore {
        Self {
            bao_store,
            rt,
            known_idens: HashMap::new()
        }
    }
}


impl MemoryIdentityStore {
    pub fn get_by_pk(&self, pk: &AuthorPublicKey) -> Option<&Identity> {
        self.known_idens.get(pk)
    }

    pub fn list_all_identities(&self) -> Box<dyn Iterator<Item=&Identity> + '_> {
        Box::new(self.known_idens.values())
    }

    pub async fn add_identity(&mut self, mut identity: Identity) -> Identity {
        // this should upload a new blob

        // create the metadata.json blob
        let im = IdentityMetadata {
            name: identity.name.clone(),
            image: identity.image.unwrap_or(Hash::EMPTY).to_string(),
            profile_doc: identity.profile_doc.into_public_key().unwrap().to_string(),
            pk: identity.pk.to_string(),
        };
        let id_md_bytes = serde_json::to_vec(&im).unwrap();
        let size = id_md_bytes.len();

        let thing = self.bao_store.import_bytes(id_md_bytes.into(), BlobFormat::RAW).await.unwrap();


        println!("hash of metadata.json is {}", thing.hash());

        let weirdbytes = Bytes::from("hehe i have data");
        let total_size = weirdbytes.len() + size;

        let weird = self.bao_store.import_bytes(weirdbytes, BlobFormat::RAW).await.unwrap();

        // normally we could add other data here too...

        //create collection blob
        let blobs: Vec<Blob> = [
            ("metadata.json", thing.hash().clone()),
            ("pic.jpg", weird.hash().clone())
        ]
            .into_iter().map(|pair| { Blob {
            name: String::from(pair.0),
            hash: pair.1
        }}).collect();

        let collection = Collection::new(blobs, total_size as u64).unwrap();
        let col_size = collection.total_blobs_size();
        let collection = collection.store(&self.bao_store).await.unwrap();
        println!("hash of collection is {}", collection.hash());



        identity.identity_blob = collection.hash().clone();
        identity.identity_blob_size = col_size;

        self.known_idens.insert(identity.pk.clone(), identity.clone());

        identity
    }
}

// pub trait IdentityStore: Debug {
//     fn get_by_pk(&self, pk: &AuthorPublicKey) -> Option<&Identity>;
//     fn list_all_identities(&self) -> Box<dyn Iterator<Item = &Identity> + '_>;
//     fn add_identity(&mut self, identity: Identity) -> Identity;
// }
