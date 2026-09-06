use crate::crypto::generate_data_seal;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportCertificate {
    pub engine_identifier: String,
    pub payload_format_version: String,
    pub target_runtime_boundary: String,
    pub cryptographic_integrity_seal: String,
    pub sanitized_payload_bytes: Vec<u8>,
}

pub fn secure_export(processed_data: &str, target_boundary: &str) -> String {
    let sanitized_string = processed_data.replace('\0', "").replace("\r", "");
    let serialized_bytes = sanitized_string.as_bytes().to_vec();
    let signature_seal = generate_data_seal(&serialized_bytes);

    let certificate = ExportCertificate {
        engine_identifier: "DERES_CORE_ENGINE".to_string(),
        payload_format_version: "1.0.0".to_string(),
        target_runtime_boundary: target_boundary.trim().to_string(),
        cryptographic_integrity_seal: signature_seal,
        sanitized_payload_bytes: serialized_bytes,
    };

    match serde_json::to_string(&certificate) {
        Ok(json_payload) => json_payload,
        Err(_) => r#"{"engine_identifier":"DERES_CORE_ENGINE","error":"CRITICAL_SYSTEM_EXPORT_MUTATION_FAILURE"}"#.to_string()
    }
}
