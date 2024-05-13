fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let input_string = "Implement a function that returns the shortest word in the string";
    if let Some(shortest) = shortest_word(input_string) {
        println!("Shortest word in the string: {}", shortest);
    } else {
        println!("No words found in the string");
    }
}
