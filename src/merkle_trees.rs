use rs_merkle::Hasher;

use crate::md5_hasher::MD5Algorithm;

pub fn demo() {
    let elements = ["a", "b"];
    let mut leaves: Vec<[u8; 16]> =
        elements.iter().map(|x| MD5Algorithm::hash(x.as_bytes())).collect();

    for h in &leaves {
        println!("{:02x?}", h); // prints like: [5e, b6, 3b, ...]
    }
}
