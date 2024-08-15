use std::io;
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        panic!("not enough arguments were supplied");
    }

    let program = args[1].clone();
    let input = args[2].chars().collect::<Vec<_>>();
    let mut interpreter = Interpreter::new(program, input);
    let output = interpreter.run();
    for c in output {
        print!("{}", c);
        let _ = io::stdout().flush();
    }
}

struct Interpreter {
    mem: [u8; 30_000],
    ptr: usize,
    loop_stack: Vec<usize>,
    program: String,
    program_idx: usize,
    input: Vec<char>,
    input_idx: usize,
}

impl Interpreter {
    fn new(program: String, input: Vec<char>) -> Self {
        Self {
            mem: [0; 30_000],
            ptr: 0,
            loop_stack: vec![],
            program,
            program_idx: 0,
            input,
            input_idx: 0,
        }
    }

    fn run(&mut self) -> Vec<char> {
        let mut output: Vec<char> = vec![];

        while self.program_idx < self.program.len() {
            match &self.program[self.program_idx..self.program_idx + 1] {
                ">" => {
                    if self.ptr == 29_999 {
                        self.ptr = 0;
                    } else {
                        self.ptr += 1;
                    }
                }
                "<" => {
                    if self.ptr == 0 {
                        self.ptr = 29_999;
                    } else {
                        self.ptr -= 1;
                    }
                }
                "+" => {
                    self.mem[self.ptr] = self.mem[self.ptr].wrapping_add(1);
                }
                "-" => {
                    self.mem[self.ptr] = self.mem[self.ptr].wrapping_sub(1);
                }
                "." => {
                    output.push(self.mem[self.ptr] as char);
                }
                "," => {
                    let input_char = self.input[self.input_idx];
                    self.mem[self.ptr] = input_char as u8;
                    self.input_idx += 1;
                }
                "[" => {
                    if self.mem[self.ptr] == 0 {
                        let mut depth = 1;

                        while depth > 0 {
                            self.program_idx += 1;

                            if &self.program[self.program_idx..self.program_idx + 1] == "[" {
                                depth += 1;
                            }

                            if &self.program[self.program_idx..self.program_idx + 1] == "]" {
                                depth -= 1;
                            }
                        }
                    } else {
                        self.loop_stack.push(self.program_idx);
                    }
                }
                "]" => {
                    if self.mem[self.ptr] != 0 {
                        self.program_idx = *self.loop_stack.last().unwrap();
                    } else {
                        self.loop_stack.pop();
                    }
                }
                _ => (),
            }

            self.program_idx += 1;
        }

        output
    }
}
