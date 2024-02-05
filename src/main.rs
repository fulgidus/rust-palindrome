use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Sets the input word to check
    #[arg(short = 'w', long, value_name = "WORD", required = true)]
    word: String,
}

fn main() {
    let args = Args::parse();

    let is_palindrome = args.word.chars().eq(args.word.chars().rev());

    if is_palindrome {
        println!("'{}' is a palindrome.", args.word);
    } else {
        println!("'{}' is not a palindrome.", args.word);
    }
}
