use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("serches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

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
