use super::checksum::checksum;

pub const SOP: u8 = 0xAA; // start of packet
const MAX_PAYLOAD_LEN: usize = 64;

enum DecodeState {
    WaitSOP,
    PacketType,
    Length,
    Payload,
    Checksum
}

pub struct DecodedPacket {
    packet_type: u8,
    payload: Vec<u8>,
}

pub struct PacketDecoder {
    state: DecodeState,
    length: usize,
    packet: Option<DecodedPacket>,
}

impl PacketDecoder {

    /// initiate a new packet decoder with default values
    pub fn new() -> Self {
        Self {
            state: DecodeState::WaitSOP,
            length: 0,
            packet: None,
        }
    }

    pub fn push(&mut self, byte: u8) -> anyhow::Result<Option<DecodedPacket>> {
        // as bytes come iterate decoding state to build packet
        match self.state {
            // at start of packet
            DecodeState::WaitSOP => {
                if byte == SOP {
                    self.state = DecodeState::PacketType;
                }
            }

            // determine packet type
            DecodeState::PacketType => {
                self.packet = Some(DecodedPacket {
                    packet_type: byte,
                    payload: Vec::new(),
                });
                self.state = DecodeState::Length;
            }

            // update length of payload
            DecodeState::Length => {
                let len_read = byte as usize;
                // stop reading if length is too large
                if len_read > MAX_PAYLOAD_LEN {
                    self.reset();
                    return Ok(None);
                }

                // length read was appropriate
                self.length = len_read;
                self.state = DecodeState::Payload;
            }

            // read entire payload
            DecodeState::Payload => {
                // assumming payload was created push bytes
                if let Some(p) = &mut self.packet {
                    p.payload.push(byte);

                    // verify if payload length has met expected
                    if p.payload.len() == self.length {
                        self.state = DecodeState::Checksum;
                    }
                }
            }

            // confirm checksum is correct and return the packet if so
            DecodeState::Checksum => {
                let p = self.packet.take().unwrap();
                let cs_expected = checksum(p.packet_type, &p.payload);
                if cs_expected != byte {
                    self.reset();
                    anyhow::bail!("Checksum packet mismatch");
                }

                self.reset();
                return Ok(Some(p));
            }
        }
        
        Ok(None) // no packet to return yet
    }

    fn reset(&mut self) {
        self.state = DecodeState::WaitSOP;
        self.length = 0;
        self.packet = None;
    }

}