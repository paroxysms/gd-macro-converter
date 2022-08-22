use std::{io::{Read, Write}, fs::File};

pub struct yBot {
    pub fps: f32,
    pub actions: Vec<(u32, bool, bool)>,
}

impl yBot {

    // function parseYbotF(view) {
    //     const fps = view.getFloat32(4, true);
    //     const nActions = view.getInt32(8, true);
    //     const actions = [];
    //     for (let i = 12; i < 12 + nActions * 8; i += 8) {
    //         const frame = view.getUint32(i, true);
    //         const idk = view.getUint32(i + 4, true);
    //         const hold = (idk & 0b10) === 2;
    //         const player2 = (idk & 0b01) === 1;
    //         actions.push({x: frame, hold, player2});
    //     }
    //     return {fps, actions};
    // }

    pub fn parse(mut file: File) -> yBot {
        let mut buffer: [u8; 4] = [0u8; 4];

        file.read(&mut buffer).unwrap();
        let fps = f32::from_le_bytes(buffer);

        file.read(&mut buffer).unwrap();

        let mut actions: Vec<(u32, bool, bool)> = Vec::new();

        loop {
            if file.read(&mut buffer).unwrap() != 4 { break; }
            let frame = u32::from_le_bytes(buffer);

            if file.read(&mut buffer).unwrap() != 4 { break; }
            let info = u32::from_le_bytes(buffer);

            let hold = (info & 0b10_u32) == 1;
            let player2 = (info & 0b01_u32) == 1;

            actions.push((frame, hold, player2));
        }

        yBot {
            fps,
            actions,
        }
    }

    // function dumpYbotF(replay) {
    //     const buffer = new ArrayBuffer(12 + replay.actions.length * 8);
    //     const view = new DataView(buffer);
    //     strToBuf('ybot').forEach((n, i) => view.setUint8(i, n));
    //     view.setFloat32(4, replay.fps, true);
    //     view.setUint32(8, replay.actions.length, true);
    //     replay.actions.forEach((action, i) => {
    //         view.setInt32(12 + i * 8, action.x, true);
    //         let idk = action.player2 + action.hold * 2;
    //         view.setUint32(16 + i * 8, idk);
    //     });
    //     return buffer;
    // }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(&("ybot".as_bytes())).unwrap();
        file.write_all(&(fps).to_le_bytes()).unwrap();
        file.write_all(&(actions.len() as u32).to_le_bytes()).unwrap();
        for i in 0..actions.len() {
            let action = actions[i];
            file.write_all(&(action.0).to_le_bytes()).unwrap();
            let info = action.2 as u8 + (action.1 as u8 * 2);
            file.write_all(&(info as u32).to_le_bytes()).unwrap();
        }
        file
    }
}