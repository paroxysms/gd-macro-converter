// function parseKDBot(view) {
//     const fps = view.getFloat32(0, true);
//     const actions = [];
//     for (let i = 4; i < view.byteLength; i += 6) {
//         const frame = view.getInt32(i, true);
//         const hold = view.getUint8(i + 4) === 1;
//         const player2 = view.getUint8(i + 5) === 1;
//         actions.push({x: frame, hold, player2});
//     }
//     return {fps, actions};
// }

// function dumpKDBot(replay) {
//     const buffer = new ArrayBuffer(4 + replay.actions.length * 6);
//     const view = new DataView(buffer);
//     view.setFloat32(0, replay.fps, true);
//     replay.actions.forEach((action, i) => {
//         view.setUint32(4 + i * 6, action.x, true);
//         view.setUint8(8 + i * 6, action.hold);
//         view.setUint8(9 + i * 6, action.player2);
//     });
//     return buffer;
// }