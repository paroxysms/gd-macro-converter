use std::{io::{Read, Write}, fs::File};
use crate::replay_types::zbot::zBot;

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
        println!("{}", fps);

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

            println!("{} {} {}", frame, state & 1, state >> 1);

            actions.push((frame as f32, click, player2));
        }

        URL {
            fps,
            actions,
        }
    }

    // function dumpUniversalReplayFormat(replay, frame) {
    //     const buffer = new ArrayBuffer(5 + replay.actions.length * 5);
    //     const view = new DataView(buffer);
    //     view.setFloat32(0, replay.fps, true);
    //     view.setUint8(4, frame, true);
    //     replay.actions.forEach((action, i) => {
    //         const state = action.hold + action.player2 * 2;
    //         view.setUint8(5 + i * 5, state);
    //         if (frame)
    //             view.setUint32(6 + i * 5, action.x, true);
    //         else
    //             view.setFloat32(6 + i * 5, action.x, true);
    //     });
    //     return buffer;
    // }

    pub fn dump(actions: Vec<(f32, bool, bool)>, fps: f32, frame: bool, mut file: File) -> File {
        file.write_all(&(fps).to_le_bytes()).unwrap();
        file.write_all(&(frame as i32).to_le_bytes()).unwrap();
        for i in 0..actions.len() {
            let action = actions[i];
            let state = action.1 as i32 + (action.2 as i32 * 2);
            file.write_all(&(state as i32).to_le_bytes()).unwrap();
            if frame {
                file.write_all(&(action.0 as u32).to_le_bytes()).unwrap();
            } else {
                file.write_all(&(action.0).to_le_bytes()).unwrap();
            }
        }
        file
    }
}