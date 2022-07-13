use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> usize;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Playing
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
    status: Option<GameStatus>
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

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

fn random_number(number: usize) -> usize {
    (now() * 10) % number
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> World {
        let size = width * width;
        let snake = Snake::new(snake_index, 3);
        
        World {
            width,
            size, 
            reward_cell: World::generate_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None
        }
    }

    fn generate_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;

        loop {
            reward_cell = random_number(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) { break; }
        }

        reward_cell
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.game_status() {
            Some(GameStatus::Won) => String::from("You won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Playing) => String::from("Playing!"),
            None => String::from("Press to play!"),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_head();
        let world_width = self.width();
        let row = snake_index / world_width;

        match direction {
            Direction::Right => {
                SnakeCell((row * world_width) + (snake_index + 1) % world_width)
            },
            Direction::Left => {
                SnakeCell((row * world_width) + (snake_index - 1) % world_width)
            },
            Direction::Up => { 
                SnakeCell((snake_index - world_width) % self.size())
            },
            Direction::Down => {
                SnakeCell((snake_index + world_width) % self.size())
            },
        }
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Playing);
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Playing) => {
                let temp = self.snake.body.clone();

                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    },
                    None => {
                        let next_cell = self.generate_next_snake_cell(&self.snake.direction);
                        self.snake.body[0] = next_cell;
                    }
                }

                for i in 1..self.snake_length() {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost);
                }

                if self.reward_cell == self.snake_head() {
                    if self.snake_length() < self.size {
                        self.reward_cell = World::generate_reward_cell(self.size(), &self.snake.body);
                    } else {
                        self.reward_cell = 10000;
                        self.status = Some(GameStatus::Won);
                    }

                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            },
            Some(GameStatus::Won) => {

            },
            Some(GameStatus::Lost) => {

            },
            None => {

            }
        };
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }
}
