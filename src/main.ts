import './styles.css'

import init, { World, Direction } from 'snake_game_wasm'

const canvas = document.querySelector(".wrapper #snake-screen") as HTMLCanvasElement

(async () => {
  await init()

  const WORLD_WIDTH = 16
  const SNAKE_INDEX = Date.now() % (WORLD_WIDTH * WORLD_WIDTH)

  const world = World.new(WORLD_WIDTH, SNAKE_INDEX)

  const context = canvas.getContext("2d") as CanvasRenderingContext2D
  const CELL_SIZE = 10

  const worldWidth = world.width()
  const worldSize = worldWidth * CELL_SIZE

  canvas.height = worldSize
  canvas.width = worldSize

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

  function drawSnake() {
    const snakeIndex = world.snake_head()
    const col = snakeIndex % worldWidth
    const row = Math.floor(snakeIndex / worldWidth)

    context.beginPath()

    context.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE)

    context.stroke()
  }

  function paint() {
    drawWorld()
    drawSnake()
  }

  function update() {
    const FPS = 5
    setTimeout(() => {
      context.clearRect(0, 0, canvas.width, canvas.height)
      world.update()
      paint()
      requestAnimationFrame(update)
    }, 1000 / FPS)
  }
  
  paint()
  update()

})()

