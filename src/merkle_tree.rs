use digest::Digest;

#[derive(Default)]
pub struct MerkleTree<T: Digest> {
    digest: T,
}

impl<T: Digest+Default> MerkleTree<T> {
    pub fn new() -> MerkleTree<T> {
        MerkleTree{ ..Default::default() }
    }
}