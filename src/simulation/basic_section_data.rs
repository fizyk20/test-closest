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
        let section = gen_names(rng, size);
        let malicious = gen_malicious(rng, &section, n_malicious);

        BasicSectionData {
            group_size,
            section,
            malicious,
        }
    }
}

impl SectionData for BasicSectionData {
    fn group_size(&self) -> usize {
        self.group_size
    }

    fn section(&self) -> HashSet<U256> {
        self.section.clone()
    }

    fn is_malicious(&self, name: &U256) -> bool {
        self.malicious.contains(name)
    }

    fn has_malicious_quorum(&self, group: &HashSet<U256>) -> bool {
        self.count_malicious(group) > group.len() / 2
    }

    fn can_stall(&self, group: &HashSet<U256>) -> bool {
        self.count_malicious(group) > (group.len() - 1) / 2
    }
}
