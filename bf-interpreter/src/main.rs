const ERROR: &str = "usage: <program_name> <code> <input_string>";
const MEM_SIZE: usize = 30_000;

fn main() {
    let mut args = std::env::args();

    args.next();

    let program = args.next().expect(ERROR);
    let input = args.next().expect(ERROR);

    let mut interpreter = Interpreter::new(&program, &input);
    interpreter.run();
}

pub struct Interpreter<'a> {
    mem: [u8; MEM_SIZE],
    ptr: usize,
    loop_stack: Vec<usize>,
    program: &'a str,
    program_idx: usize,
    input: &'a str,
    input_idx: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a str, input: &'a str) -> Self {
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

    pub fn run(&mut self) {
        while self.program_idx < self.program.len() {
            match &self.program[self.program_idx..self.program_idx + 1] {
                ">" => {
                    self.move_ptr_right();
                }
                "<" => {
                    self.move_ptr_left();
                }
                "+" => {
                    self.incr();
                }
                "-" => {
                    self.decr();
                }
                "." => {
                    self.output();
                }
                "," => {
                    self.input();
                }
                "[" => {
                    self.loop_start();
                }
                "]" => {
                    self.loop_end();
                }
                _ => (),
            }

            self.program_idx += 1;
        }
    }

    fn move_ptr_right(&mut self) {
        self.ptr = (self.ptr + 1) % MEM_SIZE;
    }

    fn move_ptr_left(&mut self) {
        self.ptr = (self.ptr - 1 + MEM_SIZE) % MEM_SIZE;
    }

    fn incr(&mut self) {
        self.mem[self.ptr] = self.mem[self.ptr].wrapping_add(1);
    }

    fn decr(&mut self) {
        self.mem[self.ptr] = self.mem[self.ptr].wrapping_sub(1);
    }

    fn output(&mut self) {
        print!("{}", self.mem[self.ptr] as char);
    }

    fn input(&mut self) {
        let input_char = self.input.as_bytes()[self.input_idx];
        self.mem[self.ptr] = input_char as u8;
        self.input_idx += 1;
    }

    fn loop_start(&mut self) {
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

    fn loop_end(&mut self) {
        if self.mem[self.ptr] != 0 {
            self.program_idx = *self.loop_stack.last().unwrap();
        } else {
            self.loop_stack.pop();
        }
    }
}
