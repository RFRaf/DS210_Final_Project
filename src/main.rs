use std::error::Error;
use std::env;
mod data;
mod graph;

fn main() -> Result<(), Box<dyn Error>> {
    let datafile = "All_Beauty_5.json";
    let reviews = data::read_data(datafile)?;
    let graph = graph::construct_graph(&reviews);
    
    let total_products = graph.len();
    let total_nodes = graph.iter().map(|(_, v)| v.len()).sum::<usize>() + total_products; 

    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(start_product) => {
            let products_amt = graph::bfs(&graph, start_product);
            let outreach = products_amt.len();

            let percentage = (outreach as f64 / total_nodes as f64) * 100.0;
            println!("The percentage of nodes reached with six degrees of separation: {:.2}%", percentage);
            
        },
        None => {
            println!("Usage: cargo run starting_product_id");
            
        }

    }
    Ok(())
}