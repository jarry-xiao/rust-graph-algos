use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::priority_queue::build_priority_queue;
use crate::graph::build_sparse_graph;
use crate::graph::Vertex;
use crate::graph::Edge;
use crate::graph::SparseGraph;

mod priority_queue;
mod graph;


fn dijkstra<T>(g: &SparseGraph<T>, src: T) -> HashMap<T, f32>
    where T: Hash + Copy + Eq + Debug + Display
{
    let mut to_visit = build_priority_queue(None, None);
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    distances.insert(src, 0.0);
    to_visit.push(Vertex{value: 0.0, id: src});
    while let Some(Vertex{value: dist, id: node}) = to_visit.pop() {
        if !visited.insert(node) {
            continue;
        }
        if let Some(adj) = g.neighbors(node) {
            for neighbor in adj {
                let updated_dist = neighbor.weight() + dist;
                if distances.get(&neighbor.id()).map_or(true, |&current_dist| updated_dist < current_dist) {
                    distances.insert(*neighbor.id(), updated_dist);
                    to_visit.push(Vertex{value: updated_dist, id: *neighbor.id()});
                }
            }
        }
    }
    return distances;
}

fn bellman<T>(g: &SparseGraph<T>, src: T) -> HashMap<T, f32>
    where T: Hash + Copy + Eq + Debug + Display
{
    let mut distances = HashMap::new();
    distances.insert(src, 0.0);
    for _ in 0..g.size() {
        for node in g.nodes() {
            if g.neighbors(node).is_none() {
                continue;
            }
            for &Edge{weight, from: _, to} in g.neighbors(node).unwrap() {
                let updated_dist = distances.get(&node).unwrap_or(&std::f32::INFINITY) + weight;
                if updated_dist < *distances.get(&to).unwrap_or(&std::f32::INFINITY) {
                    distances.insert(to, updated_dist);
                }
            }
        }
    }
    // Run the algo to check for negative cycles
    for node in g.nodes() {
        if g.neighbors(node).is_none() {
            continue;
        }
        for &Edge{weight, from: _, to} in g.neighbors(node).unwrap() {
            let updated_dist = distances.get(&node).unwrap_or(&std::f32::INFINITY) + weight;
            if updated_dist < *distances.get(&to).unwrap_or(&std::f32::INFINITY) {
                distances.insert(to, std::f32::NEG_INFINITY);
            }
        }
    }
    return distances;
}

fn main() {
    let mut pq = build_priority_queue(None, None);
    pq.push(15);
    pq.print_data();
    pq.push(2);
    pq.print_data();
    pq.push(3);
    pq.print_data();
    pq.push(9);
    pq.print_data();
    pq.push(33);
    pq.print_data();
    pq.push(5);
    pq.print_data();
    pq.push(1);
    pq.print_data();

    let mut g = build_sparse_graph((0..10).collect());
    let mut edges: Vec<Edge<i32>> = vec![];
    edges.push(Edge{weight: 1.2, from: 0, to: 1});
    edges.push(Edge{weight: 5.2, from: 0, to: 2});
    edges.push(Edge{weight: 1.9, from: 0, to: 3});
    edges.push(Edge{weight: 10.9, from: 0, to: 4});
    edges.push(Edge{weight: 5.4, from: 0, to: 5});
    edges.push(Edge{weight: 8.1, from: 0, to: 6});
    edges.push(Edge{weight: 3., from: 0, to: 7});
    edges.push(Edge{weight: 1.8, from: 0, to: 8});
    edges.push(Edge{weight: 7.2, from: 0, to: 9});
    edges.push(Edge{weight: 2.2, from: 1, to: 2});
    edges.push(Edge{weight: 9.9, from: 1, to: 3});
    edges.push(Edge{weight: 11.9, from: 1, to: 4});
    edges.push(Edge{weight: 4.2, from: 1, to: 5});
    edges.push(Edge{weight: 10.1, from: 1, to: 6});
    edges.push(Edge{weight: 3., from: 1, to: 7});
    edges.push(Edge{weight: 0.8, from: 1, to: 8});
    edges.push(Edge{weight: 1.2, from: 1, to: 9});
    g.connect_all(edges);
    let distances = dijkstra(&g, 0);
    let mut distance_vec : Vec<(&i32, &f32)> = distances.iter().collect();
    distance_vec.sort_by(|a, b| a.0.cmp(b.0));
    for (node, dist) in distance_vec {
        println!("Distance from 0 to {0}: {1} (dijkstra)", node, dist);
    }

    let distances = bellman(&g, 0);
    let mut distance_vec : Vec<(&i32, &f32)> = distances.iter().collect();
    distance_vec.sort_by(|a, b| a.0.cmp(b.0));
    for (node, dist) in distance_vec {
        println!("Distance from 0 to {0}: {1} (bellman)", node, dist);
    }
}
