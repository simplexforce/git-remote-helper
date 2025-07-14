// See https://git-scm.com/docs/gitprotocol-v2

pub const FLUSH_PKT: &[u8] = b"0000";
pub const DELIM_PKT: &[u8] = b"0001";
pub const RESPONSE_END_PKT: &[u8] = b"0002";

pub const PKTLINE_MAXIMUM_LENGTH: usize = 65520;

pub fn wrap_pktline(data: &[u8]) -> Vec<u8> {
    let len = data.len() + 4;
    if len > PKTLINE_MAXIMUM_LENGTH {
        panic!("pktline length {} exceeds maximum length {}", len, PKTLINE_MAXIMUM_LENGTH)
    }

    let hex_len = format!("{:04x}", len);
    [hex_len.as_bytes().to_vec(), data.to_vec()].concat()
}