use super::Channel;

#[derive(Debug, Clone)]
pub struct Universe {
    pub channels: Vec<Channel>
}

impl Universe {
    pub fn new() -> Self {
        Self {
            channels: vec![Channel::default(); 512],
        }
    }
}
