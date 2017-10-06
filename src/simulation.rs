use tiny_keccak;
use std::collections::{HashSet, BTreeMap};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(pub [u8; 32]);

pub fn sha3_256(data: &[u8]) -> U256 {
    U256(tiny_keccak::sha3_256(data))
}

pub fn distance(x: &U256, y: &U256) -> U256 {
    let mut result = U256([0; 32]);
    for i in 0..32 {
        result.0[i] = x.0[i] ^ y.0[i];
    }
    result
}

#[derive(Clone, Debug)]
pub struct SimResult {
    pub success_rate: f64,
    pub closest_success_rate: f64,
    pub avg_tries: f64,
    pub tries_map: BTreeMap<usize, usize>,
}

#[derive(Clone, Copy, Debug)]
struct RunResult {
    pub is_closest_to_prefix: bool,
    pub hash_num_tries: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct Simulation {
    group_size: usize,
    network_size: usize,
    quorum_size: usize,
}

impl Simulation {
    pub fn new(group_size: usize, network_size: usize, quorum_size: usize) -> Simulation {
        Simulation {
            group_size,
            network_size,
            quorum_size,
        }
    }

    pub fn close_group(&self, section: &[U256], point: U256) -> HashSet<U256> {
        let mut sorted: Vec<_> = section.into();
        sorted.sort_by_key(|x| distance(x, &point));
        sorted.into_iter().take(self.group_size).collect()
    }

    pub fn generate_section<R: Rng>(&self, rng: &mut R) -> (Vec<U256>, HashSet<U256>) {
        let mut section = vec![];
        for _ in 0..self.network_size {
            section.push(U256(rng.gen()));
        }

        let mut malicious = HashSet::new();
        while malicious.len() < self.quorum_size {
            let index = rng.gen_range(0, section.len());
            malicious.insert(section[index]);
        }
        (section, malicious)
    }

    pub fn run(&self, times: usize, tries_per_time: usize) -> SimResult {
        let results: Vec<_> = (0..times)
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                let (section, malicious) = self.generate_section(&mut rng);

                let is_closest_to_prefix =
                    malicious.is_subset(&self.close_group(&section, U256([0; 32])));

                let mut hash_num_tries = None;
                for try in 0..tries_per_time {
                    let mut data = Vec::<u8>::with_capacity(1000);
                    for _ in 0..1000 {
                        data.push(rng.gen());
                    }

                    let group = self.close_group(&section, sha3_256(&data));

                    if malicious.is_subset(&group) {
                        hash_num_tries = Some(try + 1);
                        break;
                    }
                }

                RunResult {
                    is_closest_to_prefix,
                    hash_num_tries,
                }
            })
            .collect();
        let mut successful = BTreeMap::new();
        for res in results.iter().filter_map(|&x| x.hash_num_tries) {
            let entry = successful.entry(res).or_insert_with(|| 0);
            *entry = *entry + 1;
        }
        let successful_prefix = results.iter().filter(|&x| x.is_closest_to_prefix).count();
        let mut sum = 0;
        let mut avg = 0;
        for (&tries, &num) in &successful {
            sum += num;
            avg += num * tries;
        }

        SimResult {
            success_rate: sum as f64 / times as f64 * 100.0,
            closest_success_rate: successful_prefix as f64 / times as f64 * 100.0,
            avg_tries: if sum > 0 {
                avg as f64 / sum as f64
            } else {
                0.0
            },
            tries_map: successful,
        }
    }
}
