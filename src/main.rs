use rust_sodium::crypto::scalarmult::curve25519::*;
use std::mem::size_of;
use rand::{thread_rng, RngCore};
use std::time::Instant;

fn main() {
    let mut g = vec![];
    let mut s = vec![];

    const LEN: usize = 1024 * 1024;

    println!("Generating...");

    for _ in 0..LEN {
        let mut buff = [0u8; size_of::<GroupElement>()];
        thread_rng().fill_bytes(&mut buff);
        
        g.push(GroupElement::from_slice(&buff).unwrap());

        let mut buff = [0u8; size_of::<Scalar>()];
        thread_rng().fill_bytes(&mut buff);
        
        s.push(Scalar::from_slice(&buff).unwrap());
    }

    println!("Bench...");

    let now = Instant::now();

    for i in 0..LEN {
        scalarmult(&s[i], &g[i]).unwrap();
    }

    println!("{} mul/s", LEN as f64 / now.elapsed().as_secs_f64());
}
