use utils::mm2t::PacketT;

pub struct BoomPacket {
    count: u8,
}

impl BoomPacket {
    pub fn new(count: u8) -> Self {
        Self { count }
    }
}

impl PacketT for BoomPacket {
    fn packet_type(&self) -> u8 {
        0x42 // character 'B' .. as in Boom
    }

    fn payload(&self) -> &[u8] {
        std::slice::from_ref(&self.count)
    }
}