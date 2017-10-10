use super::*;
use rand::Rng;
use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug)]
pub struct NodeAgeingSectionData {
    group_size: usize,
    section: HashMap<U256, u8>,
    malicious: HashSet<U256>,
}

impl NodeAgeingSectionData {
    pub fn new<R: Rng>(
        rng: &mut R,
        group_size: usize,
        size: usize,
        n_malicious: usize,
    ) -> NodeAgeingSectionData {
        let section = gen_names(rng, size);
        let malicious = gen_malicious(rng, &section, n_malicious);
        let section = gen_ages(rng, &section);

        NodeAgeingSectionData {
            group_size,
            section,
            malicious,
        }
    }
}

impl SectionData for NodeAgeingSectionData {
    fn group_size(&self) -> usize {
        self.group_size
    }

    fn section(&self) -> HashSet<U256> {
        self.section.keys().cloned().collect()
    }

    fn has_malicious_quorum(&self, group: &HashSet<U256>) -> bool {
        let sum_ages: u16 = group
            .iter()
            .filter_map(|name| self.section.get(name))
            .map(|&x| x as u16)
            .sum();
        let sum_ages_malicious: u16 = group
            .iter()
            .filter(|&name| self.malicious.contains(name))
            .filter_map(|name| self.section.get(name))
            .map(|&x| x as u16)
            .sum();
        self.malicious.is_subset(group) && sum_ages_malicious * 2 > sum_ages
    }
}
