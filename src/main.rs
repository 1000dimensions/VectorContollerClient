use std::net::TcpStream;
use bevy::{prelude::*, time::common_conditions::on_timer, input::*};
use std::sync::{Mutex, Arc};
use std::borrow::*;




#[derive(Resource)]
struct StickyState {
    edfenable: bool,
    speed:f32,
    client:TcpStream
}

fn main() {
    App::new()
        .insert_resource(StickyState {
            edfenable: false,
            speed: 0.0
            client: TcpStream::connect("192.168.4.1:42").unwrap();
        })
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(DefaultPlugins)
        .add_systems(FixedUpdate, keyboardCap)
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

fn keyboardCap(mut stickystate : ResMut<StickyState>, keyboard_input: Res<ButtonInput<KeyCode>>) {
   // let mut cilient = TcpStream::connect("192.168.4.1:42").unwrap();
    let mut swervelock = false;
    let mut emergancy = false;
    let mut x = 0.0;
    let mut spin = 0.0;
    let mut edfbrr:u16 = 0;
    if stickystate.edfenable == true {
        edfbrr = 3277;
    }
    if keyboard_input.pressed(KeyCode::KeyF){
        stickystate.edfenable = !stickystate.edfenable;
        info!("fast");
        info!("{:?}", stickystate.edfenable);
    }
    if keyboard_input.pressed(KeyCode::Space){
        edfbrr = 65535;
    }
    if keyboard_input.pressed(KeyCode::KeyO){
        emergancy = true;
    }
    if keyboard_input.pressed(KeyCode::KeyV){
        swervelock = true;
    }

    if keyboard_input.pressed(KeyCode::Digit1){
        stickystate.speed = 0.1;
    }
    if keyboard_input.pressed(KeyCode::Digit2){
        stickystate.speed = 0.25;
    }
    if keyboard_input.pressed(KeyCode::Digit3){
        stickystate.speed = 0.5;
    }
    if keyboard_input.pressed(KeyCode::Digit4){
        stickystate.speed = 0.9;
    }
    if keyboard_input.pressed(KeyCode::KeyA){
        spin = -1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD){
        spin = 1.0;
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
    x = x * stickystate.speed;
    y = y * stickystate.speed;
    spin = spin * stickystate.speed;
    let package = robot(x, y, spin, edfbrr, swervelock, emergancy);
    //uncommment when the robot is on and receiving.
    //client.write_all(&package);}
    
    // test functions to see if edf goes brrrrr.
    //println!("is edf going brr? {:?}", edfenable);
    //println!("how fast is edf going brr? {:?}", edfbrr);
} 