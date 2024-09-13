fn main() {
    let mut letters: Vec<String> = ["a", "b", "c"].iter().map(|&s| s.to_string()).collect();
    let mut letters_copy = letters.clone();
    let mut index = 0;
    for letter in &mut letters_copy {
        println!("{}", letter);
        letters.push(letter.clone());
        
        index += 1;
        *letter = index.to_string();
    }

    println!("{:?}", letters_copy);
    println!("{:?}", letters);
}
