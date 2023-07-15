use ark_bls12_381::{G1Affine, G1Projective};
use ark_crypto_primitives::crh::{
    pedersen::{Window, CRH},
    CRHScheme,
};
use blake2::{Blake2b512, Digest};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[derive(Clone)]
struct Window4x25 {}

impl Window for Window4x25 {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 256;
}

pub fn hash_to_curve(msg: &[u8]) -> (Vec<u8>, G1Affine) {
    let rng_pedersen = &mut ChaCha20Rng::from_seed([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ]);
    let parameters = CRH::<G1Projective, Window4x25>::setup(rng_pedersen).unwrap();
    let mut hasher = Blake2b512::new();
    hasher.update(msg);
    let b2hash = hasher.finalize();
    (
        b2hash.to_vec(),
        CRH::<G1Projective, Window4x25>::evaluate(&parameters, b2hash).unwrap(),
    )
}
