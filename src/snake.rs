use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use web_sys::js_sys::Math;
use crate::canvas;
use crate::canvas::Canvas;
use crate::direction::Direction;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn showGameOverModal(score: u32, highScore: u32);
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

#[derive(Debug)]
pub struct Snake {
    head: Block,
    tail: Vec<Block>,
    food: Block,
    height: u32,
    width: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction,
    score: u32,
    high_score: u32,
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let head_x = (Math::random() * width as f64).floor() as u32;
        let head_y = (Math::random() * height as f64).floor() as u32;
        let food_x = (Math::random() * width as f64).floor() as u32;
        let food_y = (Math::random() * height as f64).floor() as u32;

        let high_score = Self::load_high_score();

        Snake {
            head: Block(head_x, head_y),
            tail: Vec::new(),
            food: Block(food_x, food_y),
            height,
            width,
            direction: None,
            next_direction: None,
            last_direction: Direction::Right,
            score: 0,
            high_score,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if !self.last_direction.opposite(direction) && self.direction.is_none() {
            self.direction = Some(direction);
        } else if let Some(d) = self.direction {
            if !d.opposite(direction) {
                self.next_direction = Some(direction);
            }
        }
    }

    pub fn update(&mut self) {
        let direction = self.direction.take().unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            Direction::Up => Block(self.head.0, (self.head.1 + self.height - 1) % self.height),
            Direction::Down => Block(self.head.0, (self.head.1 + 1) % self.height),
            Direction::Left => Block((self.head.0 + self.width - 1) % self.width, self.head.1), Direction::Right => Block((self.head.0 + 1) % self.width, self.head.1),
        };

        if !self.tail.contains(&new_head) {
            self.tail.insert(0, self.head);
            self.head = new_head;

            if new_head == self.food {
                self.score += 1;
                if self.score > self.high_score {
                    self.high_score = self.score;
                    Self::save_high_score(self.high_score);
                }
                self.generate_food();
            } else {
                self.tail.pop();
            }
        } else {
            // Reset the game if the snake collides with itself
            console::log_1(&"Game Over!".into());
            console::log_1(&format!("Final Score: {}", self.score).into());
            showGameOverModal(self.score, self.high_score);
        }

        self.direction = self.next_direction.take();
    }

    fn load_high_score() -> u32 {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("high_score")
            .unwrap()
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or(0)
    }

    fn save_high_score(high_score: u32) {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("high_score", &high_score.to_string())
            .unwrap();
    }

    fn generate_food(&mut self) {
        loop {
            let food_x = (Math::random() * self.width as f64).floor() as u32;
            let food_y = (Math::random() * self.height as f64).floor() as u32;
            let potential_food = Block(food_x, food_y);
            if !self.tail.contains(&potential_food) && potential_food != self.head {
                self.food = potential_food;
                break;
            }
        }
    }

    pub fn draw(&self, canvas: &canvas::Canvas) {
        canvas.clear_all();
        canvas.draw(self.head.0, self.head.1, "green");
        for block in &self.tail {
            canvas.draw(block.0, block.1, "lightgreen");
        }
        canvas.draw(self.food.0, self.food.1, "red");
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.width, self.height);
    }
}