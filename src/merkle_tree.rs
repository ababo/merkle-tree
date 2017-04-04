use digest::Digest;
use generic_array::GenericArray;
use std::collections::BTreeSet;

// To achieve proper efficiency it would be better to divide this functionality
// into two parts: MerkleTreeGenerator and MerkleTree. The last one should
// contain a sorted slice instead of expensive BTreeSet.

#[derive(Default)]
pub struct MerkleTree<T: Digest> {
    // It would be better to use generic_array::GenericArray<u8, T::OutputSize>
    // instead, but unfortunately it doesn't implement std::cmp::Ord. Let's
    // omit writing a wrapping type or forking that repo for simplicity.
    leaves: BTreeSet<Vec<u8>>,
    // Would be better to use GenericArray<u8, T::OutputSize> instead of T
    // if it implemented std::default::Default.
    root: Option<T>,
}

impl<T: Digest+Default+Clone> MerkleTree<T> {
    pub fn new() -> MerkleTree<T> {
        MerkleTree{ ..Default::default() }
    }

    pub fn is_sealed(&self) -> bool {
        !self.root.is_none()
    }

    pub fn root_hash(&self) -> GenericArray<u8, T::OutputSize> {
        self.root.as_ref().unwrap().clone().result()
    }

    pub fn num_blocks(&self) -> usize {
        self.leaves.len()
    }

    pub fn height(&self) -> usize {
        let len = self.leaves.len();
        match len {
            0 | 1 => len,
            _ => ((len - 1) as f32).log2() as usize + 2,
        }
    }

    pub fn contains_block(
        &self, hash: GenericArray<u8, T::OutputSize>) -> bool {
        let vec:Vec<u8> = hash.iter().cloned().collect();
        self.leaves.contains(&vec)
    }

    pub fn add_block(&mut self, hash: GenericArray<u8, T::OutputSize>) {
        let vec = hash.iter().cloned().collect();
        self.leaves.insert(vec);
    }

    pub fn seal(&mut self) {

    }
}