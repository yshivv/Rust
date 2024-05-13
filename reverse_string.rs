fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

fn main() {
    let input_string = "Hello, World!"; // Example input string
    let reversed_string = reverse_string(input_string);
    println!("Reversed string: {}", reversed_string);
}
