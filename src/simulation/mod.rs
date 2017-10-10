mod simulation;
mod basic_section_data;
mod node_ageing_section_data;
mod node_ageing_index_weights;

pub use self::simulation::*;
pub use self::basic_section_data::BasicSectionData;
pub use self::node_ageing_section_data::NodeAgeingSectionData;
pub use self::node_ageing_index_weights::NodeAgeingIndexWeights;

use rand::Rng;
use std::collections::{HashSet, HashMap};

pub fn gen_names<R: Rng>(rng: &mut R, size: usize) -> HashSet<U256> {
    let mut section = HashSet::new();
    while section.len() < size {
        section.insert(U256(rng.gen()));
    }
    section
}

pub fn gen_malicious<R: Rng>(
    rng: &mut R,
    section: &HashSet<U256>,
    n_malicious: usize,
) -> HashSet<U256> {
    let vec_section: Vec<_> = section.into_iter().cloned().collect();

    let mut malicious = HashSet::new();
    while malicious.len() < n_malicious {
        let index = rng.gen_range(0, vec_section.len());
        malicious.insert(vec_section[index]);
    }

    malicious
}

pub fn gen_ages<R: Rng>(rng: &mut R, section: &HashSet<U256>) -> HashMap<U256, u8> {
    let size = section.len();
    let churn_events = rng.gen_range(size * 3 / 2, size * 5 / 2);
    let mut section_map = HashMap::new();
    for n in section {
        let churn_step = rng.gen_range(0, churn_events);
        section_map.insert(*n, (churn_step as f64).log2() as u8);
    }
    section_map
}
