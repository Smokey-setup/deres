use sha2::{Sha256, Digest};

pub fn generate_data_seal(payload: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    format!("{:x}", hasher.finalize())
}

pub fn compute_sequential_block_hash(previous_hash: &str, current_log_entry: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(previous_hash.as_bytes());
    hasher.update(current_log_entry.as_bytes());
    format!("{:x}", hasher.finalize())
}
