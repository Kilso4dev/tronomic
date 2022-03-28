use std::collections::HashMap;
use super::{Universe, Fixture};


#[derive(Debug, Clone)]
pub struct DmxState {
    pub universes: HashMap<usize, Universe>,
    pub fixt_next_id: usize,
    pub fixts: HashMap<usize, Fixture>,
    pub fixt_groups: HashMap<usize, Vec<usize>>,
}

impl DmxState {
    pub fn new(universe_count: usize) -> Self {
        Self {
            universes: {
                let mut n = HashMap::new();
                for i in 0..universe_count {
                    n.insert(i, Universe::new());
                }
                n
            },
            fixts: HashMap::new(),
            fixt_next_id: 0,
            fixt_groups: HashMap::new(),
        }
    }

    pub fn add_fixture(&mut self, new: Fixture) -> usize {
        let new_id = self.fixt_next_id;
        self.fixt_next_id += 1;
        self.fixts.insert(new_id, new);
        new_id
    }
    pub fn remove_fixture(&mut self, id: usize) -> Option<Fixture> {
        self.fixts.remove(&id)
    }
}
