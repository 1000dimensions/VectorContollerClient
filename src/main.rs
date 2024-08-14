use std::net::TcpStream;
use bevy::{prelude::*, time::common_conditions::on_timer, input::*};
use std::sync::{Mutex, Arc};
use std::borrow::*;




#[derive(Resource, Default)]
struct needless {
    edfenable: bool,
    speed:f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboardCap)
        .run();
}
//robot caputures all flags and then assembles them into a nice array. 
fn robot(g: f32, y: f32, spin: f32, edfbrr: u16, lock: bool, shutoff: bool) -> [u8; 16]{
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
    info!{"{:?}", package};
    return package;
}
fn keyboardCap(keyboard_input: Res<ButtonInput<KeyCode>>) {
   // let mut client = TcpStream::connect("192.168.4.1:42").unwrap();
    let mut swervelock = false;
    let mut emergancy = false;
    info!("Preston CTRL + C is to exit the program. Until you do that you will not be let out. ");    
    let mut x = 0.0;
    let mut y = 0.0;
    let mut spin = 0.0;
    let mut edfbrr:u16 = 0;
    let mut cool = needless::default();
    if cool.edfenable == true {
            edfbrr = 3277;
            println!("COOOL BEANS KIDDOS")
    }
    if keyboard_input.pressed(KeyCode::KeyF){
        cool.edfenable = !cool.edfenable;
        info!("fast");
        info!("{:?}", cool.edfenable);
    }

    if keyboard_input.pressed(KeyCode::Digit1){
        cool.speed = 0.1;
    }
    if keyboard_input.pressed(KeyCode::Digit2){
        cool.speed = 0.25;
    }
    if keyboard_input.pressed(KeyCode::Digit3){
        cool.speed = 0.5;
    }
    if keyboard_input.pressed(KeyCode::Digit4){
        cool.speed = 0.9;
    }
    
    
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        y = 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft){
        x = -1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight){
        x = 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown){
        y = -1.0;
    }
    //x = x * speed;
    //y = y * speed;
    //spin = spin * speed;
    let package = robot(x, y, spin, edfbrr, swervelock, emergancy);
    //client.write_all(&package);}
    
    // test functions to see if edf goes brrrrr.
    //println!("is edf going brr? {:?}", edfenable);
    //println!("how fast is edf going brr? {:?}", edfbrr);
} 