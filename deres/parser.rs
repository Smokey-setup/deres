use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataSovereignty {
    SovereignSecret,
    SovereignPublic,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataCapsule {
    pub owner_id: String,
    pub intent_scope: String,
    pub classification: DataSovereignty,
    pub raw_bytes: Vec<u8>,
}

pub fn parse_input(raw_input: &str, owner: &str, scope: &str) -> DataCapsule {
    let input_lower = raw_input.to_lowercase();
    
    let classification = if input_lower.contains("secret") || input_lower.contains("private_key") {
        DataSovereignty::SovereignSecret
    } else {
        DataSovereignty::SovereignPublic
    };

    DataCapsule {
        owner_id: owner.trim().to_string(),
        intent_scope: scope.trim().to_string(),
        classification,
        raw_bytes: raw_input.as_bytes().to_vec(),
    }
}
