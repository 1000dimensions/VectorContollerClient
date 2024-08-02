use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    keyboardCap();
}
fn robot(xy: [f32; 2], spin: f32, edfbrr: f32, lock: bool) {
    println!("{:?}", xy);
    println!("{:?}", spin);
    println!("{:?}", edfbrr);
    println!("{:?}", lock);


}
fn keyboardCap() {
    let mut speed: f32 = 0.0;
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut swervelock = false;
    let mut edfenable = false;
   
    write!(
        stdout,
        "{}{}Preston ctrl C is to exit. Make sure not to accidentally do it.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
    
    let mut speed:f32 = 0.0;
    for k in stdin.keys() {
        let mut xy : [f32; 2] = [0.0; 2];
        let mut x = 0.0;
        let mut y = 0.0;
        let mut spin = 0.0;
        let mut edfbrr = 0.0;
        if edfenable == true {
            edfbrr = 0.5;
        }
         
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        
        match k.as_ref().unwrap() {
            Key::Char('a') => x = -1.0,
            Key::Char('d') => x = 1.0,
            Key::Char('w') => y = 1.0,
            Key::Char('s') => y = -1.0,
            Key::Ctrl('c') => break,
            Key::Char('1') => speed = 0.1,
            Key::Char('2') => speed = 0.25,
            Key::Char('3') => speed = 0.5,
            Key::Char('4') => speed = 0.9,
            Key::Char('q') => spin = -1.0,
            Key::Char('e') => spin = 1.0,
            Key::Char('v') => {swervelock = !swervelock;}
            Key::Char('f') => {edfenable = !edfenable;}
            Key::Char(' ') => {
                if edfenable == true{
                    edfbrr = 1.0
                }
            }
            _ => {
                x = 0.0;
                y = 0.0;
            }
        }
        stdout.flush().unwrap();
        x = x * speed;
        y = y * speed;
        xy[0] = x;
        xy[1] = y;
        spin = spin * speed;
        robot(xy, spin, edfbrr, swervelock);
    }
        
    write!(stdout, "{}", termion::cursor::Show).unwrap();

}
