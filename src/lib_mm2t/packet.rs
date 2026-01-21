pub trait PacketT {
    const SOP: u8 = 0xAA; // start of packet

    fn packet_type(&self) -> u8;

    fn payload(&self) -> &[u8];

    fn checksum(&self) -> u8 {
        let mut checksum = self.packet_type() ^ self.payload().len() as u8;
        for &b in self.payload() {
            checksum ^= b;
        }

        checksum
    }

    fn to_bytes(&self) -> Vec<u8> {
        // initiate bytes with enough room for payload, SOP, packet type, 
        //      length of payload, and checksum
        let mut buf = Vec::with_capacity(self.payload().len() + 4);
        // push bytes onto buffer
        buf.push(Self::SOP); //start of packet
        buf.push(self.packet_type()); //packet type
        buf.push(self.payload().len() as u8); // payload length
        buf.extend_from_slice(self.payload());
        buf.push(self.checksum());

        buf // return bytes
    }
}