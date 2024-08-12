use std::io;
use std::io::Write;

fn main() {
    let text = get_text();

    for word in text.split_whitespace() {
        println!("{}", pig_latin(word));
    }
}

fn get_text() -> String {
    let mut input = String::from("");

    while input.trim().is_empty() {
        print!("Input words separated by spaces: ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    }

    input
}

fn pig_latin(word: &str) -> String {
    match &word[0..1] {
        "" => String::from(""),
        "a" | "e" | "i" | "o" | "u" => {
            let mut result = word.to_owned();
            result.push_str("-hay");
            result
        }
        x => {
            let mut result = word[1..].to_string();
            result.push('-');
            result.push_str(x);
            result.push_str("ay");
            result
        }
    }
}
