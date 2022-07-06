use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

impl Snake {
    pub fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> World {
        World {
            width,
            size: width * width, 
            snake: Snake::new(snake_index, 3)
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn update(&mut self) {
        let snake_index = self.snake_head();
        let world_width = self.width();

        let (row, col) = self.index_to_cell(snake_index);

        let (row, col) = match self.snake.direction {
            Direction::Right => {
                (row, (col + 1) % world_width)
            },
            Direction::Left => {
                (row, (col - 1) % world_width)
            },
            Direction::Up => {
                ((row - 1) % world_width, col)
            },
            Direction::Down => {
                ((row + 1) % world_width, col)
            },
        };

        self.set_snake_head(self.cell_to_index(row, col));
    }

    fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index;
    }

    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        let row = index / self.width();
        let col = index % self.width();

        (row, col)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width()) + col
    }
}
