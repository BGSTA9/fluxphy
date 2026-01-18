//! Data Provenance Tracking for FluxPhy
//!
//! Implements a W3C PROV-O compliant provenance recorder using JSON-LD format.
//! Tracks agents (user/host), entities (files), and activities (transfer).

use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use sysinfo::System;

/// PROV-O JSON-LD Context
const PROV_CONTEXT: &str = "http://www.w3.org/ns/prov.jsonld";

/// Provenance Record Structure
#[derive(Debug, Serialize)]
pub struct ProvRecord {
    #[serde(rename = "@context")]
    pub context: HashMap<String, String>,
    #[serde(rename = "@graph")]
    pub graph: Vec<ProvElement>,
}

/// Helper enum for different PROV elements to allow flexible JSON structure
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ProvElement {
    #[serde(rename = "prov:Activity")]
    Activity(ProvActivity),
    #[serde(rename = "prov:Agent")]
    Agent(ProvAgent),
    #[serde(rename = "prov:Entity")]
    Entity(ProvEntity),
}

/// Represents the Transfer Activity
#[derive(Debug, Serialize)]
pub struct ProvActivity {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "prov:startedAtTime")]
    pub start_time:String,
    #[serde(rename = "prov:endedAtTime")]
    pub end_time: Option<String>,
    #[serde(rename = "prov:wasAssociatedWith")]
    pub was_associated_with: Vec<String>, // Refs to Agents
    #[serde(rename = "prov:used")]
    pub used: Vec<String>, // Refs to Source Entities
    #[serde(rename = "prov:generated")]
    pub generated: Vec<String>, // Refs to Dest Entities
    
    // Custom scientific metadata
    #[serde(rename = "flux:transferRateMean")]
    pub mean_rate: Option<f64>,
    #[serde(rename = "flux:flowRegime")]
    pub flow_regime: Option<String>,
}

/// Represents Agents (User, StartHost, Tool)
#[derive(Debug, Serialize)]
pub struct ProvAgent {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "foaf:name")]
    pub name: String,
    #[serde(rename = "flux:hostInfo")]
    pub host_info: Option<String>,
}

/// Represents Entities (Files)
#[derive(Debug, Serialize)]
pub struct ProvEntity {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "prov:wasAttributedTo")]
    pub was_attributed_to: String,
    #[serde(rename = "flux:path")]
    pub path: String,
    #[serde(rename = "flux:size")]
    pub size: u64,
    #[serde(rename = "nfo:fileHash")]
    pub checksum: Option<String>,
}

impl ProvRecord {
    /// Initialize a new provenance record
    pub fn new() -> Self {
        let mut context = HashMap::new();
        context.insert("prov".to_string(), "http://www.w3.org/ns/prov#".to_string());
        context.insert("flux".to_string(), "http://example.org/fluxphy/ns#".to_string());
        context.insert("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string());
        context.insert("foaf".to_string(), "http://xmlns.com/foaf/0.1/".to_string());
        context.insert("nfo".to_string(), "http://www.semanticdesktop.org/ontologies/2007/03/22/nfo#".to_string());

        Self {
            context,
            graph: Vec::new(),
        }
    }

    /// Save the provenance record to a JSON-LD file
    pub fn save(&self, path: &PathBuf) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

/// Helper to generate the current activity ID
pub fn generate_activity_id() -> String {
    format!("urn:uuid:{}", uuid::Uuid::new_v4())
}

/// Helper to capture current agent (user + host)
pub fn capture_agent() -> ProvAgent {
    let hostname = System::host_name().unwrap_or_else(|| "unknown-host".to_string());
    let username = whoami::username().unwrap_or_else(|_| "unknown-user".to_string());
    
    ProvAgent {
        id: format!("urn:agent:{}", username),
        name: username,
        host_info: Some(hostname),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provenance_structure() {
        let mut prov = ProvRecord::new();
        let agent = capture_agent();
        let agent_id = agent.id.clone();
        prov.graph.push(ProvElement::Agent(agent));

        let source_id = "urn:uuid:source".to_string();
        prov.graph.push(ProvElement::Entity(ProvEntity {
            id: source_id.clone(),
            was_attributed_to: agent_id.clone(),
            path: "/tmp/source".to_string(),
            size: 1024,
            checksum: None,
        }));

        let json = serde_json::to_string(&prov).unwrap();
        assert!(json.contains("@context"));
        assert!(json.contains("prov:Agent"));
        assert!(json.contains("prov:Entity"));
        assert!(json.contains("urn:agent:"));
    }
}
