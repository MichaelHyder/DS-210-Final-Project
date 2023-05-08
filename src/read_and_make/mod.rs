use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

pub fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(|c: char| c == ' ' || c == '\t').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[v.len()-1].parse::<usize>().unwrap();
        result.push((x, y));
    }
    return result;
}

pub fn create_adjacency_matrix(edges: &Vec<(usize, usize)>) -> HashMap<usize, HashMap<usize, bool>> {
    let mut adjacency_matrix: HashMap<usize, HashMap<usize, bool>> = HashMap::new();
    
    for &(node1, node2) in edges {
        adjacency_matrix
            .entry(node1)
            .or_insert_with(HashMap::new)
            .insert(node2, true);
        
        adjacency_matrix
            .entry(node2)
            .or_insert_with(HashMap::new)
            .insert(node1, true);
    }
    
    adjacency_matrix
}

 pub mod analysis;