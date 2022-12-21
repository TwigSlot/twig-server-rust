use std::collections::{HashMap, HashSet};

pub enum VertexKey {
    Tag(u32),
    Resource(u32),
    Project(u32),
}

/// A graph (like in discrete mathematics)
/// where vertices are of type `T`
struct Graph<T> {
    vertices: HashMap<VertexKey, T>,
    edges: HashMap<VertexKey, HashSet<VertexKey>>,
}

struct EdgeRelation {
    from: VertexKey,
    to: VertexKey,
}