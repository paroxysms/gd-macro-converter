// function parseTASBOT(text, frame=false) {
//     const data = JSON.parse(text);
//     const fps = data.fps;
//     const actions = [];
//     for (const action of data.macro) {
//         const x = frame ? action.frame : action.player_1.x_position;
//         let h;
//         if (h = action.player_1.click ?? action.player_1_click)
//             actions.push({x, hold: h === 1, player2: false});
//         if (h = action.player_2.click ?? action.player_2_click)
//             actions.push({x, hold: h === 1, player2: true});
//     }
//     return {fps, actions};
// }

// function dumpTASBOT(replay, frame=false) {
//     const data = {
//         fps: replay.fps,
//         macro: replay.actions.map(action => {
//             return {
//                 frame: frame ? action.x : 0,
//                 player_1: {
//                     click: +!action.player2 && (!action.hold + 1),
//                     x_position: frame ? 0 : action.x
//                 },
//                 player_2: {
//                     click: +action.player2 && (!action.hold + 1),
//                     x_position: frame ? 0 : action.x
//                 }
//             };
//         })
//     };
//     return JSON.stringify(data, null, 1);
// }