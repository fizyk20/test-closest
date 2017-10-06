use super::*;
use rand::Rng;
use rand::distributions::{Exp, IndependentSample};
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
        let mut section = vec![];
        for _ in 0..size {
            section.push(U256(rng.gen()));
        }

        let mut malicious = HashSet::new();
        while malicious.len() < n_malicious {
            let index = rng.gen_range(0, section.len());
            malicious.insert(section[index]);
        }

        let mut section_map = HashMap::new();
        let exp = Exp::new(1.0);
        for n in section {
            section_map.insert(n, exp.ind_sample(rng) as u8);
        }

        NodeAgeingSectionData {
            group_size,
            section: section_map,
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
        let sum_ages: u16 = self.section.values().map(|x| *x as u16).sum();
        let sum_ages_malicious: u16 = self.section
            .iter()
            .filter(|&(name, _)| self.malicious.contains(name))
            .map(|(_, val)| *val as u16)
            .sum();
        self.malicious.is_subset(group) && sum_ages_malicious * 2 > sum_ages
    }
}
