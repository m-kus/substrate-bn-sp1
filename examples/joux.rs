extern crate bn;
extern crate rand;
use bn::{Field, Scalar, G1, G2, pairing};

fn main() {
    let rng = &mut rand::thread_rng();

    // Generate private keys
    let alice_sk = Scalar::random(rng);
    let bob_sk = Scalar::random(rng);
    let carol_sk = Scalar::random(rng);

    // Generate public keys in G1 and G2
    let (alice_pk1, alice_pk2) = (G1::one() * &alice_sk, G2::one() * &alice_sk);
    let (bob_pk1, bob_pk2) = (G1::one() * &bob_sk, G2::one() * &bob_sk);
    let (carol_pk1, carol_pk2) = (G1::one() * &carol_sk, G2::one() * &carol_sk);

    // Each party computes the shared secret
    let alice_dh = pairing(&bob_pk1, &carol_pk2) ^ &alice_sk;
    let bob_dh = pairing(&carol_pk1, &alice_pk2) ^ &bob_sk;
    let carol_dh = pairing(&alice_pk1, &bob_pk2) ^ &carol_sk;

    assert!(alice_dh == bob_dh && bob_dh == carol_dh);
}