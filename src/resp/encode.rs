use crate::resp::RespEncode;

impl RespEncode for i64 {
    fn encode(&self) -> Vec<u8> {
        let sign = if *self < 0 { b'-' } else { b'+' };
        format!("{}{}\r\n", sign, self.abs()).into_bytes()
    }
}