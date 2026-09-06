use crate::parser::DataSovereignty;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum PolicyVerdict {
    AbsoluteImperative,
    AbsoluteProhibition,
    ConditionalExecution,
    QuarantineCatch,
}

#[derive(Clone, Debug)]
pub struct PolicyRule {
    pub target_scope: String,
    pub classification_restriction: DataSovereignty,
    pub verdict: PolicyVerdict,
}

pub struct PolicyLedger {
    pub rules_registry: HashMap<String, PolicyRule>,
}

impl PolicyLedger {
    pub fn new() -> Self {
        let mut ledger = Self {
            rules_registry: HashMap::new(),
        };

        ledger.inject_default_production_matrix();
        ledger
    }

    pub fn register_rule(&mut self, rule: PolicyRule) {
        let lookup_key = format!("{}_{:?}", rule.target_scope, rule.classification_restriction);
        self.rules_registry.insert(lookup_key, rule);
    }

    pub fn retrieve_rule(&self, scope: &str, classification: &DataSovereignty) -> PolicyRule {
        let lookup_key = format!("{}_{:?}", scope, classification);

        match self.rules_registry.get(&lookup_key) {
            Some(matched_rule) => matched_rule.clone(),

            None => {
                PolicyRule {
                    target_scope: scope.to_string(),
                    classification_restriction: classification.clone(),
                    verdict: PolicyVerdict::QuarantineCatch,
                }
            }
        }
    }

    fn inject_default_production_matrix(&mut self) {
        self.register_rule(PolicyRule {
            target_scope: "financial_ledger_vault".to_string(),
            classification_restriction: DataSovereignty::SovereignPublic,
            verdict: PolicyVerdict::AbsoluteImperative,
        });

        self.register_rule(PolicyRule {
            target_scope: "secure_vault".to_string(),
            classification_restriction: DataSovereignty::SovereignSecret,
            verdict: PolicyVerdict::ConditionalExecution,
        });

        self.register_rule(PolicyRule {
            target_scope: "public_diagnostic_dump".to_string(),
            classification_restriction: DataSovereignty::SovereignSecret,
            verdict: PolicyVerdict::AbsoluteProhibition,
        });
    }
}
