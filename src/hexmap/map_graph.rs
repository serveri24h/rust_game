use std::{collections::HashMap, hash::Hash};

pub struct MapGraph<VId, E = (), V = ()> {
    vertices: HashMap<VId, V>,
    adjacency: HashMap<VId, Vec<(VId, E)>>
}

impl<VId, E, V> MapGraph<VId, E, V>
where
    VId: Eq + Hash + Clone, 
    V: Hash,
    E: Clone,
{
    pub fn new() -> MapGraph<VId, E, V> {
        MapGraph { vertices: HashMap::new(), adjacency: HashMap::new() }
    }

    pub fn push_vertex(self: &mut MapGraph<VId, E, V>, vid: VId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    pub fn push_oneway_edge(self: &mut Self, from: VId, to: VId, edge: E) {
        self.adjacency.entry(from).or_default().push((to, edge));
    }

    pub fn push_twoway_edge(self: &mut Self, v1: VId, v2: VId, edge: E) {
        self.push_oneway_edge(v1.clone(), v2.clone(), edge.clone());
        self.push_oneway_edge(v2, v1, edge);
    }

}

