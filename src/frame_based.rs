use std::{io::{Read, Write}, fs::File, fs};
use serde_json::json;

// plain text
pub struct PlainText {
    pub fps: f32,
    pub actions: Vec<(f32, bool, bool)>,
}

impl PlainText {
    pub fn parse(mut file: File) -> Self {
        let mut buffer = &mut String::new();
        file.read_to_string(buffer).unwrap();

        println!("buffer: {}", buffer);
        let lines: Vec<&str> = buffer.split("\n").collect();

        println!("line: {}", lines.get(0).unwrap());
        let fps = lines.get(0).unwrap().parse::<f32>().unwrap();
        let mut actions: Vec<(f32, bool, bool)> = Vec::new();

        for line in lines {
            let data: Vec<&str> = line.split(" ").collect();

            if data.len() == 3 {
                let frame = data.get(0).unwrap().parse::<f32>().unwrap();
                let click = data.get(1).unwrap() == &"1";
                let player2 = data.get(2).unwrap() == &"1";

                actions.push((frame, click, player2));
            }
        }

        Self {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(f32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(format!("{}\n", fps).as_ref()).unwrap();
        for action in actions {
            file.write_all(format!("{} {} {}\n", action.0, action.1 as u8, action.2 as u8).as_ref()).unwrap();
        }
        file
    }
}

// zbot
pub struct zBotFrame {
    pub delta: f32,
    pub speedhack: f32,
    pub fps: f32,
    pub actions: Vec<(i32, bool, bool)>,
}

impl zBotFrame {
    pub fn parse(mut file: File) -> Self {
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

        Self {
            delta,
            speedhack,
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(&(1.0/fps).to_le_bytes()).unwrap();
        file.write_all(&(1.0_f32).to_le_bytes()).unwrap();
        for action in 0..actions.len() {
            file.write_all(&(actions[action].0).to_le_bytes()).unwrap();
            if actions[action].1 {
                file.write_all(&(0x31_u8).to_le_bytes()).unwrap();
            } else {
                file.write_all(&(0x30_u8).to_le_bytes()).unwrap();
            }
            if !actions[action].2 {
                file.write_all(&(0x31_u8).to_le_bytes()).unwrap();
            } else {
                file.write_all(&(0x30_u8).to_le_bytes()).unwrap();
            }
        }
        file
    }
}

// xbot
pub struct xBotFrame {

}

impl xBotFrame {

}

// replaybot


// tasbot


// ybot
pub struct yBot {
    pub fps: f32,
    pub actions: Vec<(u32, bool, bool)>,
}

impl yBot {
    pub fn parse(mut file: File) -> Self {
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

        Self {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(&("ybot".as_bytes())).unwrap();
        file.write_all(&(fps).to_le_bytes()).unwrap();
        file.write_all(&(actions.len() as u32).to_le_bytes()).unwrap();
        for action in 0..actions.len() {
            let action = actions[action];
            file.write_all(&(action.0).to_le_bytes()).unwrap();
            let info = action.2 as u8 + (action.1 as u8 * 2);
            file.write_all(&(info as u32).to_le_bytes()).unwrap();
        }
        file
    }
}

// echo


// rush
pub struct Rush {
    pub fps: i16,
    pub actions: Vec<(i32, bool, bool)>,
}

impl Rush {
    pub fn parse(mut file: File) -> Self {
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

        Self {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: i16, mut file: File) -> File {
        file.write_all(&(fps).to_le_bytes()).unwrap();
        for action in 0..actions.len() {
            let action = actions[action];
            let state = action.1 as u8 + (action.2 as u8 * 2);
            file.write_all(&(action.0).to_le_bytes()).unwrap();
            file.write_all(&(state).to_le_bytes()).unwrap();
        }
        file
    }
}

// kdbot
pub struct KDBot {
    pub fps: f32,
    pub actions: Vec<(i32, bool, bool)>,
}

impl KDBot {

    pub fn parse(mut file: File) -> Self {
        let mut buffer: [u8; 4] = [0u8; 4];
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

        Self {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(&(fps).to_le_bytes()).unwrap();
        for action in 0..actions.len() {
            let action = actions[action];
            file.write_all(&(action.0 as u32).to_le_bytes()).unwrap();
            file.write_all(&(action.1 as u8).to_le_bytes()).unwrap();
            file.write_all(&(action.2 as u8).to_le_bytes()).unwrap();
        }
        file
    }
}

// mhr json
pub struct MHRJson {
    pub fps: f32,
    pub actions: Vec<(f32, bool, bool)>,
}

impl MHRJson {
    pub fn parse(mut file: File) -> Self {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let data: serde_json::Value = serde_json::from_str(&*buffer).unwrap();
        let fps = data["meta"]["fps"].as_f64().unwrap() as f32;

        let mut actions: Vec<(f32, bool, bool)> = Vec::new();

        let events = data["events"].as_array().unwrap();

        for action in events {
            if !action["down"].is_null() {
                let frame = action["frame"].as_f64().unwrap() as f32;
                let click = action["down"].as_bool().unwrap();
                let p2 = action["p2"].as_bool().is_some();
                actions.push((frame, click, p2));
            }
        }

        Self {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(i32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(
                b"{{
                \t\"_\": \"Mega Hack v7.1-GM1 Replay\",
                \t\"events\": ["
        ).unwrap();

        for action in actions {
            file.write_all(
                format!(
                    "\t\t{{
                    \t\t\t\"a\": 0,
                    \t\t\t\"down\": {},
                    \t\t\t\"frame\": {},
                    \t\t\t\"r\": 0,
                    \t\t\t\"x\": 0,
                    \t\t\t\"y\": 0,
                    \t\t}},",
                    action.1,
                    action.0,
                ).as_bytes()
            ).unwrap();
        }

        file.write_all(
            format!(
                "\t],
                \t\"meta\": {{
                \t\t\"fps\": {}
                \t}}
                }}",
                fps
            ).as_bytes()
        ).unwrap();
        file
    }
}
