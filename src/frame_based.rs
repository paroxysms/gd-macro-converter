use std::{io::{Read, Write}, fs::File};

pub struct PlainText {
    pub fps: f32,
    pub actions: Vec<(f32, bool, bool)>,
}
impl PlainText {
    pub fn parse(mut file: File) -> Self {
        let buffer = &mut String::new();
        file.read_to_string(buffer).unwrap();

        let lines: Vec<&str> = buffer.split("\n").collect();

        let fps = lines.get(0).unwrap().trim().parse::<f32>().unwrap();
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

pub struct xBotFrame {
    pub fps: u32,
    pub actions: Vec<(f32, bool, bool)>,
}
impl xBotFrame {
    pub fn parse(mut file: File) -> Self {
        let mut buffer = &mut String::new();
        file.read_to_string(buffer).unwrap();

        let lines: Vec<&str> = buffer.split("\n").collect();

        let fps = lines.get(0).unwrap()[5..8].parse::<u32>().unwrap();
        let mut actions: Vec<(f32, bool, bool)> = Vec::new();

        for line in lines {
            if !line.trim().is_empty() {
                let data: Vec<&str> = line.split_whitespace().collect();

                if data.get(0).unwrap() != &"fps:" && data.get(0).unwrap() != &"frames" {
                    let state = data.get(0).unwrap().parse::<u8>().unwrap();
                    let pos = data.get(1).unwrap().parse::<u32>().unwrap();

                    let player2 = state > 1;
                    let click = state % 2 == 1;
                    let frame = pos as f32;

                    actions.push((frame, click, player2));
                }
            }
        }

        Self {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(f32, bool, bool)>, fps: u32, mut file: File) -> File {
        file.write_all(format!("fps: {}\nframes\n", fps).as_ref()).unwrap();
        for action in actions {
            file.write_all(format!("{} {}\n", action.1 as u8, action.0).as_ref()).unwrap();
        }
        file
    }
}

pub struct yBotFrame {
    pub fps: f32,
    pub actions: Vec<(u32, bool, bool)>,
}
impl yBotFrame {
    pub fn parse(mut file: File) -> Self {
        let mut buffer: [u8; 4] = [0u8; 4];

        file.read(&mut buffer).unwrap();

        file.read(&mut buffer).unwrap();
        let fps = f32::from_le_bytes(buffer);

        file.read(&mut buffer).unwrap();

        let mut actions: Vec<(u32, bool, bool)> = Vec::new();

        loop {
            if file.read(&mut buffer).unwrap() != 4 { break; }
            let frame = u32::from_le_bytes(buffer);

            if file.read(&mut buffer).unwrap() != 4 { break; }
            let state = u32::from_le_bytes(buffer);
            let click = state & 0b10 == 2;
            let player2 = state & 0b01 == 1;

            actions.push((frame, click, player2));
        }

        Self {
            fps,
            actions,
        }
    }

    pub fn dump(actions: Vec<(u32, bool, bool)>, fps: f32, mut file: File) -> File {
        file.write_all(&("ybot".as_bytes())).unwrap();
        file.write_all(&(fps).to_le_bytes()).unwrap();
        file.write_all(&(actions.len() as u32).to_le_bytes()).unwrap();
        for action in 0..actions.len() {
            let action = actions[action];
            file.write_all(&(action.0).to_le_bytes()).unwrap();

            let info = action.2 as u8 + (action.1 as u8 * 2);
            file.write_all(&(info as u32).to_be_bytes()).unwrap();
        }
        file
    }
}

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
            b"{\n \"_\": \"Generated from macro converter\",\n \"events\": [\n"
        ).unwrap();

        for action in &actions {
            let index = &actions.iter().position(|&r| r == *action).unwrap();
            if index == &(actions.len() - 1) {
                file.write_all(
                    format!(
                        "  {{\n   \"a\": 0,\n   \"down\": {},\n   \"frame\": {},\n   \"r\": 0,\n   \"x\": 0,\n   \"y\": 0\n  }}\n",
                        action.1,
                        action.0,
                    ).as_bytes()
                ).unwrap();
            } else {
                file.write_all(
                    format!(
                        "  {{\n   \"a\": 0,\n   \"down\": {},\n   \"frame\": {},\n   \"r\": 0,\n   \"x\": 0,\n   \"y\": 0\n  }},\n",
                        action.1,
                        action.0,
                    ).as_bytes()
                ).unwrap();
            }
        }

        file.write_all(
            format!(
                " ],\n \"meta\": {{\n  \"fps\": {}\n }}\n}}",
                fps
            ).as_bytes()
        ).unwrap();
        file
    }
}
