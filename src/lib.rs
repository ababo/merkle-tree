extern crate digest;
extern crate generic_array;
extern crate sha2;

pub mod merkle_tree;

#[cfg(test)]
mod tests {
    use generic_array::GenericArray;
    use merkle_tree::MerkleTree;
    use sha2::{Digest, Sha256};

    type Hasher = Sha256;
    type Hash = GenericArray<u8, <Hasher as Digest>::OutputSize>;

    fn hash(str: &str) -> Hash {
        let mut digest = Hasher::default();
        digest.input(str.as_bytes());
        digest.result()
    }

    #[test]
    fn sanity() {
        let blocks = [
            "Ten little nigger boys went out to dine;",
            "One choked his little self, and then there were nine.",
            "Nine little nigger boys sat up very late;",
            "One overslept himself, and then there were eight.",
            "Kight little nigger boys travelling in Devon;",
            "One said he´d stay there, and then there were seven.",
            "Seven little nigger boys chopping up sticks;",
            "One chopped himself in half, and then there were six.",
            "Six little nigger boys playing with a hive;",
            "A bumble-bee stung one, and then there were five.",
            "Five little nigger boys going in for law;",
            "One got in chancery, and then there were four.",
            "Four little nigger boys going out to sea;",
            "A red herring swallowed one, and then there were three.",
            "Three little nigger boys walking in the Zoo;",
            "A big bear hugged one, and then there were two.",
            "Two little nigger boys sitting in the sun;",
            "One got frizzled up, and then there was one.",
            "One little nigger boy left all alone;",
            "He went out and hanged himself and then there were None.",
        ];

        let heights = [
            1, 2, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6
        ];

        let mut tree = MerkleTree::<Hasher>::new();
        assert_eq!(tree.num_blocks(), 0);
        assert_eq!(tree.height(), 0);
        assert!(!tree.is_sealed());

        for i in 0..blocks.len() {
            tree.add_block(hash(blocks[i]));
            assert_eq!(tree.num_blocks(), i + 1);
            assert_eq!(tree.height(), heights[i]);
            assert!(tree.contains_block(hash(blocks[i])));
        }

        tree.seal();
        assert!(tree.is_sealed());

        assert_eq!(format!("{:X}", tree.root_hash()),
            "42566FFE6175DF657034EB9ED55C955DA177769179991A1A08DCE7CAC0800742");
    }
}
