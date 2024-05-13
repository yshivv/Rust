fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let first_string = &strings[0];
    let mut prefix = String::new();

    'outer: for (i, ch) in first_string.chars().enumerate() {
        for s in &strings[1..] {
            if i >= s.len() || s.chars().nth(i) != Some(ch) {
                break 'outer;
            }
        }
        prefix.push(ch);
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ]; 

    let lcp = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", lcp);
}
