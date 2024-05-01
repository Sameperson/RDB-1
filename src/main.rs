mod node;
mod edge;
mod query_parser;
mod graph;

use graph::Graph;
use crate::query_parser::{execute_query, Query};

fn main() {
    let mut graph = Graph::new();
    let input = "PLEASE DELETE 123";  // Example input

    match input.parse::<Query>() {
        Ok(query) => {
            match execute_query(&mut graph, query) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Err(e) => eprintln!("Error parsing query: {}", e),
    }
}
