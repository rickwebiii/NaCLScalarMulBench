use std::mem::size_of;
use rand::{thread_rng, RngCore};
use std::time::Instant;

const LEN: usize = 128 * 1024;

fn bench_nacl() {
    use rust_sodium::crypto::scalarmult::curve25519::*;

    let mut g = vec![];
    let mut s = vec![];

    println!("Generating...");

    for _ in 0..LEN {
        let mut buff = [0u8; size_of::<GroupElement>()];
        thread_rng().fill_bytes(&mut buff);
        
        g.push(GroupElement::from_slice(&buff).unwrap());

        let mut buff = [0u8; size_of::<Scalar>()];
        thread_rng().fill_bytes(&mut buff);
        
        s.push(Scalar::from_slice(&buff).unwrap());
    }

    println!("Bench NaCl...");

    let now = Instant::now();

    for i in 0..LEN {
        let _ = scalarmult(&s[i], &g[i]).unwrap();
    }

    println!("{} mul/s", LEN as f64 / now.elapsed().as_secs_f64());
}

fn bench_dalek() {
    use curve25519_dalek_ng::{scalar::Scalar, ristretto::RistrettoPoint};

    let mut p = vec![];
    let mut s = vec![];

    for _ in 0..LEN {
        p.push(RistrettoPoint::random(&mut thread_rng()));
        s.push(Scalar::random(&mut thread_rng()));
    }

    println!("Bench Dalek...");

    let now = Instant::now();

    for i in 0..LEN {
        let _ = p[i] * s[i];
    }

    println!("{} mul/s", LEN as f64 / now.elapsed().as_secs_f64());
}

fn main() {
    bench_nacl();
    bench_dalek();
}
