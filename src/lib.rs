mod replay_types;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::replay_types::{frame_based::rush, frame_based::kdbot};

    #[test]
    fn ybot_to_rush() {
        let in_file = File::open("macros/ybot").unwrap();
        let out_file = File::create("macros/from_ybot.rsh").unwrap();
        let mut replay = rush::Rush::parse(in_file);
        let replay = rush::Rush::dump(
            replay.actions.iter().map(|x| (x.0 as i32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps as i16,
            out_file,
        );
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