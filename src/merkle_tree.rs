use digest::Digest;
use generic_array::GenericArray;
use std::collections::BTreeSet;
use std::mem::swap;

// In the following code I implement a single struct which uses BTreeSet for
// block ordering. But for production it would be better to divide this struct
// into two parts: MerkleTreeGenerator and MerkleTree. In that case the last
// one should contain a sorted slice instead of expensive BTreeSet.

#[derive(Default)]
pub struct MerkleTree<T: Digest> {
    // It's better to use GenericArray<u8, T::OutputSize> instead of Vec<u8>,
    // but unfortunately it doesn't implement Ord. Let's omit writing a
    // wrapping type or forking that repo for simplicity.
    leaves: BTreeSet<Vec<u8>>,
    // It would be better to use GenericArray<u8, T::OutputSize> instead of T,
    // but the first doesn't implement Default.
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

    pub fn blocks(&self) -> Vec<GenericArray<u8, T::OutputSize>> {
        // a bit of golang-style because of lacking FromIterator support
        let mut result = Vec::<GenericArray<u8, T::OutputSize>>::new();
        for h in self.leaves.iter() {
            let ga = GenericArray::<u8, T::OutputSize>::clone_from_slice(&h);
            result.push(ga);
        }
        result
    }

    pub fn add_block(&mut self, hash: GenericArray<u8, T::OutputSize>) {
        let vec = hash.iter().cloned().collect();
        self.leaves.insert(vec);
    }

    pub fn seal(&mut self) {
        let src = &mut self.leaves.iter().map(|v| {
            let mut digest =T::default();
            digest.input(v);
            digest
        }).collect::<Vec<T>>();
        let dst = &mut Vec::<T>::new();

        for _ in 0..self.height() - 1 {
            for i in 0..src.len() / 2 {
                let mut digest =T::default();
                digest.input(src[2 * i].clone().result().as_slice());
                digest.input(src[2 * i + 1].clone().result().as_slice());
                dst.push(digest);
            }

            if src.len() % 2 == 1 {
                dst.push(src[src.len() - 1].clone());
            }

            swap(dst, src);
            dst.clear();
        }

        self.root = Some(if src.len() != 0 {
            src[0].clone()
        } else {
            T::default()
        });
    }
}
