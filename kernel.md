# kernel

`kernel`は, グラフアルゴリズムで扱う要素(グラフ構造, 頂点, 辺, 重みなど)が満たすべきTraitを定義する場所です.  
`kernel`は, グラフ構造の実装方針, アルゴリズムの実装部分の抽象化を手助けるものになっています.

# kernel::graph

ここでは, グラフ抽象化に必要なTraitを定義しています.

## `trait ID`

`trait ID`は要素に以下の実装を要求します.

- `fn id(&self) -> usize` ... そのオブジェクトの番号

頂点や辺の重みを管理するとき配列を使うことがありますが, `weight[edge.id]`とするとあまり美しくありません.  
これを解消するために`ID`を使い, `Properties`によって管理することで扱いやすくします. 詳しくは`Properties`を参照してください.

実際に`usize`について`ID`を実装すると

```rust
impl ID for usize {
    fn id(&self) -> usize { *self }
}
```

となります(実際に実装されています).

## `trait Vertex`

`trait Vertex`は頂点を抽象化するためのTraitです. `Vertex`は

- `trait ID` ... 番号を持っていること
- `trait Eq` ... 2つの頂点が等しいかどうかが判定が可能
- `trait Copy` ... コピーが可能

を要求します. また, これらを満たす型全てに`Vertex`が実装されるようになります. (`impl<V: ID + Eq + Copy> Vertex for V { }`)

## `trait Edge`

`trait Edge`は辺を抽象化するためのTraitです. 以下の実装を要求します.

- `type VType: Vertex` ... 辺が結ぶ頂点の型(`Vertex`である必要がある)
- `type from(&self) -> &Self::VType` ... 辺の始点の頂点の参照
- `type to(&self) -> &Self::VType` ... 辺の終点の頂点の参照

例えば, タプル`(V, V)`に`Edge`を実装すると

```rust
impl<V> Edge for (V, V) where V: Vertex { 
    type VType = V;
    fn from(&self) -> &Self::VType { &self.0 }
    fn to(&self) -> &Self::VType { &self.1 }
}
```

このライブラリでは, `(V, V)`と`(V, V, P) Pはなんらかのプロパティ`について`Edge`が実装されています.

## `trait AdjEdge`

`trait AdjEdge`は隣接リストで使う辺を抽象化したものです. 以下の実装を要求します.

- `trait Edge` ... 辺であること.
- `trait ID` ... 番号を持っていること.
- `type EType: Edge<VType=Self::VType>` ... AdjEdgeが持っている辺の型
- `fn edge(&self) -> &Self::EType` ... もともとの辺の参照

これを導入することで, 無向グラフの辺をコピーすることなく扱うことができたり, 残余グラフでの逆辺の参照を実装することができるようになります.

## `trait ResidualEdge`

`trait ResidualEdge`は残余グラフで使う, 逆辺の参照ができる`AdjEdge`です. 以下の実装を要求します.

- `trait AdjEdge` ... AdjEdgeであること.
- `fn rev(&self) -> Self` ... 逆辺の参照を返す.

`ResidualGraph`の実装に使われます.

## `Trait Graph`

`trait Graph`はグラフを抽象化したものです. 以下の実装を要求します.

- `type VType: Vertex + 'a;` ... 頂点の型
- `type EType: Edge<VType=Self::VType>;` ... 辺の型
- `type AEType: AdjEdge<VType=Self::VType, EType=Self::EType>;` ... 隣接リストの辺の型
- `type AdjIter: std::iter::Iterator<Item=Self::AEType>;` ... 隣接リストのイテレータの型
- `type EIter: std::iter::Iterator<Item=Self::AEType>;` ... すべての辺リストのイテレータの型 
- `type VIter: std::iter::Iterator<Item=&'a Self::VType>;` ... すべての頂点リストのイテレータの型
- `fn delta(&'a self, v: &Self::VType) -> Self::AdjIter;` ... 頂点`v`の隣接する辺のイテレータを返す
- `fn edges(&'a self) -> Self::EIter;` ... すべての辺のリストのイテレータを返す
- `fn vertices(&'a self) -> Self::VIter;` ... すべての頂点のリストのイテレータを返す
- `fn v_size(&self) -> usize;` ... 頂点の数
- `fn e_size(&self) -> usize;` ... 辺の数

