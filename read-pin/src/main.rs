// Ungolfed solution to https://codegolf.stackexchange.com/q/275630/94032

use std::io::{stdin, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
fn main() {
    let a = stdin();
    let mut b = stdout().into_raw_mode().unwrap();
    let mut s = "".to_owned();
    for c in a.keys() {
        match c.unwrap() {
            Key::Esc => break,
            Key::Char(k) => {
                if "0123456789".contains(k) {
                    write!(b, "*").unwrap();
                    s.push(k)
                }
            }
            Key::Backspace => {
                write!(
                    b,
                    "{}{}",
                    termion::cursor::Left(1),
                    termion::clear::AfterCursor
                )
                .unwrap();
                s = s[..s.len() - 1].to_string()
            }
            _ => (),
        }
        b.flush().unwrap();
        if s.len() > 3 {
            break;
        }
    }
}
