# mochi-graph-algorithms

[![CircleCI](https://circleci.com/gh/kutimoti/mochi-graph-algorithms.svg?style=svg)](https://circleci.com/gh/kutimoti/mochi-graph-algorithms)

## algorithms

### single\_source\_shortest\_path

- bfs (only unit length)
  O(V + E)
- dijkstra(with binary heap) 
  O((V + E)logV)
- bellman\_ford 
  O(VE)
- spfa
  O(VE) faster than BF
- dial
  O(E + V * Wmax)
- scaling\_dijkstra
  O(Elog(Wmax))

## all\_pairs\_shortest\_path

- Warshall-Floyd
  O(V^3)
- dijkstra\_with\_potential
  O(V(V + E)logV)

### maxflow

- dinic
  O(V^2E)
- fifo\_push\_relabel
  O(V^3)
- ford\_fulkerson
  O(EF)
- fujishige
  O(VElog(Cmax))

### cardinality\_bipartite\_matching

- hopcroft\_karp
  O(V^(1/2)E)

### cardinality\_general\_matching

- gabow\_e\_algorithm
  O(VElogV)


## TODO

- RHS-algorithm(maxflow)
- Orlin scaling algorithm (maxflow) (difficult)
- primal dual (mcstf)
- min cost circulation
- min cost transshipment
- mst
- wbm and wnbm
