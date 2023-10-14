use crate::grid::*;
use ggez::graphics;
use ggez::graphics::Canvas;
use ggez::Context;
use ggez::mint::Vector2;
use std::time::Duration;

#[derive(Clone)]
pub(super) struct Cell {
    rect: graphics::Rect,
    velocity: Duration,
    acceleration: Duration,
}

impl Cell {
    pub fn new(
        _ctx: &mut ggez::Context, 
        position: Vector2<f32>,
        velocity: Duration,
        acceleration: Duration,
    ) -> Self {
        Cell{
            rect: graphics::Rect::new(position.x, position.y, 20.0, 20.0),
            velocity,
            acceleration,
        }
    }

    // move the block down an extra space when the user presses down if possible
    pub fn speed_up(&mut self, distance: f32, grid: &mut Grid) {
        if self.can_move_down(grid) {
            self.rect.y += distance;
        }
    }
    
    // automatically move the block down after a certain amount of time has passed
    pub fn move_down(&mut self, grid: &mut Grid, timer: &mut std::time::Instant, ctx: &mut Context, previous_cells: &mut Vec<Cell>) {
        if timer.elapsed() >= self.velocity {
            if self.can_move_down(grid) {
                self.rect.y += 20.0;
                *timer = std::time::Instant::now();
            } else {
                self.reset(ctx, previous_cells, grid);
            }
        }
    }

    // move the block left if possible after the user presses the left key
    pub fn move_left(&mut self, distance: f32, grid: &mut Grid)  {
        if self.can_move_left(grid) {
            self.rect.x -= distance;
        }
    }

    // move the block right if possible after the user presses the right key
    pub fn move_right(&mut self, distance: f32, grid: &mut Grid)  {
        if self.can_move_right(grid) {
            self.rect.x += distance;
        }
    }

    // check if the block can move down
    fn can_move_down(&mut self, grid: &mut Grid) -> bool {
        if self.rect.y == 585.0 {
            return false;
        }
        if grid.is_occupied((self.rect.x / 20.0).floor() as usize, ((self.rect.y / 20.0) + 1.0).floor() as usize) {
            return false;
        }
        true
    }

    // check if the block can move right
    fn can_move_right(&mut self, grid: &mut Grid) -> bool {
        if self.rect.x == 585.0 {
            return false;
        }
        if grid.is_occupied(((self.rect.x / 20.0) + 1.0).floor() as usize, (self.rect.y / 20.0).floor() as usize) {
            return false;
        }
        true
    }

    // check if the block can move left
    fn can_move_left(&mut self, grid: &mut Grid) -> bool {
        if self.rect.x == 5.0 {
            return false;
        }
        if grid.is_occupied(((self.rect.x / 20.0) - 1.0).floor() as usize, (self.rect.y / 20.0).floor() as usize) {
            return false;
        }
        true
    }

    // Draw the block onto the canvas
    pub fn draw(&self, canvas: &mut Canvas) -> ggez::GameResult<()> {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(self.rect.point())
                .scale(self.rect.size())
                .color(graphics::Color::WHITE)
        );
        Ok(())
    }

    // move the current cell into the previous_cells vector and create a new instance 
    // of the current cell at the top of the window with a higher velocity
    pub fn reset(&mut self, ctx: &mut Context, previous_blocks: &mut Vec<Cell>, grid: &mut Grid) {
        
        grid.occupy_cell((self.rect.x / 20.0).floor() as usize, (self.rect.y / 20.0).floor() as usize);

        previous_blocks.push(self.clone());
        if self.velocity > self.acceleration {
            self.velocity -= self.acceleration;
        }
        *self = Cell::new(
            ctx,
            Vector2 {
                x: self.rect.x,
                y: 5.0,
            },
            self.velocity,
            self.acceleration,
        );
        
    }
}
