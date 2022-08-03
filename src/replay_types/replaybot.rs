// function parseReplayBot(view) {
//     let magic = String.fromCharCode(...new Uint8Array(view.buffer.slice(0, 4)));
//     if (magic === 'RPLY') {
//         const version = view.getUint8(4);
//         if (version === 1 || version === 2) {
//             let offset = 0;
//             let frame = false;
//             if (version === 2) {
//                 offset = 1;
//                 frame = view.getUint8(5) === 1;
//             }
//             const fps = view.getFloat32(5 + offset, true);
//             const actions = [];
//             for (let i = 9 + offset; i < view.byteLength; i += 5) {
//                 const x = frame ? view.getUint32(i, true) : view.getFloat32(i, true);
//                 const state = view.getUint8(i + 4);
//                 const hold = !!(state & 0x1);
//                 const player2 = !!(state >> 1);
//                 actions.push({x, hold, player2});
//             }
//             return {fps, actions};
//         }
//     } else {
//         const fps = view.getFloat32(0, true);
//         const actions = [];
//         for (let i = 4; i < view.byteLength; i += 6) {
//             const x = view.getFloat32(i, true);
//             const hold = view.getUint8(i + 4) === 1;
//             const player2 = view.getUint8(i + 5) === 1;
//             actions.push({x, hold, player2});
//         }
//         return {fps, actions};
//     }
// }

// function dumpReplayBot(replay, frame=false) {
//     const buffer = new ArrayBuffer(10 + replay.actions.length * 5);
//     const view = new DataView(buffer);
//     strToBuf('RPLY').forEach((n, i) => view.setUint8(i, n));
//     view.setUint8(4, 2); // version
//     view.setUint8(5, +frame); // type
//     view.setFloat32(6, replay.fps, true);
//     replay.actions.forEach((action, i) => {
//         if (frame)
//             view.setUint32(10 + i * 5, action.x, true);
//         else
//             view.setFloat32(10 + i * 5, action.x, true);
//         const state = action.hold | (action.player2 << 1);
//         view.setUint8(14 + i * 5, state);
//     });
//     return buffer;
// }