#[derive(Debug, Clone)]
pub struct FixturePurpose {
    pub fixture_id: usize,
    pub fixture_purpose_id: usize,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub val: u8,
    pub val_ovr: u8,
    pub ovr: bool,
    pub purpose: Option<FixturePurpose>,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            val: 0,
            val_ovr: 0,
            ovr: false,
            purpose: None,
        }
    }
}

impl Channel {
    pub fn get(&self) -> u8 {
        if self.ovr {
            self.val_ovr
        } else {
            self.val
        }
    }

    pub fn set(&mut self, new: u8) {
        if self.ovr {
            self.val_ovr = new;
        } else {
            self.val = new
        }
    }

    pub fn override_with(&mut self, en: bool, new: u8) {
        self.ovr = en;
        self.set(new);
    }
}
