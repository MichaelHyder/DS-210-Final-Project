mod read_and_make;
mod tests;

fn main(){
    use read_and_make::read_file;
    use read_and_make::create_adjacency_matrix;
    use read_and_make::analysis::{find_num_vertices, calculate_betweenness, sort_hashmap_by_value_descending, closeness_centrality, find_matching_values};

    let mut read = read_file("roadNet-PA.txt");
    read.sort();
    let clone_read = read.clone();
    let num_of_vertices = find_num_vertices(&read);
    let adj_matrix = create_adjacency_matrix(&clone_read);
    let betweenness = calculate_betweenness(&adj_matrix);
    let closeness = closeness_centrality(&adj_matrix);
    let sorted = sort_hashmap_by_value_descending(betweenness);
    let mut combined = find_matching_values(closeness, sorted);
    println!("Number of Vertices: {}", num_of_vertices);
    println!("The vertices with the highest combined betweenness and closeness are: ");
    combined.sort();
    combined.reverse();
    for (value, node) in combined.iter().take(10) {
        println!("Node: {}, Combined: {}", node, value);
    }
}
