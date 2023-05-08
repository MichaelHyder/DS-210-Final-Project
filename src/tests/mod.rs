use crate::read_and_make::analysis::*;
use crate::read_and_make::create_adjacency_matrix;
use std::collections::HashSet;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
use super::*;
use std::collections::HashSet;
use std::collections::HashMap;
 #[test]
     fn test_create_adjacency_matrix() {
        let edges = vec![(1, 2), (2, 3), (3, 4), (4, 1)];
        let adjacency_matrix = create_adjacency_matrix(&edges);

        assert_eq!(adjacency_matrix.get(&1).unwrap().get(&2), Some(&true));
        assert_eq!(adjacency_matrix.get(&2).unwrap().get(&1), Some(&true));
        assert_eq!(adjacency_matrix.get(&2).unwrap().get(&3), Some(&true));
        assert_eq!(adjacency_matrix.get(&3).unwrap().get(&2), Some(&true));
        assert_eq!(adjacency_matrix.get(&3).unwrap().get(&4), Some(&true));
        assert_eq!(adjacency_matrix.get(&4).unwrap().get(&3), Some(&true));
        assert_eq!(adjacency_matrix.get(&4).unwrap().get(&1), Some(&true));
        assert_eq!(adjacency_matrix.get(&1).unwrap().get(&4), Some(&true));

        assert_eq!(adjacency_matrix.get(&1).unwrap().get(&3), None);
        assert_eq!(adjacency_matrix.get(&3).unwrap().get(&1), None);
        assert_eq!(adjacency_matrix.get(&2).unwrap().get(&4), None);
        assert_eq!(adjacency_matrix.get(&4).unwrap().get(&2), None);
    }

#[test]
     fn test_closeness_centrality() {
        let mut adjacency_matrix: HashMap<usize, HashMap<usize, bool>> = HashMap::new();
        adjacency_matrix.insert(1, HashMap::new());
        adjacency_matrix.insert(2, HashMap::new());
        adjacency_matrix.insert(3, HashMap::new());
        adjacency_matrix.insert(4, HashMap::new());
        
        adjacency_matrix.get_mut(&1).unwrap().insert(2, true);
        adjacency_matrix.get_mut(&2).unwrap().insert(1, true);
        adjacency_matrix.get_mut(&2).unwrap().insert(3, true);
        adjacency_matrix.get_mut(&3).unwrap().insert(2, true);
        adjacency_matrix.get_mut(&3).unwrap().insert(4, true);
        adjacency_matrix.get_mut(&4).unwrap().insert(3, true);
        adjacency_matrix.get_mut(&4).unwrap().insert(1, true);
        adjacency_matrix.get_mut(&1).unwrap().insert(4, true);
        
        let closeness_values = closeness_centrality(&adjacency_matrix);
        
        let expected_values: Vec<(usize, f32)> = vec![(1, 0.75), (2, 0.75), (3, 0.75), (4, 0.75)];
 	for (expected_node, expected_closeness) in expected_values.iter() {
            let (actual_node, actual_closeness) = closeness_values.iter().find(|(node, _)| *node == *expected_node).unwrap();
            assert_eq!(actual_node, expected_node);
        }
    }

 #[test]
     fn test_find_matching_values() {
        let vec1: Vec<(usize, f32)> = vec![(1, 0.5), (2, 0.3), (3, 0.8)];
        let vec2: Vec<(usize, f32)> = vec![(2, 0.2), (3, 0.4), (4, 0.6)];
        
        let result = find_matching_values(vec1, vec2);
        
        let expected_result: Vec<(usize, usize)> = vec![(0, 2), (0, 3)];
        
        assert_eq!(result, expected_result);
    }
}
