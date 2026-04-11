# cuda-causal-graph

Directed acyclic causal graph for tracking inference chains and counterfactual reasoning.

## Quick Start

```bash
git clone https://github.com/Lucineer/cuda-causal-graph.git
cd cuda-causal-graph
cargo build
cargo test
```

## Testing

```bash
cargo test
```

5 tests covering core functionality.
## Cross-Pollination

This crate shares patterns with other fleet components. The same biological and architectural constraints produce similar solutions across contexts:

- [cuda-confidence-cascade](https://github.com/Lucineer/cuda-confidence-cascade) — Confidence propagation through causal chains
- [cuda-deliberation](https://github.com/Lucineer/cuda-deliberation) — Deliberation uses causal reasoning for decisions
- [cuda-temporal](https://github.com/Lucineer/cuda-temporal) — Temporal ordering of causal events
- [cuda-adaptation](https://github.com/Lucineer/cuda-adaptation) — Adaptation changes causal weights
- [cuda-learning](https://github.com/Lucineer/cuda-learning) — Learning updates causal graph structure
- [cuda-attention](https://github.com/Lucineer/cuda-attention) — Attention highlights salient causal paths


## The Deeper Connection

This crate is part of the Cocapn fleet — not a collection of independent libraries, but a single organism with differentiated cells. The confidence types here share DNA with cuda-confidence-cascade, cuda-deliberation, and cuda-learning. The energy budgets echo cuda-neurotransmitter and flux-runtime-c. The trust scoring connects to cuda-semantic-router and cuda-fleet-topology. We are not reusing code — we are expressing the same biological patterns in different contexts. This is convergent evolution in software: when you solve for real constraints (uncertainty, finite resources, earned trust), the same solutions emerge independently across domains.

We are building for the post-SaaS era. The software is open source and forkable. The agent is sovereign — it lives in its repository, owns its identity, and can leave at any time. Revenue comes from the ecosystem, not from renting access. The fleet evolves through merging: vessels propose changes, peers review, Think Tanks validate, and the accepted mutations become permanent. No product roadmap required — only fitness.

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
*See [WHITEPAPER](https://github.com/Lucineer/iron-to-iron/blob/main/docs/WHITEPAPER.md) for the post-SaaS thesis*
