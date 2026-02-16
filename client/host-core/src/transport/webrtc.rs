#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportDescriptor {
    pub force_lan_mode: bool,
    pub enable_unordered_move_channel: bool,
    pub max_datagram_size: u16,
}

impl TransportDescriptor {
    pub fn lan_low_latency() -> Self {
        Self {
            force_lan_mode: true,
            enable_unordered_move_channel: true,
            max_datagram_size: 1200,
        }
    }
}
