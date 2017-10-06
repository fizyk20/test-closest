use tiny_keccak;
use std::collections::{HashSet, BTreeMap};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(pub [u8; 32]);

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

pub trait SectionData {
    fn section(&self) -> HashSet<U256>;
    fn group_size(&self) -> usize;
    fn has_malicious_quorum(&self, nodes: &HashSet<U256>) -> bool;

    fn has_malicious_prefix_close_group(&self) -> bool {
        let group = self.close_group(U256([0; 32]));
        self.has_malicious_quorum(&group)
    }

    fn has_malicious_close_group(&self, point: U256) -> bool {
        let group = self.close_group(point);
        self.has_malicious_quorum(&group)
    }

    fn close_group(&self, point: U256) -> HashSet<U256> {
        let mut sorted: Vec<_> = self.section().into_iter().collect();
        sorted.sort_by_key(|x| distance(x, &point));
        sorted.into_iter().take(self.group_size()).collect()
    }
}

pub trait Simulation<T: SectionData>: Sync {
    fn generate_section<R: Rng>(&self, rng: &mut R) -> T;

    fn run(&self, times: usize, tries_per_time: usize) -> SimResult {
        let results: Vec<_> = (0..times)
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                let section_data = self.generate_section(&mut rng);

                let is_closest_to_prefix = section_data.has_malicious_prefix_close_group();

                let mut hash_num_tries = None;
                for try in 0..tries_per_time {
                    let mut data = Vec::<u8>::with_capacity(1000);
                    for _ in 0..1000 {
                        data.push(rng.gen());
                    }

                    if section_data.has_malicious_close_group(sha3_256(&data)) {
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