## `Trait Directed`

`trait Directed`は有向グラフを表すTraitです. 要求する関数はなく, だた有向グラフであることを保証するだけです.

## `Trait Undirected`

`trait Undirected`は無向グラフを表すTraitです. こちらも要求する関数はなく, 保証するだけです.

## `Trait Bipartite`

`trait Bipartite`は二部グラフを表すTraitです. 以下の実装を要求します.

- `type BVIter: std::iter::Iterator<Item=&'a Self::VType>;` ... 片側の頂点のリストのイテレータの型
- `fn left_vertices(&'a self) -> Self::BVIter;` ... 左側の頂点のリストのイテレータを返す.
- `fn right_vertices(&'a self) -> Self::BVIter;` ... 右側の頂点のリストのイテレータを返す.

## `Trait Residual`

`trait Residual`は残余グラフを表すTraitです. 隣接リストの辺は`ResidualEdge`である必要があります.

# kernel::property

ここでは, グラフに乗せる重みやプロパティが満たすべきTraitを定義しています.

## `trait Property`

`trait Property`は値を表します. `Copy`であることを要求します. また, `Copy`である方はすべて`Property`として扱うことができます.

## `trait ArbWeight`

`trait ArbWeight`は加算, 比較可能な`Property`を表すTraitです. 以下の実装を要求します.

- `trait ToNNegWeight` ... このあと説明します
- `trait ToArbWeight` ... このあと説明します
- `trait Property`
- `std::ops::Add<Output=Self>` ... 加算可能
- `std::cmp::Ord` ... 比較可能
- `fn inf() -> Self` ... +∞
- `fn zero() -> Self` ... 0
- `fn neg_inf() -> Self` ... -∞

## `trait NNegWeight`

`trait NNegWeight`は非負重みであることを保証した`ArbWeight`です.

アルゴリズムには, 非負重みで有ることを要求するものがいくつかあります. `Dijkstra's Algorithm`などがあります. これには`NNegWeight`しか使えないようにしてあります.

しかし, 実行可能ポテンシャルを使った`Dijkstra's Algorithm`では, 重みが負であってもポテンシャルによって正にすることができます. しかしこのままでは, 正であるあることがアルゴリズム的に保証されていようが`Dijkstra's Algorithm`を適用することはできません.

それを解決するのが次の２つです.

## `trait ToNNegWeight`

`trait ToNNegWeight`はその重みを非負に変換する操作の実装をすることを要求するTraitです. 以下の実装を要求します.

- `type Output: NNegWeight` ... 変換先の型
- `fn to_nnegw(&self) -> Self::Output` ... 非負に変換する

## `trait ToArbWeight`

`trait ToArbWeight`はその重みを非負ではない重みに変換する操作の実装をすることを要求するTraitです. 以下の実装を要求します.

- `type Output: ArbWeight` ... 変換先の型
- `fn to_arbw(&self) -> Self::Output` ... 変換する

## `trait SubtractableWeight`

`trait SubtractableWeight`は減算が可能である`ArbWeight`です. 以下の実装を要求します.

- `trait std::ops::Sub<Output=Self>` ... 減算可能

# kernel::Properties

`Properties`は頂点や辺(`ID`を実装しているもの)に与えるプロパティを管理するものです. 例を挙げます.

```rust
let mut dist = Properties::new(n, &W::inf());
for ref e in g.delta(v) {
  if dist[e.from()] + cost(e) < dist[e.to()] {
    dist[e.to()] = dist[e.from()] + cost(e);
  }
}
```

最短距離を求めるときのコードの一部です. `e`は`AdjEdge`なので`ID`を持っているのでこれが可能です.  
コードが美しくなります.
