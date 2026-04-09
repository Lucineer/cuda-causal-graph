# cuda-causal-graph

GPU-accelerated causal graph for failure diagnosis.

Builds and queries directed acyclic graphs of causal relationships
to diagnose why something failed and what to fix.

## CUDA Acceleration
- Parallel graph traversal (BFS/DFS) on GPU
- Batched causal chain computation
- Real-time root cause analysis