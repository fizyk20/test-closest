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

    fn total_age(&self, group: &HashSet<U256>) -> u16 {
        group
            .iter()
            .filter_map(|name| self.section.get(name))
            .map(|&x| x as u16)
            .sum()
    }

    fn malicious_age(&self, group: &HashSet<U256>) -> u16 {
        self.malicious_nodes(group)
            .into_iter()
            .filter_map(|name| self.section.get(&name))
            .map(|&x| x as u16)
            .sum()
    }
}

impl SectionData for NodeAgeingSectionData {
    fn group_size(&self) -> usize {
        self.group_size
    }

    fn section(&self) -> HashSet<U256> {
        self.section.keys().cloned().collect()
    }

    fn is_malicious(&self, name: &U256) -> bool {
        self.malicious.contains(name)
    }

    fn has_malicious_quorum(&self, group: &HashSet<U256>) -> bool {
        self.count_malicious(group) > group.len() / 2 &&
            self.malicious_age(group) * 2 > self.total_age(group)
    }

    fn can_stall(&self, group: &HashSet<U256>) -> bool {
        self.count_malicious(group) > (group.len() - 1) / 2 ||
            self.malicious_age(group) * 2 >= self.total_age(group)
    }
}
