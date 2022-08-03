// function parsexBot(text) {
//     const lines = text.split('\n');
//     const fps = parseInt(lines.splice(0,1)[0].split(' ')[1].trim());
//     if (lines[0].trim() !== 'pro_plus') {
//         alert('xbot only works with pro+');
//         return;
//     }
//     lines.splice(0,1);
//     const actions = [];
//     // for converting the x pos
//     const buffer = new ArrayBuffer(4);
//     const view = new DataView(buffer);
//     for (const line of lines) {
//         if (!line.trim()) continue;
//         const [state, rawPos] = line.trim().split(' ').map(i => parseInt(i));
//         // state:
//         // 0 - release
//         // 1 - hold
//         // 2 - p2 release
//         // 3 - p2 hold
//         const player2 = state > 1;
//         const hold = state % 2 == 1;
//         view.setUint32(0, rawPos);
//         const x = view.getFloat32(0);
//         actions.push({ x, hold, player2 });
//     }
//     return {fps, actions};
// }

// function dumpxBot(replay) {
//     let final = `fps: ${Math.round(replay.fps)}\r\npro_plus\r\n`;
//     const buffer = new ArrayBuffer(4);
//     const view = new DataView(buffer);
//     replay.actions.forEach(action => {
//         // amazing
//         const state = action.hold + 2 * action.player2;
//         view.setFloat32(0, action.x);
//         const pos = view.getUint32(0);
//         final += `${state} ${pos}\r\n`;
//     });
//     return final.slice(0, final.length - 1);
// }