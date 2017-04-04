use digest::Digest;
use generic_array::GenericArray;
use std::collections::BTreeSet;
use std::mem::swap;

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

        self.root = Some(if dst.len() != 0 {
            dst[0].clone()
        } else {
            T::default()
        });
    }
}
