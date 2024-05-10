use std::io;

fn main() {
    let text = get_text();

    for word in text.split_whitespace() {
        println!("{}", pig_latin(word));
    }
}

fn get_text() -> String {
    let mut input = String::new();
    println!("Input words separated by spaces:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input.trim().is_empty() {
        panic!("No text was entered");
    }

    input
}

fn pig_latin(word: &str) -> String {
    match word.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => {
            let mut word_string = word.to_owned();
            word_string.push_str("-hay");
            word_string
        }
        Some(consonant) => {
            let word_vec = word.chars().collect::<Vec<_>>();
            let mut pig_latin = word_vec[1..].iter().copied().collect::<String>();
            pig_latin.push('-');
            pig_latin.push(consonant);
            pig_latin.push_str("ay");
            pig_latin
        }
        None => String::from(""),
    }
}
