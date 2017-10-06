use super::*;
use super::basic_section_data::BasicSectionData;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct SimStruct {
    group_size: usize,
    network_size: usize,
    quorum_size: usize,
}

impl SimStruct {
    pub fn new(group_size: usize, network_size: usize, quorum_size: usize) -> SimStruct {
        SimStruct {
            group_size,
            network_size,
            quorum_size,
        }
    }
}

impl Simulation<BasicSectionData> for SimStruct {
    fn generate_section<R: Rng>(&self, rng: &mut R) -> BasicSectionData {
        BasicSectionData::new(rng, self.group_size, self.network_size, self.quorum_size)
    }
}
