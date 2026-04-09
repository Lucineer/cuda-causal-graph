//! cuda-causal-graph — GPU causal reasoning

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType { Event, Observation, RootCause, Mitigation }

#[derive(Debug)]
pub struct DiagnosisResult { pub root_causes: Vec<(String, f64)>, pub causal_chain: Vec<String>, pub confidence: f64 }

pub struct CausalGraph {
    nodes: HashMap<String, NodeType>, edges: Vec<(String, String, f64, f64)>,
    adj: HashMap<String, Vec<String>>, rev: HashMap<String, Vec<String>>,
}

impl CausalGraph {
    pub fn new() -> Self { CausalGraph { nodes: HashMap::new(), edges: Vec::new(), adj: HashMap::new(), rev: HashMap::new() } }
    pub fn add_node(&mut self, id: &str, nt: NodeType) { self.nodes.insert(id.to_string(), nt); }
    pub fn add_edge(&mut self, from: &str, to: &str, w: f64, e: f64) {
        self.edges.push((from.to_string(), to.to_string(), w, e));
        self.adj.entry(from.to_string()).or_default().push(to.to_string());
        self.rev.entry(to.to_string()).or_default().push(from.to_string());
    }
    pub fn diagnose(&self, obs: &str) -> DiagnosisResult {
        let mut visited = HashSet::new(); let mut queue = VecDeque::new();
        let mut chain = Vec::new(); let mut roots = Vec::new();
        queue.push_back(obs.to_string()); visited.insert(obs.to_string());
        while let Some(cur) = queue.pop_front() {
            if let Some(&nt) = self.nodes.get(&cur) {
                if nt == NodeType::RootCause { roots.push((cur.clone(), self.evidence(&cur))); }
            }
            chain.push(cur.clone());
            if let Some(parents) = self.rev.get(&cur) {
                for p in parents { if !visited.contains(p) { visited.insert(p.clone()); queue.push_back(p.clone()); } }
            }
        }
        roots.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        DiagnosisResult { root_causes: roots, causal_chain: chain, confidence: roots.first().map(|(_,c)|*c).unwrap_or(0.0) }
    }
    fn evidence(&self, node: &str) -> f64 { self.edges.iter().filter(|(_,t,_,_)| t == node).map(|(_,_,w,e)| w*e).sum() }
    pub fn topo_sort(&self) -> Vec<String> {
        let mut deg: HashMap<&str,usize> = HashMap::new();
        for n in self.nodes.keys() { deg.entry(n.as_str()).or_insert(0); }
        for (_,to,_,_) in &self.edges { *deg.entry(to.as_str()).or_insert(0) += 1; }
        let mut q: VecDeque<&str> = deg.iter().filter(|(_,&d)| d==0).map(|(&k,_)| k).collect();
        let mut res = Vec::new();
        while let Some(n) = q.pop_front() { res.push(n.to_string());
            if let Some(ch) = self.adj.get(n) { for c in ch { if let Some(d) = deg.get_mut(c.as_str()) { *d-=1; if *d==0 { q.push_back(c); } } } }
        }
        res
    }
}

#[cfg(test)]
mod tests { use super::*;
    #[test] fn test_diagnosis() {
        let mut g = CausalGraph::new();
        g.add_node("temp", NodeType::RootCause); g.add_node("fan", NodeType::RootCause);
        g.add_node("heat", NodeType::Event); g.add_node("shutdown", NodeType::Observation);
        g.add_edge("temp", "heat", 0.8, 0.9); g.add_edge("fan", "heat", 0.7, 0.8);
        g.add_edge("heat", "shutdown", 0.9, 0.95);
        let d = g.diagnose("shutdown"); assert!(!d.root_causes.is_empty());
    }
}
