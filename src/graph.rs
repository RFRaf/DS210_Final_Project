use std::collections::{HashMap, HashSet, VecDeque};
use crate::data::Review; 

pub fn construct_graph(reviews: &[Review]) -> HashMap<String, HashMap<String, usize>> {
    let mut graph: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for review in reviews {
        let connections = graph.entry(review.asin.clone()).or_insert_with(HashMap::new);
        connections.entry(review.reviewer_id.clone()).and_modify(|e| *e += 1).or_insert(1);
    }
    graph
}



pub fn bfs(graph: &HashMap<String, HashMap<String, usize>>, start: &str) -> HashSet<String> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.to_string(), 0));

    while let Some((node, depth)) = queue.pop_front() {
        if depth >= 6 {
            break;
        }
        if !visited.insert(node.clone()) {
            continue;
        }
        if let Some(neighbors) = graph.get(&node) {
            for (neighbor, _) in neighbors {
                if !visited.contains(neighbor) {
                    queue.push_back((neighbor.clone(), depth + 1));
                }
            }
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_review() -> Vec<Review> {
        vec!{
            Review {
                overall: 5.0,
                verified: true, 
                review_time: "09 1, 2016".to_string(),
                reviewer_id: "A3CIUOJXQ5VDQ2".to_string(),
                asin: "B0000530HU".to_string(),
                style: None, 
                reviewer_name: "Shelly F".to_string(),
                review_text: Some("As advertised. Reasonably priced".to_string()), 
                summary: None,
                unix_review_time: 1472688000,
            },
            Review {
                overall: 5.0,
                verified: true,
                review_time: "11 14, 2013".to_string(),
                reviewer_id: "A3H7T87S984REU".to_string(),
                asin: "B0000530HU".to_string(),
                style: None,
                reviewer_name: "houserules18".to_string(),
                review_text: Some("Like the oder and the feel when I put it on my face.  I have tried other brands but the reviews from people I know they prefer the oder of this brand. 
                Not hard on the face when dry.  Does not leave dry skin.".to_string()),
                summary: None,
                unix_review_time: 1384387200,

            },
        }
    }
    
    #[test]
    fn test_construct_graph() {
        let reviews = mock_review();
        let graph = construct_graph(&reviews);
        assert_eq!(graph.len(),1); //Assumes the reviews all have the same asin
        assert!(graph.contains_key("B0000530HU"));
        assert_eq!(graph["B0000530HU"].len(), 2); //checks if two reviewers reviewed the same product
    }

    #[test]
    fn test_bfs() {
        let reviews = mock_review();
        let graph = construct_graph(&reviews);
        let reached_products = bfs(&graph, "B0000530HU");
        assert!(reached_products.contains("A3CIUOJXQ5VDQ2"));
        assert!(reached_products.contains("A3H7T87S984REU"));
        assert_eq!(reached_products.len(), 2);
    }
}