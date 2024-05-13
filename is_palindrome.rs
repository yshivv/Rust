//is_palindrome function
fn is_palindrome(s: &str) -> bool {
    let s = s.trim().to_lowercase();
    s.chars().eq(s.chars().rev())
}

//main function
fn main() {
    let input_string = "121"; 
    if is_palindrome(input_string) {
        println!("{} is a palindrome!", input_string);
    } else {
        println!("{} is not a palindrome.", input_string);
    }
}
