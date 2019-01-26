# mochi-graph-algorithms

[![CircleCI](https://circleci.com/gh/kutimoti/mochi-graph-algorithms.svg?style=svg)](https://circleci.com/gh/kutimoti/mochi-graph-algorithms)

## algorithms

### single_source_shortest_path

- dijkstra(with binary heap) 
  O((V + E)logV)
- bellman_ford 
  O(VE)
- spfa
  O(VE) faster than BF
- d_any_heap_dijkstra
  O(E log_E/V_V)
- dial
  O(E + V * Wmax)
- scaling_dijkstra
  O(Elog(Wmax))

## all_pairs_shortest_path

- Warshall-Floyd
  O(V^3)

### maxflow

- dinic
  O(V^2E)
- fifo_push_relabel
  O(V^3)
- ford_fulkerson
  O(EF)
- fujishige
  O(VElog(Cmax))

### cardinality_bipartite_matching

- hopcroft_karp
  O(V^(1/2)E)

### cardinality_nonbipartite_matching

- gabow_e_algorithm
  O(VElogV)


## TODO

- RHS-algorithm(maxflow)
- potential dijkstra (apsp)
- Orlin scaling algorithm (maxflow) (difficult)
- primal dual (mcstf)
- min cost circulation
- min cost transshipment
- mst
- wbm and wnbm
