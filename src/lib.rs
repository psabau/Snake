use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, KeyboardEvent, Window};

mod canvas;
mod direction;
mod snake;

use crate::canvas::Canvas;
use crate::direction::Direction;
use crate::snake::Snake;

use std::cell::RefCell;
use std::rc::Rc;

static mut GAME_STATE: Option<Rc<RefCell<Snake>>> = None;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Possibly move some initialization logic here if needed for the first page load
    Ok(())
}

#[wasm_bindgen]
pub fn start_game() {
    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let canvas = Canvas::new("#canvas", 20, 20).expect("Failed to create Canvas");

    // Safely manage game state, ensuring we can reset it for game restarts
    unsafe {
        GAME_STATE = Some(Rc::new(RefCell::new(Snake::new(20, 20))));
    }

    let snake = unsafe { GAME_STATE.as_ref().unwrap().clone() };
    snake.borrow().draw(&canvas);

    // Setup keyboard event listener
    setup_keyboard_listener(snake.clone());

    // Start or restart the game loop
    game_loop(snake, Rc::new(canvas), 100).expect("Failed to start the game loop");
}

fn setup_keyboard_listener(snake: Rc<RefCell<Snake>>) {
    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let dir = match event.key().as_str() {
            "ArrowLeft" => Some(Direction::Left),
            "ArrowRight" => Some(Direction::Right),
            "ArrowDown" => Some(Direction::Down),
            "ArrowUp" => Some(Direction::Up),
            _ => None,
        };

        if let Some(direction) = dir {
            snake.borrow_mut().change_direction(direction);
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget(); // Prevents the closure from being deallocated
}

#[wasm_bindgen]
pub fn restart_game() {
    unsafe {
        if let Some(snake) = GAME_STATE.as_ref() {
            snake.borrow_mut().reset();
        } else {
            console::log_1(&"No game state exists to reset".into());
        }
    }
}

fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>, time: u32) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    let snake_clone = snake.clone();
    let canvas_clone = canvas.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        snake_clone.borrow_mut().update();
        snake_clone.borrow().draw(&canvas_clone);

        // Re-fetch the window reference to avoid borrowing issues
        let window = web_sys::window().expect("should have a Window");
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            time as i32,
        ).expect("should register `setTimeout`");
    }) as Box<dyn FnMut()>));

    // Re-fetch the window reference for the same reason as above
    let window = web_sys::window().expect("should have a Window");
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        g.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        time as i32,
    )?;

    Ok(())
}
