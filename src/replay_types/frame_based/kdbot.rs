use std::{io::{Read, Write}, fs::File};

pub struct KDBot {
    pub fps: f32,
    pub actions: Vec<(i32, bool, bool)>,
}

impl KDBot {

    // function parseKDBot(view) {
    //     const fps = view.getFloat32(0, true);
    //     const actions = [];
    //     for (let i = 4; i < view.byteLength; i += 6) {
    //         const frame = view.getInt32(i, true);
    //         const hold = view.getUint8(i + 4) === 1;
    //         const player2 = view.getUint8(i + 5) === 1;
    //         actions.push({x: frame, hold, player2});
    //     }
    //     return {fps, actions};
    // }

    pub fn parse(mut file: File) -> KDBot {
        let mut buffer: [u8; 4] = [0u8; 4];
        let mut double_buffer: [u8; 2] = [0u8; 2];
        let mut single_buffer: [u8; 1] = [0u8; 1];

        file.read(&mut buffer).unwrap();
        let fps = f32::from_le_bytes(buffer);

        let mut actions: Vec<(i32, bool, bool)> = Vec::new();

        loop {
            if file.read(&mut buffer).unwrap() != 4 { break; }
            let frame = i32::from_le_bytes(buffer);

            if file.read(&mut single_buffer).unwrap() != 1 { break; }
            let hold = u8::from_le_bytes(single_buffer) == 1;

            if file.read(&mut single_buffer).unwrap() != 1 { break; }
            let player2 = u8::from_le_bytes(single_buffer) == 1;

            actions.push((frame, hold, player2));
        }

        KDBot {
            fps,
            actions,
        }
    }

    // function dumpKDBot(replay) {
    //     const buffer = new ArrayBuffer(4 + replay.actions.length * 6);
    //     const view = new DataView(buffer);
    //     view.setFloat32(0, replay.fps, true);
    //     replay.actions.forEach((action, i) => {
    //         view.setUint32(4 + i * 6, action.x, true);
    //         view.setUint8(8 + i * 6, action.hold);
    //         view.setUint8(9 + i * 6, action.player2);
    //     });
    //     return buffer;
    // }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(&(fps).to_le_bytes()).unwrap();
        for i in 0..actions.len() {
            let action = actions[i];
            file.write_all(&(action.0 as u32).to_le_bytes()).unwrap();
            file.write_all(&(action.1 as u8).to_le_bytes()).unwrap();
            file.write_all(&(action.2 as u8).to_le_bytes()).unwrap();
        }
        file
    }
}