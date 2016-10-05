extern crate sha1;

pub trait Object {
    fn get_hash_content(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        let mut sha = sha1::Sha1::new();
        sha.update(&self.get_hash_content());
        sha.digest()
    }
}
