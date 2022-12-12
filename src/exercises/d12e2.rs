use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/resources/d12") {
        let (graph, starts, end) = construct_graph(lines);
        let mut steps = 9999;
        for start in starts {
            let result = *dijkstra(&graph, start, Some(end), |_| 1)
                .get(&end)
                .unwrap_or(&9999);
            if result <= steps {
                steps = *dijkstra(&graph, start, Some(end), |_| 1).get(&end).unwrap();
            }
        }
        println!("{}", steps);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// return graph, starting nodes, end node
fn construct_graph(
    lines: Lines<io::BufReader<File>>,
) -> (Graph<char, char>, Vec<NodeIndex>, NodeIndex) {
    let collected_lines = lines.filter_map(|result| match result {
        Ok(line) => Some(line),
        Err(_) => None,
    });
    let mut node_array: [[Option<NodeIndex<u32>>; 159]; 41] = [[None; 159]; 41];
    let mut starts: Vec<NodeIndex> = Vec::new();
    let mut end: Option<NodeIndex> = None;
    let mut graph: Graph<char, char> = Graph::new();
    for (i, line) in (0_i32..).zip(collected_lines) {
        for (j, step) in (0_i32..).zip(line.chars()) {
            // add node with char size in graph
            let node = graph.add_node(step);
            //put node in 2D array

            node_array[i as usize][j as usize] = Some(node);

            // to convert 'E' and 'S' to size charachter
            let mut node_char = step;
            if node_char == 'E' {
                node_char = 'z';
                end = Some(node);
            }
            if node_char == 'S' {
                node_char = 'a';
            }
            // get all starting points
            if node_char == 'a' {
                starts.push(node);
            }
            // check upside node in 2D array to see if an edge can be made in one direction on another
            if i > 0 {
                let up_node = (node_array[(i - 1) as usize][j as usize]).unwrap();
                let mut node_up_char = graph[up_node];
                if node_up_char == 'E' {
                    node_up_char = 'z';
                }
                if node_up_char == 'S' {
                    node_up_char = 'a';
                }
                if node_up_char as u16 <= (node_char as u16) + 1 {
                    graph.extend_with_edges([(node, up_node)]);
                }
                if (node_up_char as u16) + 1 >= (node_char as u16) {
                    graph.extend_with_edges([(up_node, node)]);
                }
            }
            // check left node in 2D array to see if an edge can be made in one direction on another
            if j > 0 {
                let up_node = (node_array[i as usize][(j - 1) as usize]).unwrap();
                let mut node_up_char = graph[up_node];
                if node_up_char == 'E' {
                    node_up_char = 'z';
                }
                if node_up_char == 'S' {
                    node_up_char = 'a';
                }
                if node_up_char as u16 <= (node_char as u16) + 1 {
                    graph.extend_with_edges([(node, up_node)]);
                }
                if (node_up_char as u16) + 1 >= node_char as u16 {
                    graph.extend_with_edges([(up_node, node)]);
                }
            }
        }
    }

    return (graph, starts, end.unwrap());
}
