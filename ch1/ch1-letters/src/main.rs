fn main() {
    let mut letters: Vec<String> = ["a", "b", "c"].iter().map(|&s| s.to_string()).collect();
    let mut letters_copy = letters.clone();
    for (index, letter) in letters_copy.iter_mut().enumerate() {
        println!("{}", letter);
        letters.push(letter.clone());

        let mut hoge: String = letter.clone();
        hoge = "uhi".to_string();
        
        *letter = format!("{} {}", hoge, index);
        *letter = index.to_string();
    }

    println!("{:?}", letters_copy);
    println!("{:?}", letters);
}
