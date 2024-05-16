fn main() {
    println!("{}", is_palindrome(111));
}

pub fn is_palindrome(x: i32) -> bool {
    let chars: Vec<char> = x.to_string().chars().collect();
    let length = chars.len();
    for (index, &value) in chars.iter().enumerate() {
        if value != chars[length - 1 - index] {
            return false
        }
    }
    true
}