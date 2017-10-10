use super::*;
use rand::Rng;
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct NodeAgeingIndexWeights {
    group_size: usize,
    section: HashMap<U256, u8>,
    malicious: HashSet<U256>,
}

impl NodeAgeingIndexWeights {
    pub fn new<R: Rng>(
        rng: &mut R,
        group_size: usize,
        size: usize,
        n_malicious: usize,
    ) -> NodeAgeingIndexWeights {
        let section = gen_names(rng, size);
        let malicious = gen_malicious(rng, &section, n_malicious);
        let section = gen_ages(rng, &section);

        NodeAgeingIndexWeights {
            group_size,
            section,
            malicious,
        }
    }

    fn sorted_group(&self, group: &HashSet<U256>) -> Vec<U256> {
        let mut sorted_group: Vec<_> = group
            .into_iter()
            .filter_map(|name| {
                self.section.get(name).and_then(|age| Some((name, age)))
            })
            .collect();
        let zero = U256([0; 32]);
        sorted_group.sort_unstable_by(|a, b| {
            let age_order = a.1.cmp(b.1);
            if age_order == Ordering::Equal {
                distance(a.0, &zero).cmp(&distance(b.0, &zero))
            } else {
                age_order
            }
        });
        sorted_group.into_iter().map(|(name, _)| *name).collect()
    }

    fn sum_indices_malicious(&self, group: &HashSet<U256>) -> u16 {
        self.sorted_group(group)
            .into_iter()
            .enumerate()
            .filter(|&(_, name)| self.malicious.contains(&name))
            .map(|(index, _)| index as u16)
            .sum()
    }

    fn weight_limit(&self) -> u16 {
        let three_quarters = (self.group_size * 3) / 4;
        (three_quarters * (three_quarters - 1) / 2) as u16
    }
}

impl SectionData for NodeAgeingIndexWeights {
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
            self.sum_indices_malicious(group) > self.weight_limit()
    }

    fn can_stall(&self, group: &HashSet<U256>) -> bool {
        let total_indices = group.len() * (group.len() - 1) / 2;
        self.count_malicious(group) > (group.len() - 1) / 2 ||
            self.sum_indices_malicious(group) >= total_indices as u16 - self.weight_limit()
    }
}
