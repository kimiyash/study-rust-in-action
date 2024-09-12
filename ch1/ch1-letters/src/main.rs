fn main() {
    let mut letters = vec!["a", "b", "c"];
    let letters_copy = letters.clone();

    for letter in &letters_copy {
        println!("{}", letter);
        letters.push(*letter);
    }

    println!("{:?}", letters_copy);    
    println!("{:?}", letters);
}
