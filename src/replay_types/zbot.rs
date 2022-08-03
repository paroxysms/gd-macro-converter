use std::{io::{Read, Write}, fs::File};

pub struct zBot {
    pub delta: f32,
    pub speedhack: f32,
    pub fps: f32,
    pub actions: Vec<(i32, bool, bool)>,
}

impl zBot {
    pub fn parse(mut file: File) -> zBot {
        let mut buffer: [u8; 4] = [0u8; 4];
        let mut double_buffer: [u8; 2] = [0u8; 2];

        file.read(&mut buffer).unwrap();
        let delta = f32::from_le_bytes(buffer);

        file.read(&mut buffer).unwrap();
        let speedhack = f32::from_le_bytes(buffer);

        let fps: f32 = 1.0/delta/speedhack;

        let mut actions: Vec<(i32, bool, bool)> = Vec::new();

        loop {
            if file.read(&mut buffer).unwrap() != 4 { break; }
            let frame = i32::from_le_bytes(buffer);

            if file.read(&mut double_buffer).unwrap() != 2 { break; }
            let click = double_buffer[0] == 0x31;
            let player1 = double_buffer[1] == 0x31;

            actions.push((frame, click, !player1));
        }

        zBot {
            delta,
            speedhack,
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(&(1.0/fps).to_le_bytes()).unwrap();
        file.write_all(&(1.0_f32).to_le_bytes()).unwrap();
        for i in 0..actions.len() {
            file.write_all(&(actions[i].0).to_le_bytes()).unwrap();
            if actions[i].1 {
                file.write_all(&(0x31_u8).to_le_bytes()).unwrap();
            } else {
                file.write_all(&(0x30_u8).to_le_bytes()).unwrap();
            }
            if !actions[i].2 {
                file.write_all(&(0x31_u8).to_le_bytes()).unwrap();
            } else {
                file.write_all(&(0x30_u8).to_le_bytes()).unwrap();
            }
        }
        file
    }
}