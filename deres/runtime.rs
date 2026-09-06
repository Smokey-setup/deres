use crate::parser::DataCapsule;
use crate::crypto::compute_sequential_block_hash;
use crate::policy::{PolicyLedger, PolicyVerdict};

#[derive(Debug, PartialEq)]
pub enum ConsequenceOutcome {
    ExecutedSafely(String),
    BlockedProhibition(String),
    QuarantineTriggered(String),
}

pub struct ConsequenceEngine {
    pub ledger_hash: String,
    pub operations_counter: u64,
}

impl ConsequenceEngine {
    pub fn new() -> Self {
        Self {
            ledger_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            operations_counter: 0,
        }
    }

    pub fn evaluate_action(
        &mut self,
        capsule: &DataCapsule,
        target_scope: &str,
        policy_ledger: &mut PolicyLedger
    ) -> ConsequenceOutcome {
        self.operations_counter += 1;

        let active_rule = policy_ledger.retrieve_rule(target_scope, &capsule.classification);

        match active_rule.verdict {
            PolicyVerdict::AbsoluteProhibition => {
                let diagnostic = format!(
                    "CRITICAL: Absolute Prohibition Intercepted. Data classification {:?} denied access to scope '{}'.",
                    capsule.classification, target_scope
                );
                ConsequenceOutcome::BlockedProhibition(diagnostic)
            }

            PolicyVerdict::QuarantineCatch => {
                let diagnostic = format!(
                    "ALERT: Scope intent mismatch. Capsule scope '{}' does not match target scope '{}'. Thread isolated.",
                    capsule.intent_scope, target_scope
                );
                ConsequenceOutcome::QuarantineTriggered(diagnostic)
            }

            PolicyVerdict::AbsoluteImperative | PolicyVerdict::ConditionalExecution => {
                if capsule.intent_scope != target_scope {
                    let diagnostic = format!(
                        "ALERT: Dynamic divergence detected. Isolating memory execution thread for target: {}",
                        target_scope
                    );
                    return ConsequenceOutcome::QuarantineTriggered(diagnostic);
                }

                let converted_data = match String::from_utf8(capsule.raw_bytes.clone()) {
                    Ok(valid_string) => valid_string,
                    Err(_) => return ConsequenceOutcome::QuarantineTriggered("ERR: Corrupted byte flow.".to_string()),
                };

                let audit_log = format!(
                    "TX_{}: Owner '{}' authorized processing in scope '{}'.",
                    self.operations_counter, capsule.owner_id, target_scope
                );

                self.ledger_hash = compute_sequential_block_hash(&self.ledger_hash, &audit_log);

                ConsequenceOutcome::ExecutedSafely(converted_data)
            }
        }
    }
}
