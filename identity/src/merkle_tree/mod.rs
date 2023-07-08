mod common;

pub use common::{CompressH, JubJubMerkleTree, LeafH, Root, SimplePath};

#[test]
fn test_simple_merkle_tree() {
    use ark_crypto_primitives::crh::{CRHScheme, TwoToOneCRHScheme};
    use common::{CompressH, JubJubMerkleTree, LeafH};

    // Let's set up an RNG for use within tests. Note that this is *not* safe
    // for any production use.
    let mut rng = ark_std::test_rng();

    // step 1: given rng, prepare leaf_crh_params
    let leaf_crh_params = <LeafH as CRHScheme>::setup(&mut rng).unwrap();

    // step 2: given rng, prepare two_to_one_crh_params
    let two_to_one_crh_params = <CompressH as TwoToOneCRHScheme>::setup(&mut rng).unwrap();

    // step 3: new tree
    let mut leaves: Vec<Vec<u8>> = Vec::new();
    for i in 0..4u8 {
        let input: Vec<u8> = vec![i; 30];
        leaves.push(input);
    }

    let tree = JubJubMerkleTree::new(
        &leaf_crh_params,
        &two_to_one_crh_params,
        leaves.iter().map(|v| v.as_slice()), // the i-th entry is the i-th leaf.
    )
    .unwrap();

    // Now, let's try to generate a membership proof for the 3rd item.
    let proof = tree.generate_proof(2).unwrap();
    let root = tree.root();
    let result = proof
        .verify(&leaf_crh_params, &two_to_one_crh_params, &root, vec![2; 30])
        .unwrap();
    assert!(result);
}
