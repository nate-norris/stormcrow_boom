use crate::mm2t_packet::Packet;

pub struct BoomPacket;

impl Packet for BoomPacket {
    fn packet_type(&self) -> u8 {
        0x42 // character 'B' .. as in Boom
    }

    fn payload(&self) -> &[u8] {
        static PAYLOAD: [u8; 1] = [0x01];
        &PAYLOAD
    }
}