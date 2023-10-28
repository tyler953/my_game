mod cell;
use crate::cell::*;
mod grid;
use crate::grid::*;
mod tetromino;
use crate::tetromino::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;
use ggez::input::keyboard::KeyCode;
use std::time::Duration;
use ggez::conf::WindowMode;



fn main() {
    // Define the window mode.
    let window_mode = WindowMode {
        width: 610.0,
        height: 610.0,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None,
    };
    
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("MyGame", "Tyler")
        .window_setup(ggez::conf::WindowSetup::default().title("MyGame"))
        .window_mode(window_mode)
        .build()
        .expect("Something failed with the contex.");

    // Create an instance of your event handler.
    let my_game = MainState::new(&mut ctx);

    // Run
    event::run(ctx, event_loop, my_game);
}

struct MainState {
    // Establish variables
    previous_cells: Vec<Cell>,
    current_tetromino: Tetromino,
    timer: std::time::Instant,
    grid: Grid,
}

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        MainState {
            // create an instance of everything needed
            // a timer to control when it moves
            timer: std::time::Instant::now(),
            // a vector to hold all cells that have been moved
            previous_cells: Vec::new(),
            // a grid to display the board and track locations for collision detection
            grid: Grid::new(
                ctx,
                30,
                30,
            ),
            current_tetromino: Tetromino::new(
                ctx,
                Tetromino::get_random_shape(),
                Point2 { x: 265.0, y: 5.0 },
                Duration::from_millis(500),
                Duration::from_millis(5),
            ),
        }
    }
}



impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let k_ctx = &ctx.keyboard;
        // handle keyboard inputs (left, right, and down)
        if k_ctx.is_key_just_pressed(KeyCode::Right) {
            self.current_tetromino.move_right(&mut self.grid);
        } else if k_ctx.is_key_just_pressed(KeyCode::Left) {
            self.current_tetromino.move_left(&mut self.grid);
        } else if k_ctx.is_key_pressed(KeyCode::Down) {
            self.current_tetromino.speed_up(&mut self.grid);
        } else if k_ctx.is_key_just_pressed(KeyCode::Up) {
            self.current_tetromino.rotate_tetromino();
        }

        // automatically move the current cell down when the time runs out
        self.current_tetromino.move_down(&mut self.grid, &mut self.timer, ctx, &mut self.previous_cells);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        
        // draw the grid, the current cell, and all previous cells onto the canvas
        self.grid.draw(ctx, &mut canvas).expect("Couldn't draw grid");
        self.current_tetromino.draw_tetromino(&mut canvas);
        for cell in &self.previous_cells {
            cell.draw_previous(&mut canvas);
        }
        
        canvas.finish(ctx)
    }
}
