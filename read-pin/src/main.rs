// Ungolfed solution to https://codegolf.stackexchange.com/q/275630/94032

use std::io::{stdin, stdout, Write};
use termion::{
    clear,
    cursor::Left,
    event::Key::{Backspace, Char},
    input::TermRead,
    raw::IntoRawMode,
};
fn main() {
    let (mut o, mut l) = (stdout().into_raw_mode().unwrap(), 0);
    for k in stdin().keys() {
        match k.unwrap() {
            Char(c) => {
                if c.is_digit(10) {
                    write!(o, "*").unwrap();
                    l += 1
                }
            }
            Backspace => {
                write!(o, "{}{}", Left(1), clear::AfterCursor).unwrap();
                l -= 1
            }
            _ => (),
        }
        o.flush().unwrap();
        if l > 3 {
            break;
        }
    }
}
