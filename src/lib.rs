mod replay_types;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::replay_types::{frame_based::zbot_frame, universal_replay, frame_based::rush};

    /*#[test]
    fn url_f_to_zbf() {
        let in_url = File::open("macros/url/url.replay").unwrap();
        let out_zbf = File::create("macros/zbf/from_url.zbf").unwrap();
        let mut replay = universal_replay::URL::parse(in_url, true);
        let zbf_replay = zbot_frame::zBotFrame::dump(
            replay.actions.iter().map(|x| (x.0 as i32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps,
            out_zbf
        );
    }

    #[test]
    fn zbf_to_url_f() {
        let in_zbf = File::open("macros/zbf/zbf.zbf").unwrap();
        let out_url = File::create("macros/url/from_zbf.replay").unwrap();
        let mut replay = zbot_frame::zBotFrame::parse(in_zbf);
        let url_replay = universal_replay::URL::dump(
            replay.actions.iter().map(|x| (x.0 as f32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps,
            true,
            out_url,
        );
    }*/

    #[test]
    fn url_to_rush() {
        let in_url = File::open("macros/url/url.replay").unwrap();
        let out_rush = File::create("macros/rush/from_rush.rsh").unwrap();
        let mut replay = universal_replay::URL::parse(in_url, true);
        let rush_replay = rush::Rush::dump(
            replay.actions.iter().map(|x| (x.0 as i32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps as i16,
            out_rush,
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