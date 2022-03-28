use super::{Channel, ChannelMut};

#[derive(Debug, Clone)]
pub struct Universe {
    vals: [u8; 512],
    vals_ovr: [u8; 512],
    ovr: [bool; 512],
}

impl Universe {
    pub fn new() -> Self {
        Self {
            vals: [0; 512],
            vals_ovr: [0; 512],
            ovr: [false; 512],
        }
    }

    pub fn get_channel(&self, i: usize) -> Option<Channel> {
        Some(Channel {
            val: self.vals.get(i)?,
            val_ovr: self.vals_ovr.get(i)?,
            ovr: self.ovr.get(i)?,
        })
    }

    pub fn get_channel_mut(&mut self, i: usize) -> Option<ChannelMut> {
        Some(ChannelMut {
            val: self.vals.get_mut(i)?,
            val_ovr: self.vals_ovr.get_mut(i)?,
            ovr: self.ovr.get_mut(i)?,
        })
    }

    pub fn get(&self, i: usize) -> u8 {
        if self.ovr[i] {
            self.vals_ovr[i]
        } else {
            self.vals[i]
        }
    }

    pub fn set(&mut self, i: usize, n: u8) {
        self.vals[i] = n;
    }

    pub fn set_ovr(&mut self, i: usize, n: u8) {
        self.ovr[i] = true;
        self.vals_ovr[i] = n;
    }

    pub fn reset_ovr(&mut self, i: usize) {
        self.ovr[i] = false;
    }
}
