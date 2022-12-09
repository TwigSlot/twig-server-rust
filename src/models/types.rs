use std::collections::{HashMap, HashSet};

type ResourceId = u32;
type ProjectId = u32;
type TagId = u32;

type VertexKey = u32;

struct Graph<T> {
    vertices: HashMap<VertexKey, T>,
    edges: HashMap<VertexKey, HashSet<VertexKey>>,
}
