mod frame_based;
mod position_based;
mod variable_based;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use serde_json::Value;
    use crate::frame_based;
    use crate::frame_based::{MHRJson, PlainText, zBotFrame};

    #[test]
    fn mhr_json_to_plain_txt() {
        let in_file = File::open("macros/frame/vsc.mhr.json").unwrap();
        let out_file = File::create("macros/frame/vsc.txt").unwrap();
        let mut replay = MHRJson::parse(in_file);
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
        let in_file = File::open("macros/frame/vsc.txt").unwrap();
        let out_file = File::create("macros/frame/vsc.zbf").unwrap();
        let mut replay = PlainText::parse(in_file);
        let replay = zBotFrame::dump(
            replay.actions.iter().map(|x| (x.0 as i32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps,
            out_file,
        );

        let created_file = &mut String::new();
        File::open("macros/frame/vsc.zbf").unwrap().read_to_string(created_file).unwrap();

        let checked_file = &mut String::new();
        File::open("macros/frame/checked/vsc.zbf").unwrap().read_to_string(checked_file).unwrap();

        assert_eq!(created_file.trim(), checked_file.replace("\r", "").trim());
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