use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub overall: f64,
    pub verified: bool,
    #[serde(rename = "reviewTime")]
    pub review_time: String,
    #[serde(rename = "reviewerID")]
    pub reviewer_id: String,
    pub asin: String,
    pub style: Option<Style>,
    #[serde(rename = "reviewerName")]
    pub reviewer_name: String,
    #[serde(rename = "reviewText")]
    pub review_text: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "unixReviewTime")]
    pub unix_review_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Style {
    #[serde(rename = "Size")]
    pub size: Option<String>,  // Making Size optional
    #[serde(rename = "Flavor")]
    pub flavor: Option<String>, // Assuming Flavor can also be optional
}

pub fn read_data(datafile: &str) -> Result<Vec<Review>, Box<dyn Error>> {
    let file = File::open(datafile)?;
    let reader = BufReader::new(file);
    let mut reviews = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let review: Review = serde_json::from_str(&line)?;
        reviews.push(review);
    }

    Ok(reviews)
}

#[cfg(test)]
mod data_tests {
    use super::*;

    #[test]
    fn test_read_data() {
        let mock_data = "All_Beauty_5.json";
        let result = read_data(mock_data);
        assert!(result.is_ok());
        let reviews = result.unwrap();
        assert!(!reviews.is_empty());
    }
}