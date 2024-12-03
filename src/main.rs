extern crate termion;
extern crate shamir;

use clap::Parser;
use shamir::combine_shares;
use std::io::{ self, Write };
use std::process::Command;
use termion::event::Key;
use termion::input::TermRead;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Sets the input word to check
    #[arg(short = 'w', long, value_name = "WORD")]
    word: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(word) = args.word {
        check_palindrome(&[word]);
    } else {
        let stdin = io::stdin();
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc
            ::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            })
            .unwrap_or_else(|e| {
                eprintln!("Error setting Ctrl-C handler: {}", e);
                std::process::exit(1);

        while running.load(Ordering::SeqCst) {
            print!("Press q to exit, d to enter decode mode, or g to enter generation mode: ");
            if io::stdout().flush().is_err() {
                eprintln!("Failed to flush stdout");
                continue;
            }

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read line");
                continue;
            }
            let input = input.trim();

            match input {
                "q" => {
                    break;
                }
                "d" => {
                    clear_terminal();
                    decode_mode();
                }
                "g" => {
                    clear_terminal();
                    generation_mode();
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

fn decode_mode() {
    let stdin = io::stdin();
    let running = Arc::new(AtomicBool::new(true));
    let _r = running.clone();

    print!("Decode mode: Enter the number of words (or press q to exit decode mode): ");
    if io::stdout().flush().is_err() {
        eprintln!("Failed to flush stdout");
        return;
    }

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read line");
        return;
    }
    let input = input.trim();

    if input == "q" {
        clear_terminal();
        return;
    }

    let num_words: usize = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number");
            return;
        }
    };

    let mut words = Vec::new();
    for _ in 0..num_words {
        print!("Enter a word: ");
        if io::stdout().flush().is_err() {
            eprintln!("Failed to flush stdout");
            continue;
        }

        let mut word = String::new();
        if io::stdin().read_line(&mut word).is_err() {
            eprintln!("Failed to read line");
            continue;
        }
        words.push(word.trim().to_string());
    }

    clear_terminal();
    match decode_shamir(&words) {
        Ok(secret) => println!("Decoded secret: {}", secret),
        Err(e) => eprintln!("Failed to decode secret: {}", e),
    }
}

fn generation_mode() {
    let stdin = io::stdin();
    let running = Arc::new(AtomicBool::new(true));
    let _r = running.clone();

    while running.load(Ordering::SeqCst) {
        print!("Generation mode: Enter a word (or press q to exit generation mode): ");
        if io::stdout().flush().is_err() {
            eprintln!("Failed to flush stdout");
            continue;
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }
        let input = input.trim();

        if input == "q" {
            clear_terminal();
            break;
        }

        clear_terminal();
        check_palindrome(&[input.to_string()]);
    }
}

fn decode_shamir(words: &[String]) -> Result<String, String> {
    let shares: Vec<(u8, Vec<u8>)> = words.iter()
        .enumerate()
        .map(|(i, word)| (i as u8 + 1, word.as_bytes().to_vec()))
        .collect();

    match combine_shares(&shares) {
        Ok(secret) => Ok(String::from_utf8(secret).map_err(|e| e.to_string())?),
        Err(e) => Err(e.to_string()),
    }
}

fn check_palindrome(words: &[String]) {
    let concatenated: String = words.concat();
    let is_palindrome = concatenated.chars().eq(concatenated.chars().rev());

    if is_palindrome {
        println!("The concatenated words '{:?}' form a palindrome.", words);
    } else {

        println!("The concatenated words '{:?}' do not form a palindrome.", words);
    }
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        if let Err(e) = Command::new("cmd").arg("/C").arg("cls").status() {
            eprintln!("Failed to clear terminal: {}", e);
        }
    } else {
        if let Err(e) = Command::new("clear").status() {
            eprintln!("Failed to clear terminal: {}", e);
        }
    }
}
