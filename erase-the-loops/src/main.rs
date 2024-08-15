// Solution to https://codegolf.stackexchange.com/q/274973/94032

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
        dbg!(i);

        while j < input.len() {
            dbg!(j);

            if input[j] == input[i] {
                dbg!(input[i]);

                for k in i..j {
                    input[k] = None;
                    dbg!(k, &input);
                }

                i = j;
                j = i + 1;

                dbg!(i, j);

                continue 'outer;
            }

            j += 1;
            dbg!(i);
        }

        i += 1;
        j = i + 1;
        dbg!(j);
    }

    input
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<String>()
}
