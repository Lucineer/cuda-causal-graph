//! cuda-causal-graph: Causal inference graph with counterfactual reasoning.
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalNode { pub id: String, pub label: String, pub confidence: f64, pub causes: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalPath { pub nodes: Vec<String>, pub total_confidence: f64, pub path_length: usize }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult { pub hypothesis: String, pub supporting_evidence: Vec<String>, pub counter_evidence: Vec<String>, pub strength: f64 }

pub struct CausalGraph { nodes: HashMap<String, CausalNode> }
impl CausalGraph {
    pub fn new() -> Self { Self { nodes: HashMap::new() } }
    pub fn add_node(&mut self, id: &str, label: &str, confidence: f64, causes: Vec<&str>) {
        self.nodes.insert(id.into(), CausalNode { id: id.into(), label: label.into(), confidence, causes: causes.iter().map(|s| s.to_string()).collect() });
    }
    pub fn trace(&self, from: &str) -> Vec<CausalPath> {
        let mut paths = Vec::new(); let mut current = vec![vec![from.to_string()]];
        while let Some(path) = current.pop() {
            let last = path.last().unwrap().clone();
            let effects: Vec<String> = self.nodes.values().filter(|n| n.causes.contains(&last)).map(|n| n.id.clone()).collect();
            if effects.is_empty() { let conf: f64 = path.iter().map(|n| self.nodes.get(n).map(|nd| nd.confidence).unwrap_or(0.0)).product(); paths.push(CausalPath { nodes: path, total_confidence: conf, path_length: path.len() }); }
            else { for e in effects { let mut p = path.clone(); p.push(e); current.push(p); } }
        }
        paths
    }
    pub fn infer(&self, hypothesis: &str) -> InferenceResult {
        let paths = self.trace(hypothesis);
        let supporting: Vec<String> = paths.iter().filter(|p| p.total_confidence > 0.5).flat_map(|p| p.nodes.clone()).collect();
        let counter: Vec<String> = paths.iter().filter(|p| p.total_confidence <= 0.5).flat_map(|p| p.nodes.clone()).collect();
        let strength = if paths.is_empty() { 0.0 } else { paths.iter().map(|p| p.total_confidence).sum::<f64>() / paths.len() as f64 };
        InferenceResult { hypothesis: hypothesis.into(), supporting_evidence: supporting, counter_evidence: counter, strength }
    }
    pub fn counterfactual(&self, node_id: &str, removed_cause: &str) -> f64 {
        if let Some(node) = self.nodes.get(node_id) {
            let active_causes: Vec<&String> = node.causes.iter().filter(|c| c.as_str() != removed_cause).collect();
            if active_causes.is_empty() { return node.confidence * 0.1; }
            node.confidence * (active_causes.len() as f64 / node.causes.len() as f64)
        } else { 0.0 }
    }
    pub fn node_count(&self) -> usize { self.nodes.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_graph() -> CausalGraph {
        let mut g = CausalGraph::new();
        g.add_node("rain", "Rainfall", 0.9, vec![]);
        g.add_node("wet", "Wet ground", 0.8, vec!["rain"]);
        g.add_node("slip", "Slippery", 0.7, vec!["wet"]);
        g
    }
    #[test] fn test_trace() { let g = make_graph(); let paths = g.trace("rain"); assert_eq!(paths.len(), 2); assert_eq!(paths[0].path_length, 3); }
    #[test] fn test_confidence_product() { let g = make_graph(); let paths = g.trace("rain"); assert!((paths[0].total_confidence - (0.9 * 0.8 * 0.7)).abs() < 0.01); }
    #[test] fn test_counterfactual() { let g = make_graph(); let orig = g.nodes.get("slip").unwrap().confidence; let without = g.counterfactual("slip", "wet"); assert!(without < orig); }
    #[test] fn test_infer() { let g = make_graph(); let r = g.infer("rain"); assert!(r.strength > 0.5); }
    #[test] fn test_empty() { let g = CausalGraph::new(); assert_eq!(g.node_count(), 0); }
}