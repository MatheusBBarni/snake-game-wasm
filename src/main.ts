import init, { greet, sum } from 'snake_game_wasm'

async function main() {
  await init()


  console.log('init wasm-pack')
  greet('from vite!')
  console.log("sum from wasm", sum(2, 2))
}

main()
