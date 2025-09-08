use md5::{Digest, Md5};
use rs_merkle::Hasher;

#[derive(Clone)]
pub struct MD5Algorithm;

impl Hasher for MD5Algorithm {
    type Hash = [u8; 16];

    fn hash(data: &[u8]) -> Self::Hash {
        let mut hasher = Md5::new();
        hasher.update(data);
        <[u8; 16]>::from(hasher.finalize())
    }
}
