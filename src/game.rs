use piston_window::types::Color;
use piston_window::*;
use serde_json::{Result, Value};
use std::fs;
use std::io::Read;
use std::process;

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::Direction;
use crate::snake::Snake;

const FOOD_COLOR: Color = [0.94, 0.14, 0.23, 1.0];
const BORDER_COLOR: Color = [0.9, 0.9, 0.9, 1.0];
const GAME_OVER_COLOR: Color = [0.90, 0.00, 0.00, 0.3];

const CONFIG_PATH: &str = "./config.json";
const SCORE_PATH: &str = "./score.txt";

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
    moving_period: f64,
    restart_time: f64,
    score: i32,
    game_score: i32,
}

impl Game {
    // impl function
    // function to initiate a new game
    pub fn new(width: i32, height: i32) -> Game {
        // =========================================================
        // Reading the config variables & If config file is read successfully we init the game
        match fs::read_to_string(CONFIG_PATH) {
            Ok(file_contents) => {
                let file_contents_str: &str = &file_contents;
                let parsed: Result<Value> = serde_json::from_str(&file_contents_str);

                match parsed {
                    Ok(config) => {
                        let food_exists = config["food_exists"].as_bool().unwrap_or(true);
                        let moving_period = config["moving_period"].as_f64().unwrap_or(0.1);
                        let restart_time = config["restart_time"].as_f64().unwrap_or(1.0);

                        println!("===================[GAME CONFIG]===================");
                        println!("Food exists: {}", food_exists);
                        println!("Moving period: {}", moving_period);
                        println!("Restart time: {}", restart_time);
                        println!("===================================================");

                        match fs::read_to_string(SCORE_PATH) {
                            Ok(score) => {
                                return Game {
                                    snake: Snake::new(2, 2),
                                    waiting_time: 0.0,
                                    food_exists,
                                    food_x: 6,
                                    food_y: 4,
                                    width,
                                    height,
                                    game_over: false,
                                    moving_period,
                                    restart_time,
                                    score: score.parse::<i32>().unwrap_or(0),
                                    game_score: 0,
                                };
                            }
                            Err(err) => {
                                eprintln!("Error parsing score.txt file | Either score.txt file doesn't exist: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "Error parsing JSON | Either config.json params are not correct sample file [{{
  'food_exists': true,
  'moving_period': 0.1,
  'restart_time': 1.0
}}] : {}",
                            err
                        );
                    }
                }
            }
            Err(err) => {
                eprintln!(
                    "Error reading file | Either config.json file doesn't exist: {}",
                    err
                );
            }
        }
        // Exiting the process here
        // ------------------------
        Game::exit();

        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            moving_period: 0.5,
            restart_time: 1.0,
            score: 0,
            game_score: 0,
        }
        // =========================================================
    }

    // impl function
    // function to handle key press in the game
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    // impl function
    // function to draw the snake, foods & border in the graphic generated
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    // impl function
    // function to update the game state
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > self.restart_time {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > self.moving_period {
            self.update_snake(None);
        }
    }

    // impl function
    // function to check whether that the snake is eating the food
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
            self.game_score += 1;
        }
    }

    // impl function
    // function to check if the snake is alive
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // when snake touches the borders
        return next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1;
    }

    // impl function
    // function to add the food into the graphic canvas
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    // impl function
    // function to add update the snake if the snake is alive else restart
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    // impl function
    // function to restart the game
    fn restart(&mut self) {
        match self.update_score() {
            Ok(()) => {
                self.snake = Snake::new(2, 2);
                self.waiting_time = 0.0;
                self.food_exists = true;
                self.food_x = 6;
                self.food_y = 4;
                self.game_over = false;
                self.game_score = 0;
            }
            Err(err) => {
                eprintln!("Some error while updting the score : {}", err);

                self.snake = Snake::new(2, 2);
                self.waiting_time = 0.0;
                self.food_exists = true;
                self.food_x = 6;
                self.food_y = 4;
                self.game_over = false;
                self.game_score = 0;
            }
        }
    }

    fn update_score(&mut self) -> std::io::Result<()> {
        let _score = self.game_score;

        println!("===================[SCOREBOARD]===================");
        println!("Current Game Score : {}", self.game_score);
        println!("High Score : {}", self.score);
        println!("==================================================");

        if self.score < _score {
            match fs::File::open(SCORE_PATH) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    let _score_str = _score.to_string();
                    match fs::write(SCORE_PATH, _score_str) {
                        Ok(()) => {}
                        Err(err) => {
                            eprintln!(
                            "Error writing score.txt file | Either score.txt file doesn't exist: {}",err
                            );
                        }
                    }
                    self.score = _score;
                }
                Err(err) => {
                    eprintln!(
                        "Error parsing score.txt file | Either score.txt file doesn't exist: {}",
                        err
                    );
                }
            }
        } else {
            println!("Not Enough Score !!!");
        }

        return Ok(());
    }

    // impl function
    // function to restart the game
    fn exit() {
        process::exit(1);
    }
}
