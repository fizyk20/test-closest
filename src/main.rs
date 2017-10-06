extern crate rand;
extern crate tiny_keccak;

use rand::{thread_rng, Rng};
use std::collections::HashSet;

const G: usize = 10;
const S: usize = 25;
const Q: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct U256(pub [u8; 32]);

fn sha3_256(data: &[u8]) -> U256 {
    U256(tiny_keccak::sha3_256(data))
}

fn distance(x: &U256, y: &U256) -> U256 {
    let mut result = U256([0; 32]);
    for i in 0..32 {
        result.0[i] = x.0[i] ^ y.0[i];
    }
    result
}

fn close_group(section: &[U256], point: U256) -> HashSet<U256> {
    let mut sorted: Vec<_> = section.into();
    sorted.sort_by_key(|x| distance(x, &point));
    sorted.into_iter().take(G).collect()
}

fn generate_section<R: Rng>(rng: &mut R) -> (Vec<U256>, HashSet<U256>) {
    let mut section = vec![];
    let mut name = [0; 32];
    name[0] = 5;
    // only works for S = 25
    for _ in 0..S {
        section.push(U256(name));
        name[0] += 10;
    }

    let mut malicious = HashSet::new();
    while malicious.len() < Q {
        let index = rng.gen_range(0, section.len());
        malicious.insert(section[index]);
    }
    (section, malicious)
}

fn main() {
    let mut rng = thread_rng();
    let mut successful: Vec<(usize, usize)> = vec![];
    let n = 5000;

    for test in 0..n {
        let (section, malicious) = generate_section(&mut rng);

        for try in 0..500 {
            let mut data = vec![];
            for _ in 0..1000 {
                data.push(rng.gen());
            }

            let group = close_group(&section, sha3_256(&data));

            if malicious.is_subset(&group) {
                if successful.is_empty() || successful[successful.len() - 1].0 != test {
                    successful.push((test, try + 1));
                    break;
                }
            }
        }
    }
    println!(
        "Success rate: {}%",
        successful.len() as f64 / n as f64 * 100.0
    );
    let sum: usize = successful.iter().map(|x| x.1).sum();
    if !successful.is_empty() {
        println!(
            "Avg number of tries: {}",
            sum as f64 / successful.len() as f64
        );
    }
    if let Some(max) = successful.iter().map(|x| x.1).max() {
        for i in 1..max + 1 {
            let count = successful.iter().map(|x| x.1).filter(|&x| x == i).count();
            if count != 0 {
                println!("{} tries: {} cases", i, count);
            }
        }
    }
}
