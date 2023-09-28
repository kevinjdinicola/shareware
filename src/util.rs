#[derive(Debug)]
pub struct Hash(pub String);

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct PublicKey(pub String);

impl PublicKey {
    pub fn from_str(str: String) -> PublicKey {
        Self(str)
    }
}