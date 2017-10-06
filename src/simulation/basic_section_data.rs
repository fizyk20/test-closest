use super::*;
use rand::Rng;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct BasicSectionData {
    group_size: usize,
    section: HashSet<U256>,
    malicious: HashSet<U256>,
}

impl BasicSectionData {
    pub fn new<R: Rng>(
        rng: &mut R,
        group_size: usize,
        size: usize,
        n_malicious: usize,
    ) -> BasicSectionData {
        let mut section = vec![];
        for _ in 0..size {
            section.push(U256(rng.gen()));
        }

        let mut malicious = HashSet::new();
        while malicious.len() < n_malicious {
            let index = rng.gen_range(0, section.len());
            malicious.insert(section[index]);
        }

        BasicSectionData {
            group_size,
            section: section.into_iter().collect(),
            malicious,
        }
    }
}

impl SectionData for BasicSectionData {
    fn group_size(&self) -> usize {
        self.group_size
    }

    fn section(&self) -> &HashSet<U256> {
        &self.section
    }

    fn has_malicious_quorum(&self, group: &HashSet<U256>) -> bool {
        self.malicious.is_subset(group)
    }
}
