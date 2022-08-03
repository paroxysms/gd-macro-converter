// function parseMHRjson(text) {
//     const data = JSON.parse(text);
//     const fps = data.meta.fps;
//
//     const actions = [];
//     for (const action of data.events) {
//         if (action.hasOwnProperty("down")) {
//             const x = action.frame;
//             const hold = action.down;
//             actions.push({x, hold, player2: !!action.p2});
//         }
//     }
//     return {fps, actions};
// }

// function dumpMHRjson(replay) {
//     // Does not support any physics.
//     const data = {
//         "_": "Generated from macro converter",
//         "events": replay.actions.map(action => {
//             let e = {
//                 "a": 0, // used for physics
//                 "down": action.hold,
//                 "frame": action.x,
//                 "r": 0, // used for physics
//                 "x": 0,
//                 "y": 0
//             }
//             if (action.player2) e.p2 = true;
//             return e;
//         }),
//         "meta": {"fps": replay.fps}
//     }
//     return JSON.stringify(data, null, 1);
// }