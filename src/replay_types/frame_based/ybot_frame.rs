// function parseYbotF(view) {
//     const fps = view.getFloat32(4, true);
//     const nActions = view.getInt32(8, true);
//     const actions = [];
//     for (let i = 12; i < 12 + nActions * 8; i += 8) {
//         const frame = view.getUint32(i, true);
//         const idk = view.getUint32(i + 4, true);
//         const hold = (idk & 0b10) === 2;
//         const player2 = (idk & 0b01) === 1;
//         actions.push({x: frame, hold, player2});
//     }
//     return {fps, actions};
// }

// function dumpYbotF(replay) {
//     const buffer = new ArrayBuffer(12 + replay.actions.length * 8);
//     const view = new DataView(buffer);
//     strToBuf('ybot').forEach((n, i) => view.setUint8(i, n));
//     view.setFloat32(4, replay.fps, true);
//     view.setUint32(8, replay.actions.length, true);
//     replay.actions.forEach((action, i) => {
//         view.setInt32(12 + i * 8, action.x, true);
//         let idk = action.player2 + action.hold * 2;
//         view.setUint32(16 + i * 8, idk);
//     });
//     return buffer;
// }