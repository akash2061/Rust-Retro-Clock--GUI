use bevy::prelude::*;
use chrono::Timelike;

fn main() {
    clock_face();
}

fn clock_face() {
    let now = chrono::Local::now();

    let hour = now.hour() as f32;
    let min = now.minute() as f32;
    let sec = now.second() as f32;

    println!("\n{hour} : {min} : {sec}");
}
