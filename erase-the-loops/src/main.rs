// Context: https://codegolf.stackexchange.com/q/274973/94032

use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        panic!("not enough arguments were supplied");
    }

    let input = &args[1];
    let output = loop_eraser(input);
    println!("{}", output);
}

fn loop_eraser(input: &str) -> String {
    let mut input = input.chars().map(|x| Some(x)).collect::<Vec<_>>();

    let mut i = 0;
    let mut j = i + 1;

    'outer: while i < input.len() {
        while j < input.len() {
            if input[j] == input[i] {
                for k in i..j {
                    input[k] = None;
                }

                i = j;
                j = i + 1;
                continue 'outer;
            }

            j += 1;
        }

        i += 1;
        j = i + 1;
    }

    input.iter().flatten().collect::<String>()
}
