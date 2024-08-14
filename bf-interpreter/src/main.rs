use std::io;
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        panic!("not enough arguments were supplied");
    }

    let program = &args[1];
    let input = &args[2];
    interpreter(program, input);
}

fn interpreter(program: &str, input: &str) {
    let mut mem = [0_u8; 30_000];
    let mut ptr = 0;

    let input = input.chars().collect::<Vec<_>>();

    let mut program_idx = 0;
    let mut input_idx = 0;
    let mut loop_stack: Vec<usize> = vec![];

    while program_idx < program.len() {
        match &program[program_idx..program_idx + 1] {
            ">" => {
                if ptr == 29_999 {
                    ptr = 0;
                } else {
                    ptr += 1;
                }
            }
            "<" => {
                if ptr == 0 {
                    ptr = 29_999;
                } else {
                    ptr -= 1;
                }
            }
            "+" => {
                mem[ptr] = mem[ptr].wrapping_add(1);
            }
            "-" => {
                mem[ptr] = mem[ptr].wrapping_sub(1);
            }
            "." => {
                print!("{}", mem[ptr] as char);
                let _ = io::stdout().flush();
            }
            "," => {
                let input_char = input[input_idx];
                mem[ptr] = input_char as u8;
                input_idx += 1;
            }
            "[" => {
                if mem[ptr] == 0 {
                    let mut depth = 1;

                    while depth > 0 {
                        program_idx += 1;

                        if &program[program_idx..program_idx + 1] == "[" {
                            depth += 1;
                        }

                        if &program[program_idx..program_idx + 1] == "]" {
                            depth -= 1;
                        }
                    }
                } else {
                    loop_stack.push(program_idx);
                }
            }
            "]" => {
                if mem[ptr] != 0 {
                    program_idx = *loop_stack.last().unwrap();
                } else {
                    loop_stack.pop();
                }
            }
            _ => (),
        }

        program_idx += 1;
    }
}
