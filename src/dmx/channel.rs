#[derive(Debug, Clone)]
pub struct FixturePurpose {
    pub fixture_id: usize,
    pub fixture_purpose_id: usize,
}

#[derive(Debug)]
pub struct Channel<'a> {
    pub val: &'a u8,
    pub val_ovr: &'a u8,
    pub ovr: &'a bool,
}

impl<'a> Channel<'a> {

    pub fn get(&'a self) -> &'a u8 {
        if *self.ovr {
            self.val_ovr
        } else {
            self.val
        }
    }
}

#[derive(Debug)]
pub struct ChannelMut<'a> {
    pub val: &'a mut u8,
    pub val_ovr: &'a mut u8,
    pub ovr: &'a mut bool,
}

impl<'a> ChannelMut<'a> {

    pub fn get(&'a self) -> &'a u8 {
        if *self.ovr {
            self.val_ovr
        } else {
            self.val
        }
    }

    pub fn set(&'a mut self, new: u8) {
        if *self.ovr {
            *self.val_ovr = new;
        } else {
            *self.val = new
        }
    }

    pub fn override_with(&'a mut self, en: bool, new: u8) {
        *self.ovr = en;
        self.set(new);
    }
}
