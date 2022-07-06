use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

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
    pub fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec!(SnakeCell(spawn_index)),
            direction: Direction::Left
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> World {
        World {
            width,
            size: width * width, 
            snake: Snake::new(snake_index)
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
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
