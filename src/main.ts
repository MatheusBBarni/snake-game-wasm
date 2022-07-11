import './styles.css'

import init, { World, Direction, GameStatus } from 'snake_game_wasm'

const canvas = document.querySelector("#snake-screen") as HTMLCanvasElement
const gameControlButton = document.querySelector("#game-control") as HTMLButtonElement
const gameStatusLabel = document.querySelector("#game-status") as HTMLDivElement

(async () => {
  const wasm = await init()

  const REWARD_CELL_COLOR = '#FF0000'
  const SNAKE_HEAD_COLOR = '#7878db'
  const SNAKE_BODY_COLOR = '#404040'
  const WORLD_WIDTH = 16
  const SNAKE_INDEX = Date.now() % (WORLD_WIDTH * WORLD_WIDTH)

  const world = World.new(WORLD_WIDTH, SNAKE_INDEX)

  const context = canvas.getContext("2d") as CanvasRenderingContext2D
  const CELL_SIZE = 10

  const worldWidth = world.width()
  const worldSize = worldWidth * CELL_SIZE

  canvas.height = worldSize
  canvas.width = worldSize

  gameControlButton.addEventListener('click', () => {
    if (world.game_status() !== undefined) {
      location.reload()
      return
    }
    world.start_game()
    gameStatusLabel.innerHTML = 'Playing'
    gameControlButton.innerHTML = 'Reload'
    play()
  })

  document.addEventListener('keydown', ({ code }: KeyboardEvent) => {
    switch (code) {
      case 'ArrowUp':
        world.change_snake_direction(Direction.Up)
        break
      case 'ArrowRight':
        world.change_snake_direction(Direction.Right)
        break
      case 'ArrowDown':
        world.change_snake_direction(Direction.Down)
        break
      case 'ArrowLeft':
        world.change_snake_direction(Direction.Left)
        break
      default:
        console.log(`Key: ${code} don't work!`)
        break
    }
  })

  function drawWorld() {
    context.beginPath()

    for (let x = 0; x < worldWidth + 1; x++) {
      context.moveTo(CELL_SIZE * x, 0)
      context.lineTo(CELL_SIZE * x, worldSize)
    }
    for (let y = 0; y < worldWidth + 1; y++) {
      context.moveTo(0, CELL_SIZE * y)
      context.lineTo(worldSize, CELL_SIZE * y)
    }

    context.stroke()
  }

  function drawReward() {
    const index = world.reward_cell()
    const col = index % worldWidth
    const row = Math.floor(index / worldWidth)

    context.beginPath()
    context.fillStyle = REWARD_CELL_COLOR

    context.fillRect(
      col * CELL_SIZE,
      row * CELL_SIZE,
      CELL_SIZE,
      CELL_SIZE
    )
    
    context.stroke()
  }

  function drawSnake() {
    const snakeCellPtr = world.snake_cells()
    const snakeLength = world.snake_length()
    const snakeCells = new Uint32Array(wasm.memory.buffer, snakeCellPtr, snakeLength)

    snakeCells.forEach((cellIndex, index) => {
      const col = cellIndex % worldWidth
      const row = Math.floor(cellIndex / worldWidth)

      context.fillStyle = index === 0 ? SNAKE_HEAD_COLOR : SNAKE_BODY_COLOR

      context.beginPath()

      context.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE)
    })
    context.stroke()
  }

  function paint() {
    drawWorld()
    drawSnake()
    drawReward()
  }

  function play() {
    const FPS = 5
    setTimeout(() => {
      context.clearRect(0, 0, canvas.width, canvas.height)
      world.step()
      paint()
      requestAnimationFrame(play)
    }, 1000 / FPS)
  }
  
  paint()

})()

