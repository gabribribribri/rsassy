// Interesting things :
// The message can't be bigger than the second number of the key

use std::collections::HashSet;

pub type PublicKey = (u32, u128);
pub type SecreteKey = (u32, u128);

pub fn encrypt(message: u128, pkey: PublicKey) -> u128 {
    message.pow(pkey.0) % pkey.1
}

pub fn decrypt(message: u128, skey: SecreteKey) -> u128 {
    message.pow(skey.0) % skey.1
}

pub fn generate_keys() -> (PublicKey, SecreteKey) {
    // Choose two prime numbers
    let p: u128 = 2;
    let q: u128 = 7;

    // Product of p and q
    let n = p * q;

    // numbers of coprimes with n
    let cop_n = coprimes_count(p, q);

    // e must be coprime with n and cop_n
    let e = coprimes_2num(n, cop_n);
    todo!()
}

fn coprimes_count(p: u128, q: u128) -> u128 {
    (p - 1) * (q - 1)
}

// return the number of numbers lesser than n that doesn't share a common factor with n
pub fn coprimes_2num(n: u128, m: u128) -> HashSet<u128> {
    let mut nmfactors = factors(n, m);
    nmfactors.remove(&1);
    (1..=std::cmp::min(n, m))
        .filter(|a| nmfactors.iter().all(|b| a % b != 0))
        .collect()
}

// factors of n
pub fn factors(n: u128, m: u128) -> HashSet<u128> {
    (1..=(n as f32).sqrt() as u128 + 1)
        .filter(|x| n % x == 0 && m % x == 0)
        .flat_map(|x| [x, m / x])
        .collect::<HashSet<u128>>()
}
