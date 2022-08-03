// function parseEcho(text, frame) {
//     const data = JSON.parse(text);
//     const fps = data.FPS;
//     // thanks krx
//     const startingFrame = data['Starting Frame'];
//     const actions = data['Echo Replay'].map(action => ({
//         x: frame ? action.Frame + startingFrame : action['X Position'],
//         hold: action.Hold,
//         player2: action['Player 2']
//     }));
//     return {fps, actions};
// }

// function dumpEcho(replay, frame) {
//     return JSON.stringify({
//         FPS: replay.fps,
//         'Starting Frame': 0,
//         'Echo Replay': replay.actions.map(action => ({
//             Hold: action.hold,
//             'Player 2': action.player2,
//             Frame: frame ? action.x : 0,
//             'X Position': frame ? 0 : action.x
//         }))
//     }, null, 4);
// }