mod frame_based;
mod position_based;
mod variable_based;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use serde_json::Value;
    use crate::frame_based::{KDBot, MHRJson, PlainText, Rush, xBotFrame, yBotFrame, zBotFrame};

    #[test]
    fn mhr_json_to_plain_txt() {
        let in_file = File::open("macros/frame/checked/vsc.mhr.json").unwrap();
        let out_file = File::create("macros/frame/vsc.txt").unwrap();
        let replay = MHRJson::parse(in_file);
        let replay = PlainText::dump(
            replay.actions.iter().map(|x| (x.0, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps,
            out_file,
        );

        let created_file = &mut String::new();
        File::open("macros/frame/vsc.txt").unwrap().read_to_string(created_file).unwrap();

        let checked_file = &mut String::new();
        File::open("macros/frame/checked/vsc.txt").unwrap().read_to_string(checked_file).unwrap();

        assert_eq!(created_file.trim(), checked_file.replace("\r", "").trim());
    }

    #[test]
    fn plain_txt_to_zbot_frame() {
        let in_file = File::open("macros/frame/checked/vsc.txt").unwrap();
        let out_file = File::create("macros/frame/vsc.zbf").unwrap();
        let replay = PlainText::parse(in_file);
        let replay = zBotFrame::dump(
            replay.actions.iter().map(|x| (x.0 as i32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps,
            out_file,
        );

        let created_file: &mut Vec<u8> = &mut Vec::new();
        File::open("macros/frame/vsc.zbf").unwrap().read_to_end(created_file).unwrap();

        let checked_file: &mut Vec<u8> = &mut Vec::new();
        File::open("macros/frame/checked/vsc.zbf").unwrap().read_to_end(checked_file).unwrap();

        assert_eq!(created_file, checked_file);
    }

    #[test]
    fn zbot_frame_to_xbot_frame() {
        let in_file = File::open("macros/frame/checked/vsc.zbf").unwrap();
        let out_file = File::create("macros/frame/vsc.xbot").unwrap();
        let replay = zBotFrame::parse(in_file);
        let replay = xBotFrame::dump(
            replay.actions.iter().map(|x| (x.0 as f32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps as u32,
            out_file,
        );

        let created_file = &mut String::new();
        File::open("macros/frame/vsc.xbot").unwrap().read_to_string(created_file).unwrap();

        let checked_file = &mut String::new();
        File::open("macros/frame/checked/vsc.xbot").unwrap().read_to_string(checked_file).unwrap();

        assert_eq!(created_file.trim(), checked_file.replace("\r", ""));
    }

    #[test]
    fn xbot_frame_to_ybot_frame() {
        let in_file = File::open("macros/frame/checked/vsc.xbot").unwrap();
        let out_file = File::create("macros/frame/vsc").unwrap();
        let replay = xBotFrame::parse(in_file);
        let replay = yBotFrame::dump(
            replay.actions.iter().map(|x| (x.0 as u32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps as f32,
            out_file,
        );

        let created_file: &mut Vec<u8> = &mut Vec::new();
        File::open("macros/frame/vsc").unwrap().read_to_end(created_file).unwrap();

        let checked_file: &mut Vec<u8> = &mut Vec::new();
        File::open("macros/frame/checked/vsc").unwrap().read_to_end(checked_file).unwrap();

        assert_eq!(created_file, checked_file);
    }

    #[test]
    fn ybot_frame_to_rush() {
        let in_file = File::open("macros/frame/checked/vsc").unwrap();
        let out_file = File::create("macros/frame/vsc.rsh").unwrap();
        let replay = yBotFrame::parse(in_file);
        let replay = Rush::dump(
            replay.actions.iter().map(|x| (x.0 as i32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps as i16,
            out_file,
        );

        let created_file: &mut Vec<u8> = &mut Vec::new();
        File::open("macros/frame/vsc.rsh").unwrap().read_to_end(created_file).unwrap();

        let checked_file: &mut Vec<u8> = &mut Vec::new();
        File::open("macros/frame/checked/vsc.rsh").unwrap().read_to_end(checked_file).unwrap();

        assert_eq!(created_file, checked_file);
    }

    #[test]
    fn rush_to_kd_bot() {
        let in_file = File::open("macros/frame/checked/vsc.rsh").unwrap();
        let out_file = File::create("macros/frame/vsc.kd").unwrap();
        let replay = Rush::parse(in_file);
        let replay = KDBot::dump(
            replay.actions.iter().map(|x| (x.0, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps as f32,
            out_file,
        );

        let created_file: &mut Vec<u8> = &mut Vec::new();
        File::open("macros/frame/vsc.kd").unwrap().read_to_end(created_file).unwrap();

        let checked_file: &mut Vec<u8> = &mut Vec::new();
        File::open("macros/frame/checked/vsc.kd").unwrap().read_to_end(checked_file).unwrap();

        assert_eq!(created_file, checked_file);
    }

    #[test]
    fn kd_bot_to_mhr_json() {
        let in_file = File::open("macros/frame/checked/vsc.kd").unwrap();
        let out_file = File::create("macros/frame/vsc.final.mhr.json").unwrap();
        let replay = KDBot::parse(in_file);
        let replay = MHRJson::dump(
            replay.actions.iter().map(|x| (x.0, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps as f32,
            out_file,
        );

        let created_file = &mut String::new();
        File::open("macros/frame/vsc.final.mhr.json").unwrap().read_to_string(created_file).unwrap();

        let checked_file = &mut String::new();
        File::open("macros/frame/checked/vsc.final.mhr.json").unwrap().read_to_string(checked_file).unwrap();

        assert_eq!(created_file, checked_file.replace("\r", "").trim());
    }
}

// function cleanReplay(replay) {
//     let last1 = false;
//     let last2 = false;
//     let n = 0;
//     let final = [];
//     replay.actions.forEach(action => {
//         if (action.hold == (action.player2 ? last2 : last1)) {
//             ++n;
//             return;
//         }
//         if (action.player2) last2 = action.hold;
//         else last1 = action.hold;
//         final.push(action);
//     });
//     if (n) console.log(`Removed ${n} reduntant actions`);
//     replay.actions = final;
//     updateTxt();
// }

// const extensions = {
//     replaybot: 'replay',
//     replaybotf: 'replay',
//     zbot: 'zbot',
//     ybot: 'dat',
//     ddhor: 'ddhor',
//     'ddhor-new': 'ddhor',
//     xbot: 'xbot',
//     kdbot: 'kd',
//     zbf: 'zbf',
//     'xbot-frame': 'xbot',
//     'ybot-frame': null, // why
//     'url': 'replay',
//     'url-f': 'replay',
//     'rush': 'rsh',
// 	   'mhrjson': 'json'