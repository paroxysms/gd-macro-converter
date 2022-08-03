// function parseDDHOR(view) {
//     let magic = String.fromCharCode(...new Uint8Array(view.buffer.slice(0, 4)));
//     if (magic === 'DDHR') {
//         const fps = view.getInt16(4, true);
//         const player1ActionCount = view.getInt32(6, true);
//         const player2ActionCount = view.getInt32(10, true);
//         console.log(player1ActionCount, player2ActionCount);
//         const actions = [];
//         for (let i = 14; i < view.byteLength; i += 5) {
//             const x = view.getFloat32(i, true);
//             const action = view.getUint8(i + 4);
//             const player2 = i - 14 >= player1ActionCount * 5;
//             actions.push({x, hold: action == 0, player2});
//         }
//         actions.sort((a, b) => a.x - b.x);
//
//         return {fps, actions};
//     } else {
//         const data = JSON.parse(new TextDecoder().decode(new Uint8Array(view.buffer)));
//         let type = data.macro;
//         const fps = data.fps;
//         if (type === 'x-position') {
//             const actions = [
//                 ...data.inputsP1.map(ipt => {
//                     return {
//                         x: ipt.position,
//                         hold: ipt.action === 'PUSH',
//                         player2: false
//                     };
//                 }),
//                 ...data.inputsP2.map(ipt => {
//                     return {
//                         x: ipt.position,
//                         hold: ipt.action === 'PUSH',
//                         player2: true
//                     };
//                 }),
//             ];
//             actions.sort((a, b) => a.x - b.x);
//             return {fps, actions};
//         }
//     }
// }

// function dumpDDHOR(replay) {
//     const player1Actions = replay.actions.filter(i => !i.player2);
//     const player2Actions = replay.actions.filter(i => i.player2);
//     return JSON.stringify({
//         fps: Math.round(replay.fps),
//         levelID: null,
//         macro: 'x-position',
//         inputsP1: player1Actions.map(action => {
//             return {
//                 action: action.hold ? 'PUSH' : 'RELEASE',
//                 position: action.x
//             };
//         }),
//         inputsP2: player2Actions.map(action => {
//             return {
//                 action: action.hold ? 'PUSH' : 'RELEASE',
//                 position: action.x
//             };
//         })
//     });
// }