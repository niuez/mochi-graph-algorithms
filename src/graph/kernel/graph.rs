use graph::kernel::property::*;

/// Trait for elements of graph (Vertex, Edge, ...) that have ID (usize).
/// the elements implementing ID are able to use [`graph::kernel::Properties`].
pub trait ID {

    /// return id of the element.
    fn id(&self) -> usize;
}

/// Implementing ID for usize.
impl ID for usize {

    /// return the own value
    fn id(&self) -> usize { *self }
}

/// Trait for vertices of graphs.
pub trait Vertex: ID + Eq + Copy { }

impl<V: ID + Eq + Copy> Vertex for V { }

/// Trait for edges of graphs. 
pub trait Edge {

    /// Vertex type at both ends of edge
    type VType: Vertex;

    /// Start point of edge
    fn from(&self) -> &Self::VType;

    /// End point of edge
    fn to(&self) -> &Self::VType;
}

/// Implementing Edge for the simple tuple. 
impl<V> Edge for (V, V) where V: Vertex { 
    type VType = V;
    fn from(&self) -> &Self::VType { &self.0 }
    fn to(&self) -> &Self::VType { &self.1 }
}

/// Implementing Edge for the simple tuple. 
impl<V, P> Edge for (V, V, P) where V: Vertex, P: Property { 
    type VType = V;
    fn from(&self) -> &Self::VType { &self.0 }
    fn to(&self) -> &Self::VType { &self.1 }
}

/// Trait for adjacency edges of graph.
/// Why do we use [`Edge`] as is? There are 2 reasons.
/// - To give values to the edges to use Properties (AdjEdge has ID).
/// - When using a undirected graph as a directed graph, must swap two ends of edge.
pub trait AdjEdge: ID + Edge + Copy {

    /// Edge type of raw edge.
    type EType: Edge<VType=Self::VType>;

    /// return raw edge.
    fn edge(&self) -> &Self::EType;
}

/// Trait for adjcency edges on ResidualNetwork.
/// It has reverse edge.
pub trait ResidualEdge: AdjEdge {
    fn rev(&self) -> Self;
}

/// Trait of graph.
pub trait Graph<'a> {
    /// Type of vertices.
    type VType: Vertex + 'a;

    /// Type of edges.
    type EType: Edge<VType=Self::VType>;

    /// Type of adjacency edges.
    type AEType: AdjEdge<VType=Self::VType, EType=Self::EType>;

    /// Type of iterator for adjacency list.
    type AdjIter: std::iter::Iterator<Item=Self::AEType>;

    /// Type of iterator for edges list.
    type EIter: std::iter::Iterator<Item=Self::AEType>;

    /// Type of iterator for vertices list.
    type VIter: std::iter::Iterator<Item=&'a Self::VType>;

    /// return adjacency list from the vertex v.
    fn delta(&'a self, v: &Self::VType) -> Self::AdjIter;

    /// return edges list.
    fn edges(&'a self) -> Self::EIter;

    /// return vertices list.
    fn vertices(&'a self) -> Self::VIter;

    /// return the number of vertices.
    fn v_size(&self) -> usize;

    /// return the number of edges.
    fn e_size(&self) -> usize;
}

/// Trait of directed graph.
pub trait Directed<'a>: Graph<'a> {}

/// Trait of undirected graph.
/// graphs implementing this hold that the edge `(v, u)` exists for the edge `(u, v)` when the graph
/// use as directed graph
pub trait Undirected<'a>: Graph<'a> {}

/// Trait of bipartite graph.
pub trait Bipartite<'a>: Undirected<'a> {

    /// Type of iterator for vertices in one side.
    type BVIter: std::iter::Iterator<Item=&'a Self::VType>;

    /// return vertices list in left side.
    fn left_vertices(&'a self) -> Self::BVIter;

    /// return vertices list in right side.
    fn right_vertices(&'a self) -> Self::BVIter;
}

/// Trait of residual network
/// `AEType` must be `ResidualEdge`.
pub trait Residual<'a>: Directed<'a> where <Self as Graph<'a>>::AEType: ResidualEdge {}

pub fn generate_func<AE, P, F>(f: F) -> impl Fn(&AE) -> P
where AE: AdjEdge, P: Property, F: Fn(&AE::EType) -> P {
    move |ae| f(ae.edge())
}
