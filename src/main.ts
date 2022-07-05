import init, { greet, sum } from 'snake_game_wasm';
// Don't worry if vscode told you can't find my-crate
// It's because you're using a local crate
// after yarn dev, wasm-pack plugin will install my-crate for you

init().then(() => {
  console.log('init wasm-pack');
  greet('from vite!');
  console.log("sum from wasm", sum(2, 2));
  
});