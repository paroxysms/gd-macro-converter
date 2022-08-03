use std::{io::{Read, Write}, fs::File};

pub struct Rush {
    pub fps: i16,
    pub actions: Vec<(i32, bool, bool)>,
}

impl Rush {
    pub fn parse(mut file: File) -> Rush {
        let mut buffer: [u8; 4] = [0u8; 4];
        let mut double_buffer: [u8; 2] = [0u8; 2];
        let mut single_buffer: [u8; 1] = [0u8; 1];

        file.read(&mut double_buffer).unwrap();
        let fps = i16::from_le_bytes(double_buffer);

        let mut actions: Vec<(i32, bool, bool)> = Vec::new();

        loop {
            if file.read(&mut buffer).unwrap() != 4 { break; }
            let frame = i32::from_le_bytes(buffer);

            if file.read(&mut single_buffer).unwrap() != 1 { break; }
            let state = u8::from_le_bytes(single_buffer);

            let hold = (state & 1) == 1;
            let player2 = (state >> 1) == 1;

            actions.push((frame, hold, player2));
        }

        Rush {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: i16, mut file: File) -> File {
        file.write_all(&(fps).to_le_bytes()).unwrap();
        for i in 0..actions.len() {
            let action = actions[i];
            let state = action.1 as u8 + (action.2 as u8 * 2);
            file.write_all(&(action.0).to_le_bytes()).unwrap();
            file.write_all(&(state).to_le_bytes()).unwrap();
        }
        file
    }
}