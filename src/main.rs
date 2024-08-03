use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    keyboardCap();
}
//robot caputures all flags and then assembles them into a nice array. 
fn robot(g: f32, y: f32, spin: f32, edfbrr: u16, lock: bool, shutoff: bool) {
    let mut bytes = g.to_le_bytes();
    let mut package: [u8; 16] = [0; 16];
    let mut position:usize = 0;
    for x in bytes{
        package[position] = x;
        position += 1
    }
    position = 4; 
    bytes = y.to_le_bytes();
    for x in bytes{
        package[position] = x;
        position += 1
    }
    position = 8;
    bytes = spin.to_le_bytes();
    for x in bytes{
        package[position] = x;
        position += 1
    }
    position = 12;
    let byte = edfbrr.to_le_bytes();
    for x in byte{
        package[position] = x;
        position += 1
    }
    let mut bit = 0;
    bit |= (lock as u8) << 0;
    bit |= (shutoff as u8) << 1;
    position = 14;
    package[position] = bit;
    println!{"{:?}", package};

}
fn keyboardCap() {
    let mut speed: f32 = 0.0;
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut swervelock = false;
    let mut edfenable = false;
    let mut emergancy = false;
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
        let mut x = 0.0;
        let mut y = 0.0;
        let mut spin = 0.0;
        let mut edfbrr:u16 = 0;
        if edfenable == true {
            edfbrr = 3277;
        }
         

        
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
            Key::Char('o') => {emergancy = true;},
            Key::Char(' ') => {
                if edfenable == true{
                    edfbrr = 65535;
                }
            }
            _ => {
                x = 0.0;
                y = 0.0;
            }
        }
        x = x * speed;
        y = y * speed;
        spin = spin * speed;
        robot(x, y, spin, edfbrr, swervelock, emergancy);
        // test functions to see if edf goes brrrrr.
        //println!("is edf going brr? {:?}", edfenable);
        //println!("how fast is edf going brr? {:?}", edfbrr);
    }
}
