use std::fmt::Debug;
use std::fmt::Display;
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub struct Vertex<T, V> 
    where T: Display
{
    pub value: V,
    pub id: T,
}

impl<T, V> PartialEq for Vertex<T, V> 
    where V: PartialEq, T: Display {
    fn eq(&self, other: &Vertex<T, V>) -> bool {
        return self.value == other.value;
    }
}


impl<T, V> PartialOrd for Vertex<T, V> 
    where V: PartialOrd, T: Display {
    fn partial_cmp(&self, other: &Vertex<T, V>) -> Option<Ordering> {
        return self.value.partial_cmp(&other.value);
    }
}

// Edge Struct
pub struct Edge<T> {
    pub weight: f32,
    pub from: T, 
    pub to: T,
}

impl<T> Edge<T> {
    pub fn id(&self) -> &T {
        return &self.to;
    }

    pub fn weight(&self) -> f32 {
        return self.weight;
    }
}

// Sparse adjacency list graph
pub struct SparseGraph<T> {
    edges: HashMap<T, Vec<Edge<T>>>,
}

pub fn build_sparse_graph<T>(nodes: Vec<T>) -> SparseGraph<T> 
    where T: Hash + Copy + Eq + Debug + Display {
    let edges: HashMap<T, Vec<Edge<T>>> = { 
        nodes
        .iter()
        .map(|v| (*v, vec![]))
        .collect()
    };
    return SparseGraph{ edges };
}

impl<T> SparseGraph<T> 
    where T: Hash + Copy + Eq + Debug + Display {

    pub fn connect(&mut self, v1: T, v2: T, weight: f32) {
        self.edges.entry(v1).or_insert(vec![]).push(Edge{weight, from: v1, to: v2});
    }

    pub fn connect_all(&mut self, edges: Vec<Edge<T>>) {
        for e in edges {
            println!("Connect node {0} to {1}, weight={2}", e.from, e.to, e.weight);
            self.connect(e.from, e.to, e.weight);
        }
    }

    pub fn neighbors(&self, node: T) -> Option<&Vec<Edge<T>>> {
        return self.edges.get(&node);
    }

}

// Dense matrix-based graph
pub struct DenseGraph<T> {
    weights: Vec<Vec<f32>>,
    labels: HashMap<T, usize>
}

pub fn build_dense_graph<T>(nodes: Vec<T>) -> DenseGraph<T> 
    where T: Hash + Copy + Eq + Debug {
    let labels: HashMap<_, usize> = { 
        nodes
        .iter()
        .enumerate()
        .into_iter()
        .map(|(v, k)| (*k, v))
        .collect()
    };
    return DenseGraph{
        weights: vec![vec![0.0; nodes.len()]; nodes.len()],
        labels, 
    }
}

impl<T> DenseGraph<T> 
    where T: Hash + Copy + Eq + Debug {

    pub fn connect(&mut self, v1: T, v2: T, weight: f32) {
        let i;
        let j;
        match self.labels.get(&v1) {
            Some(name) => i = *name,
            None => return,
        }
        match self.labels.get(&v2) {
            Some(name) => j = *name,
            None => return,
        }
        self.weights[i][j] = weight;
    }

    pub fn connect_all(&mut self, edges: Vec<Edge<T>>) {
        let _ = edges.iter().map(|e| self.connect(e.from, e.to, e.weight));
    }

    pub fn neighbors(&self, node: usize) -> Vec<(usize, f32)> {
        return {
            self.weights[node]
            .iter()
            .cloned()
            .enumerate()
            .into_iter()
            .filter(|(_i, w)| w != &0.0)
            .collect()
        };
    }
}
