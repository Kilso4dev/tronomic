pub mod artnet;
pub mod sacn;
pub mod ofl;

pub trait UniverseSender {
    fn send(&self, univ: &crate::dmx::Universe);
}

use slotmap::SlotMap;

slotmap::new_key_type! {
    pub struct ArtnetConnectionId;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TronCon {
    ArtnetOut(SlotMap<ArtnetConnectionId, artnet::ArtnetConnection>)
}
