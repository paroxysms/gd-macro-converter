use std::{io::{Read, Write}, fs::File};

pub struct URL {
    pub fps: f32,
    pub actions: Vec<(f32, bool, bool)>,
}

impl URL {
    pub fn parse(mut file: File, frame_based: bool) -> URL {
        let mut buffer: [u8; 4] = [0u8; 4];
        let mut single_buffer: [u8; 1] = [0u8; 1];

        file.read(&mut buffer).unwrap();
        let fps = f32::from_le_bytes(buffer);

        file.read(&mut single_buffer).unwrap();
        let replay_type = u8::from_le_bytes(single_buffer);

        let mut actions: Vec<(f32, bool, bool)> = Vec::new();

        loop {
            if file.read(&mut single_buffer).unwrap() != 1 { break; }
            let state = u8::from_le_bytes(single_buffer);
            let click = state & 1 == 1;
            let player2 = state >> 1 == 1;

            let mut frame: f32 = 0.0;

            if file.read(&mut buffer).unwrap() != 4 { break; }
            if replay_type == 2 {
                if frame_based {
                    if file.read(&mut buffer).unwrap() != 4 { break; }
                    frame = u32::from_le_bytes(buffer) as f32;
                } else {
                    frame = f32::from_le_bytes(buffer);
                }
                if file.read(&mut buffer).unwrap() != 4 { break; }
            } else {
                if replay_type == 0 {
                    frame = f32::from_le_bytes(buffer);
                } else {
                    frame = u32::from_le_bytes(buffer) as f32;
                }
            }

            actions.push((frame as f32, click, player2));
        }

        URL {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(f32, bool, bool)>, fps: f32, frame: bool, mut file: File) -> File {
        file.write_all(&(fps).to_le_bytes()).unwrap();
        file.write_all(&(frame as u8).to_le_bytes()).unwrap();
        for i in 0..actions.len() {
            let action = actions[i];
            let state = action.1 as u8 + (action.2 as u8 * 2);
            file.write_all(&(state as u8).to_le_bytes()).unwrap();
            if frame {
                file.write_all(&(action.0 as u32).to_le_bytes()).unwrap();
            } else {
                file.write_all(&(action.0).to_le_bytes()).unwrap();
            }
        }
        file
    }
}