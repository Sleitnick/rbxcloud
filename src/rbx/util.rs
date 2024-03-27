use base64::{engine::general_purpose::STANDARD, Engine as _};
use md5::{Digest, Md5};

pub type QueryString = Vec<(&'static str, String)>;

#[inline]
pub fn get_checksum_base64(data: &String) -> String {
    let mut md5_hash = Md5::new();
    md5_hash.update(data.as_bytes());
    STANDARD.encode(md5_hash.finalize())
}
