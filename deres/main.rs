mod parser;
mod crypto;
mod runtime;
mod policy;
mod export;

use parser::parse_input;
use runtime::{ConsequenceEngine, ConsequenceOutcome};
use policy::PolicyLedger;

fn main() {
    println!("============================================================");
    println!("   DERES RUNTIME ENGINE v1.0.0 : ACTIVE AUDIT MATRIX       ");
    println!("============================================================\n");

    let mut core_engine = ConsequenceEngine::new();
    let mut policy_manager = PolicyLedger::new();

    // SCENARIO 1: Compliant Loop
    println!("[Step 1] Ingesting standard transactional request data...");
    let data_capsule_1 = parse_input("payload: data_processing_request_block_01", "entity_user_88", "financial_ledger_vault");

    if let ConsequenceOutcome::ExecutedSafely(clean_payload) = core_engine.evaluate_action(&data_capsule_1, "financial_ledger_vault", &mut policy_manager) {
        println!("[✔] CONDITION MET: Action processed cleanly.");
        let safe_response = export::secure_export(&clean_payload, "Python_FFI_Target");
        println!("    Export Status: {}", safe_response);
        println!("    Active Ledger Log Hash: {}\n", &core_engine.ledger_hash[..16]);
    }

    // SCENARIO 2: Hacker Cross-Scope Bleeding Attempt
    println!("[Step 2] Ingesting deceptive transactional request data (Exploit Vector)...");
    let data_capsule_2 = parse_input("payload: user_private_billing_credentials", "entity_user_88", "financial_ledger_vault");

    if let ConsequenceOutcome::QuarantineTriggered(alert_message) = core_engine.evaluate_action(&data_capsule_2, "untrusted_marketing_tracker", &mut policy_manager) {
        println!("[🛡] DEFENSE REGISTERED: {}", alert_message);
        println!("    Action: System instantly neutralized the memory pipeline. No data leaked.\n");
    }

    // SCENARIO 3: Absolute Prohibition Attack
    println!("[Step 3] Ingesting highly classified system data...");
    let data_capsule_3 = parse_input("payload: corporate_master_private_secret_key", "root_administrator", "secure_vault");

    if let ConsequenceOutcome::BlockedProhibition(panic_message) = core_engine.evaluate_action(&data_capsule_3, "public_diagnostic_dump", &mut policy_manager) {
        println!("[🚨] CRITICAL ABSOLUTE PROHIBITION TRIGGERED: {}", panic_message);
        println!("    Action: Wiped active processing registers. Threat completely air-gapped.");
    }
}
