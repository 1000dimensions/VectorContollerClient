use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    keyboardCap();
}
fn robot(xy: i32, pos: String, speed: f32 ) {

}
fn keyboardCap() {
    let mut speed: f32 = 0.0;
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
   
    write!(
        stdout,
        "{}{}Preston ctrl C is to exit. Make sure to turn off robot before that.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
    
    let mut speed:f32 = 0.0;
    for k in stdin.keys() {
        let mut xy = Vec::new();
        let mut x = 0;
    let mut y = 0;
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();
        
        match k.as_ref().unwrap() {
            Key::Char('a') => x += -1,
            Key::Char('d') => x += 1,
            Key::Char('w') => y += 1,
            Key::Char('s') => y += -1,
            Key::Ctrl('c') => break,
            _ => {
                println!("{:?}", k)
            }
        }
        stdout.flush().unwrap();
        
        xy.push(x);
        xy.push(y);
        println!("{:?}", xy);
    }
        
    write!(stdout, "{}", termion::cursor::Show).unwrap();

}
