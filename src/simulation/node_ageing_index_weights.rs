use super::*;
use rand::Rng;
use rand::distributions::{Exp, IndependentSample};
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

        NodeAgeingIndexWeights {
            group_size,
            section: section_map,
            malicious,
        }
    }
}

impl SectionData for NodeAgeingIndexWeights {
    fn group_size(&self) -> usize {
        self.group_size
    }

    fn section(&self) -> HashSet<U256> {
        self.section.keys().cloned().collect()
    }

    fn has_malicious_quorum(&self, group: &HashSet<U256>) -> bool {
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
        let three_quarters = self.group_size * 3 / 4;
        let weight_limit = three_quarters * (three_quarters - 1) / 2;
        let sum_indices_malicious: u16 = sorted_group
            .iter()
            .map(|x| *x.0)
            .enumerate()
            .filter(|&(_, name)| self.malicious.contains(&name))
            .map(|(index, _)| index as u16)
            .sum();
        self.malicious.is_subset(group) && sum_indices_malicious >= weight_limit as u16
    }
}
