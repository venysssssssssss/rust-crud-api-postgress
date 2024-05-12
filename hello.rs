fn main() {
    let input = "Hello, World!";
    
    for c in input.chars() {
        let ascii_value = c as u8;
        println!("Character: {}, ASCII Value: {}", c, ascii_value);
    }
}