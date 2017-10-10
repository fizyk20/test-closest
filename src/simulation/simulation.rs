use tiny_keccak;
use std::collections::{HashSet, BTreeMap};
use rand::{thread_rng, Rng, ThreadRng};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(pub [u8; 32]);

fn sha3_256(data: &[u8]) -> U256 {
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
    pub stall_rate: f64,
    pub closest_success_rate: f64,
    pub avg_tries: f64,
    pub tries_map: BTreeMap<usize, usize>,
}

#[derive(Clone, Copy, Debug)]
struct RunResult {
    pub is_closest_to_prefix: bool,
    pub hash_num_tries: Option<usize>,
    pub num_stalled: usize,
}

pub trait SectionData {
    fn section(&self) -> HashSet<U256>;
    fn is_malicious(&self, name: &U256) -> bool;
    fn group_size(&self) -> usize;
    fn has_malicious_quorum(&self, nodes: &HashSet<U256>) -> bool;
    fn can_stall(&self, nodes: &HashSet<U256>) -> bool;

    fn can_stall_data(&self, point: U256) -> bool {
        let group = self.close_group(point);
        self.can_stall(&group)
    }

    fn malicious_nodes(&self, group: &HashSet<U256>) -> HashSet<U256> {
        group
            .into_iter()
            .filter(|&x| self.is_malicious(x))
            .cloned()
            .collect()
    }

    fn count_malicious(&self, group: &HashSet<U256>) -> usize {
        group
            .into_iter()
            .filter(|&name| self.is_malicious(name))
            .count()
    }

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

    pub fn run<F, S>(&self, times: usize, tries_per_time: usize, generate_section: F) -> SimResult
    where
        F: Fn(&mut ThreadRng, usize, usize, usize) -> S + Sync,
        S: SectionData,
    {
        let results: Vec<_> = (0..times)
            .into_par_iter()
            .map(|_| {
                let mut rng = thread_rng();
                let section_data = generate_section(
                    &mut rng,
                    self.group_size,
                    self.network_size,
                    self.quorum_size,
                );

                let is_closest_to_prefix = section_data.has_malicious_prefix_close_group();

                let mut hash_num_tries = None;
                let mut num_stalled = 0;
                for try in 0..tries_per_time {
                    let mut data = Vec::<u8>::with_capacity(1000);
                    for _ in 0..1000 {
                        data.push(rng.gen());
                    }

                    if section_data.has_malicious_close_group(sha3_256(&data)) &&
                        hash_num_tries.is_none()
                    {
                        hash_num_tries = Some(try + 1);
                    }

                    if section_data.can_stall_data(sha3_256(&data)) {
                        num_stalled += 1;
                    }
                }

                RunResult {
                    is_closest_to_prefix,
                    hash_num_tries,
                    num_stalled,
                }
            })
            .collect();
        let mut successful = BTreeMap::new();
        let mut successful_prefix = 0;
        let mut total_stalled = 0;
        for res in &results {
            if let Some(tries) = res.hash_num_tries {
                let entry = successful.entry(tries).or_insert_with(|| 0);
                *entry = *entry + 1;
            }
            total_stalled += res.num_stalled;
            if res.is_closest_to_prefix {
                successful_prefix += 1;
            }
        }

        let mut sum = 0;
        let mut avg = 0;
        for (&tries, &num) in &successful {
            sum += num;
            avg += num * tries;
        }

        SimResult {
            success_rate: sum as f64 / times as f64 * 100.0,
            stall_rate: total_stalled as f64 / (times as f64 * tries_per_time as f64) * 100.0,
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
