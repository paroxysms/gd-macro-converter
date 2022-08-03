// function parsezBot(view) {
//     const delta = view.getFloat32(0, true);
//     const speedhack = view.getFloat32(4, true);
//     const fps = 1 / delta / speedhack
//     const actions = [];
//     for (let i = 8; i < view.byteLength; i += 6) {
//         const x = view.getFloat32(i, true);
//         // once again i will make fun of fig for using 0x30 and 0x31
//         const hold = view.getUint8(i + 4) === 0x31;
//         const player1 = view.getUint8(i + 5) === 0x31;
//         actions.push({x, hold, player2: !player1});
//     }
//     return {fps, actions};
// }

// function dumpzBot(replay) {
//     const buffer = new ArrayBuffer(8 + replay.actions.length * 6);
//     const view = new DataView(buffer);
//     view.setFloat32(0, 1 / replay.fps, true);
//     view.setFloat32(4, 1, true);
//     replay.actions.forEach((action, i) => {
//         view.setFloat32(8 + i * 6, action.x, true);
//         view.setUint8(12 + i * 6, action.hold ? 0x31 : 0x30);
//         view.setUint8(13 + i * 6, !action.player2 ? 0x31 : 0x30);
//     });
//     return buffer;
// }