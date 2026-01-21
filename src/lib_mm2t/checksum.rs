/// TODO
pub(crate) fn checksum(packet_type: u8, payload: &[u8] ) -> u8 {
    let mut checksum = packet_type ^ payload.len() as u8;
    for &b in payload {
        checksum ^= b;
    }

    checksum
}