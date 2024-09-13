use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
it is the same with books.What do we seek through millions of pages?";

    for line in quote.lines() {
        let contrains_substring = re.find(line);
        if contrains_substring.is_some() {
            println!("{}", line)
        }
    }
}
