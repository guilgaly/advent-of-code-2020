mod document;

use crate::document::Document;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let parsed_docs: Vec<Document> = INPUT
        .split("\n\n")
        .filter_map(|raw_document| Document::parse(raw_document).ok())
        .collect();
    println!("Part 1 result: {}", parsed_docs.len());

    let validated_docs: Vec<Document> = parsed_docs
        .iter()
        .filter_map(|doc| Document::validate(doc).ok())
        .collect();
    println!("Part 2 result: {}", validated_docs.len());

    Ok(())
}
