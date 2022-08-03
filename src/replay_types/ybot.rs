// function parseyBot(text) {
//     const data = JSON.parse(text);
//     const choice = prompt(`which level? ${Object.keys(data)}`);
//     if (data[choice]) {
//         const fps = 1 / data[choice]['delta_override'];
//         const actions = data[choice]['instructions'].map(instruction => {
//             return {
//                 x: instruction.x,
//                 hold: instruction.press,
//                 player2: instruction.p2
//             };
//         });
//         return {fps, actions};
//     }
// }

// function dumpyBot(replay) {
//     return JSON.stringify({
//         converted: {
//             _reserved: 0,
//             delta_override: 1 / replay.fps,
//             instructions: replay.actions.map(action => {
//                 return {
//                     x: action.x,
//                     press: action.hold,
//                     p2: action.player2
//                 };
//             })
//         }
//     }, null, 2);
// }