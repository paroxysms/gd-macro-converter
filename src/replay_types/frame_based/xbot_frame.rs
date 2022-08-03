// function parsexBotFrame(text) {
//     const lines = text.split('\n');
//     const fps = parseInt(lines.splice(0,1)[0].split(' ')[1].trim());
//     if (lines[0].trim() !== 'frames') {
//         alert('not a frame');
//         return;
//     }
//     lines.splice(0,1);
//     const actions = [];
//     for (const line of lines) {
//         if (!line.trim()) continue;
//         const [state, rawPos] = line.trim().split(' ').map(i => parseInt(i));
//         const player2 = state > 1;
//         const hold = state % 2 == 1;
//         actions.push({ x: parseInt(rawPos), hold, player2 });
//     }
//     return {fps, actions};
// }

// function dumpxBotFrame(replay) {
//     let final = `fps: ${Math.round(replay.fps)}\r\nframes\r\n`;
//     replay.actions.forEach(action => {
//         const state = action.hold + 2 * action.player2;
//         final += `${state} ${Math.floor(action.x)}\r\n`;
//     });
//     return final.slice(0, final.length - 1);
// }