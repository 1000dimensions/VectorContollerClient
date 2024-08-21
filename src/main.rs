use std::net::TcpStream;
use bevy::{prelude::*, time::common_conditions::on_timer, input::*};
use std::sync::{Mutex, Arc};
use std::borrow::*;
use crate::gamepad::*;


#[derive(Resource)]
struct MyGamepad(Gamepad);


#[derive(Resource)]
struct StickyState {
    edfenable: bool,
    speed:f32,
    //client:TcpStream
}

fn main() {
    App::new()
        .insert_resource(StickyState {
            edfenable: false,
            speed: 0.0,
            //client: TcpStream::connect("192.168.4.1:42").unwrap()
        })
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(DefaultPlugins)
        .add_systems(FixedUpdate, (gamepad_connections, keyboardCap))
        .run();
}
//robot caputures all flags and then assembles them into a nice array. 
fn robot(g: f32, y: f32, spin: f32, edfbrr: u16, lock: bool, shutoff: bool) -> [u8; 16]{
    let mut bytes = g.to_le_bytes();
    let mut package: [u8; 16] = [0; 16];
    let mut position:usize = 0;
    for x in bytes{
        package[position] = x;
        position += 1;
    }
    position = 4; 
    bytes = y.to_le_bytes();
    for x in bytes{
        package[position] = x;
        position += 1;
    }
    position = 8;
    bytes = spin.to_le_bytes();
    for x in bytes{
        package[position] = x;
        position += 1;
    }
    position = 12;
    let byte = edfbrr.to_le_bytes();
    for x in byte{
        package[position] = x;
        position += 1;
    }
    let mut bit = 0;
    bit |= (lock as u8) << 0;
    bit |= (shutoff as u8) << 1;
    position = 14;
    package[position] = bit;
    return package;
}


fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    ev_conn.gamepad, info.name,
                );
                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
                // if it's the one we previously used for the player, remove it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == ev_conn.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}

fn powah(i : f32, p : f32) -> f32 {
    i.abs().powf(p) * (if i < 0.0 {-1.0} else {1.0})
}

fn keyboardCap(mut stickystate : ResMut<StickyState>, keyboard_input: Res<ButtonInput<KeyCode>>, axes: Res<Axis<GamepadAxis>>, buttons: Res<ButtonInput<GamepadButton>>, my_gamepad: Option<Res<MyGamepad>>) {
   // let mut cilient = TcpStream::connect("192.168.4.1:42").unwrap();
    let mut swervelock = false;
    let mut emergancy = false;
    let mut x = 0.0;
    let mut y = 0.0;
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

    if let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() {
        let axis_swerve_left = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::RightStickX
        };
        let axis_swerve_forwards = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::RightStickY
        };
        let axis_turn = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX
        };
        let button_edfenable = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::LeftTrigger
        };
        let button_edfbrrrrr = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::RightTrigger
        };
        let button_swervelock = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::East
        };
        let button_select = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Select
        };
        let button_start = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start
        };
        if let (Some(s_x), Some(s_y), Some(s_a)) = (axes.get(axis_swerve_left), axes.get(axis_swerve_forwards), axes.get(axis_turn)) {
            if s_x.abs() > 0.1 {
                x = powah(s_x, 2.5);
            }
            if s_y.abs() > 0.1 {
                y = powah(s_y, 2.5);
            }
            if s_a.abs() > 0.1 {
                spin = powah(s_a, 2.5);
            }
        }
        if buttons.just_pressed(button_edfenable) {
            stickystate.edfenable = !stickystate.edfenable;
        }
        if buttons.pressed(button_edfbrrrrr) && stickystate.edfenable {
            edfbrr = 65535;
        }
        if buttons.pressed(button_swervelock) {
            swervelock = true;
        }
        if buttons.pressed(button_select) && buttons.pressed(button_start) {
            emergancy = true;
        }
    }

    //println!("x = {}, y = {}, spin = {}, edf is at {}, the swerve drive is {}, and we {} in an emergency.", x, y, spin, edfbrr, if swervelock { "locked" } else { "unlocked" }, if emergancy { "ARE" } else { "are NOT" });

    let package = robot(x, y, spin, edfbrr, swervelock, emergancy);
    //uncommment when the robot is on and receiving.
    //client.write_all(&package);}
    
    // test functions to see if edf goes brrrrr.
    //println!("is edf going brr? {:?}", edfenable);
    //println!("how fast is edf going brr? {:?}", edfbrr);
} 