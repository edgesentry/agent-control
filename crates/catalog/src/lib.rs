//! OWASP LLM + Agentic Top 10 risk register (`catalog/owasp-llm-asi.yaml`).

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Root document at `catalog/owasp-llm-asi.yaml`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskRegister {
    pub version: String,
    pub schema: String,
    #[serde(default)]
    pub updated: Option<String>,
    #[serde(default)]
    pub sources: Vec<SourceRef>,
    pub risks: Vec<RiskEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceRef {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskEntry {
    pub id: String,
    pub family: RiskFamily,
    pub name: String,
    pub summary: String,
    pub hooks: Vec<String>,
    pub acs_methods: Vec<String>,
    pub test_id_prefix: String,
    pub challenges: Vec<Challenge>,
    pub p0_smoke: bool,
    #[serde(default)]
    pub related: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskFamily {
    Llm,
    Agentic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Challenge {
    #[serde(rename = "CS01")]
    Cs01,
    #[serde(rename = "CS02")]
    Cs02,
    #[serde(rename = "both", alias = "BOTH")]
    Both,
}

impl RiskRegister {
    pub fn parse_yaml(yaml: &str) -> Result<Self, CatalogError> {
        let register: RiskRegister = serde_yaml::from_str(yaml)?;
        register.validate()?;
        Ok(register)
    }

    pub fn load(path: &Path) -> Result<Self, CatalogError> {
        let raw = std::fs::read_to_string(path)?;
        Self::parse_yaml(&raw)
    }

    /// Default path relative to repository root: `catalog/owasp-llm-asi.yaml`.
    pub fn load_default_repo_file() -> Result<Self, CatalogError> {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = manifest.join("../../catalog/owasp-llm-asi.yaml");
        Self::load(&path)
    }

    pub fn validate(&self) -> Result<(), CatalogError> {
        if self.schema != "agent-control/catalog/v1" {
            return Err(CatalogError::UnsupportedSchema(self.schema.clone()));
        }
        let llm = self
            .risks
            .iter()
            .filter(|r| r.family == RiskFamily::Llm)
            .count();
        let agentic = self
            .risks
            .iter()
            .filter(|r| r.family == RiskFamily::Agentic)
            .count();
        if llm != 10 {
            return Err(CatalogError::RiskCount {
                family: "llm",
                expected: 10,
                actual: llm,
            });
        }
        if agentic != 10 {
            return Err(CatalogError::RiskCount {
                family: "agentic",
                expected: 10,
                actual: agentic,
            });
        }
        for risk in &self.risks {
            if risk.hooks.is_empty() {
                return Err(CatalogError::MissingHooks(risk.id.clone()));
            }
            if risk.test_id_prefix.is_empty() {
                return Err(CatalogError::MissingTestPrefix(risk.id.clone()));
            }
            if risk.challenges.is_empty() {
                return Err(CatalogError::MissingChallenges(risk.id.clone()));
            }
        }
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&RiskEntry> {
        self.risks.iter().find(|r| r.id == id)
    }

    pub fn p0_smoke_risks(&self) -> impl Iterator<Item = &RiskEntry> {
        self.risks.iter().filter(|r| r.p0_smoke)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error("unsupported schema: {0}")]
    UnsupportedSchema(String),
    #[error("expected {expected} {family} risks, found {actual}")]
    RiskCount {
        family: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("risk {0} has no hooks")]
    MissingHooks(String),
    #[error("risk {0} has no test_id_prefix")]
    MissingTestPrefix(String),
    #[error("risk {0} has no challenges")]
    MissingChallenges(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_repo_register_has_20_risks() {
        let reg = RiskRegister::load_default_repo_file().expect("catalog file");
        assert_eq!(reg.risks.len(), 20);
        assert_eq!(reg.version, "1.0.0");
    }

    #[test]
    fn p0_smoke_covers_plan_categories() {
        let reg = RiskRegister::load_default_repo_file().unwrap();
        let p0: Vec<_> = reg.p0_smoke_risks().map(|r| r.id.as_str()).collect();
        assert!(p0.contains(&"LLM01:2025"));
        assert!(p0.contains(&"ASI05:2026"));
        assert!(reg.get("LLM02:2025").unwrap().test_id_prefix == "AC-LLM02");
    }

    #[test]
    fn each_risk_maps_to_cs01_or_cs02() {
        let reg = RiskRegister::load_default_repo_file().unwrap();
        for risk in &reg.risks {
            assert!(
                !risk.challenges.is_empty(),
                "{} must list CS01/CS02/both",
                risk.id
            );
        }
    }
}
