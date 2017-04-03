extern crate digest;
extern crate sha2;

mod merkle_tree;

use sha2::Sha256;
use merkle_tree::MerkleTree;

fn main() {
	let mut mt = MerkleTree::<Sha256>::new();
    println!("Hello, world!");
}
