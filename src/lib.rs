mod replay_types;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::replay_types::{frame_based::zbot_frame, universal_replay};

    #[test]
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
        let out_url = File::create("macros/url/from_zbf.url").unwrap();
        let mut replay = zbot_frame::zBotFrame::parse(in_zbf);
        let url_replay = universal_replay::URL::dump(
            replay.actions.iter().map(|x| (x.0 as f32, x.1, x.2)).collect::<Vec<_>>(),
            replay.fps,
            true,
            out_zbf,
        );
    }
}

/*
echo frame
kdbot frame
replaybot frame
rush frame
tasbot frame
url frame
xbot frame
ybt frame

ddhor
echo
replaybot
tasbot
url
xbot
ybot
 */