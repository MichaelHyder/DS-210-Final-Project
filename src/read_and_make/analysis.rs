use std::collections::HashSet;
use std::collections::HashMap;


pub fn find_num_vertices(vector: &Vec<(usize, usize)>) -> usize {

    let mut vertices = HashSet::new();
    
    for &(value1, value2) in vector {
        vertices.insert(value1);
    }
    
    vertices.len()
}

pub fn calculate_betweenness(adjacency_matrix: &HashMap<usize, HashMap<usize, bool>>) -> Vec<(usize, f32)> {
    let mut betweenness: HashMap<usize, f32> = HashMap::new();

    for &node in adjacency_matrix.keys() {
        let mut stack: Vec<usize> = Vec::new();
        let mut distance: HashMap<usize, isize> = HashMap::new();
        let mut paths: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
        let mut sigma: HashMap<usize, f32> = HashMap::new();
        let mut delta: HashMap<usize, f32> = HashMap::new();

        for &v in adjacency_matrix.keys() {
            distance.insert(v, -1);
            paths.insert(v, Vec::new());
            sigma.insert(v, 0.0);
            delta.insert(v, 0.0);
        }

        distance.insert(node, 0);
        sigma.insert(node, 1.0);

        let mut queue: Vec<usize> = vec![node];
        while let Some(v) = queue.pop() {
            stack.push(v);

            if let Some(neighbors) = adjacency_matrix.get(&v) {
                for (&w, _) in neighbors.iter() {
                    if *distance.get(&w).unwrap() == -1 {
                        queue.push(w);
                        *distance.entry(w).or_insert(0) = *distance.get(&v).unwrap() + 1;
                    }

                    if *distance.get(&w).unwrap() == *distance.get(&v).unwrap() + 1 {
                        *sigma.entry(w).or_insert(0.0) += *sigma.get(&v).unwrap();
                        paths.get_mut(&w).unwrap().push(vec![v, w]);
                    }
                }
            }
        }

        while let Some(v) = stack.pop() {
            for path in paths[&v].iter() {
                let w = path[0];

                *delta.entry(w).or_insert(0.0) += (*sigma.get(&w).unwrap() / *sigma.get(&v).unwrap())
                    * (1.0 + *delta.get(&v).unwrap());
            }

            if v != node {
                *betweenness.entry(v).or_insert(0.0) += *delta.get(&v).unwrap();
            }
        }
    }

    let betweenness_vec: Vec<(usize, f32)> = betweenness.into_iter().collect();
    betweenness_vec
}

pub fn sort_hashmap_by_value_descending(mut vec: Vec<(usize, f32)>) -> Vec<(usize, f32)> {
    vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    vec
}



pub fn closeness_centrality(adj_matrix: &HashMap<usize, HashMap<usize, bool>>) -> Vec<(usize, f32)> {
    let n = adj_matrix.len();
    let mut closeness_values: HashMap<usize, f32> = HashMap::new();

    for (node, _) in adj_matrix {
        let distances = shortest_paths(adj_matrix, *node);
        let sum_dist: usize = distances.values().sum();
        let closeness = (n - 1) as f32 / sum_dist as f32;

        closeness_values.insert(*node, closeness);
    }

    let mut closeness_values: Vec<(usize, f32)> = closeness_values.into_iter().collect();
    closeness_values.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    closeness_values
}

pub fn shortest_paths(adj_matrix: &HashMap<usize, HashMap<usize, bool>>, start: usize) -> HashMap<usize, usize> {
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue = std::collections::VecDeque::new();

    distances.insert(start, 0);
    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for neighbor in adj_matrix[&node].keys() {
            if !visited.contains(neighbor) {
                visited.insert(*neighbor);
                let distance = distances[&node] + 1;
                distances.insert(*neighbor, distance);
                queue.push_back(*neighbor);
            }
        }
    }

    distances
}

pub fn find_matching_values(vec1: Vec<(usize, f32)>, vec2: Vec<(usize, f32)>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    
    for (idx1, val1) in vec1 {
        for (idx2, val2) in vec2.iter() {
            if idx1 == *idx2 {
                result.push(((val1 * *val2) as usize , idx1));
                break;
            }
        }
    }
    
    result
}




