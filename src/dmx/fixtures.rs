use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum ChannelPurpose {
    NoType,

    ColR,
    ColG,
    ColB,
    ColW,

    ColC,
    ColM,
    ColY,
    ColK,

    Dimmer,

    Pan,
    PanFine,
    Tilt,
    TiltFine,
}

#[derive(Debug, Clone)]
pub struct Fixture {
    pub universe_id: usize,
    pub start: usize,
    pub channel_purposes: Vec<ChannelPurpose>,
}
