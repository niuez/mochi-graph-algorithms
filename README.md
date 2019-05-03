# mochi-graph-algorithms

[![CircleCI](https://circleci.com/gh/kutimoti/mochi-graph-algorithms.svg?style=svg)](https://circleci.com/gh/kutimoti/mochi-graph-algorithms)

mochi-graph-algorithms is the library of graphs abstraction by Rust.

## algorithms

### single\_source\_shortest\_path

- bfs (only unit length)
  O(V + E)
- dijkstra(with binary heap) 
  O((V + E)logV)
- dijkstra(with radix heap 32)
  O(E log log V)
- dijkstra(with radix heap 64)
  O(E log log V)
- bellman\_ford 
  O(VE)
- spfa
  O(VE) faster than BF
- dial
  O(E + V * Wmax)
- scaling\_dijkstra
  O(Elog(Wmax))

### all\_pairs\_shortest\_path

- feasible\_potential
  O(VE)
- warshall\_floyd
  O(V^3)
- dijkstra\_with\_potential
  O(VE + V(V + E)logV)

### k-th\_shortest\_paths

- yen
  O(k ( E log V ))
- eppistein
  O(E log V + k)

### minimum\_spanning\_tree

- prim
  O(E log V)
- kruskal
  O(E log V + E α(V))
- boruvka
  O(E log V)

### maxflow

- dinic
  O(V^2E)
- fifo\_push\_relabel
  O(V^3)
- ford\_fulkerson
  O(EF)
- fujishige
  O(VElog(Cmax))

### minimum\_cost\_flow

- successive shortest paths(primal dual?)
  O(VE + FElogV)

### cardinality\_bipartite\_matching

- hopcroft\_karp
  O(V^(1/2)E)

### cardinality\_general\_matching

- gabow\_e\_algorithm
  O(VElogV)


## TODO

- RHS-algorithm(min cost flow)
- Orlin scaling algorithm (min cost flow) (difficult)
- min cost circulation
- min cost transshipment
- mst
- wbm and wnbm
