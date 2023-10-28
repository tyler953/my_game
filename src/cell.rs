use crate::grid::*;
use ggez::graphics;
use ggez::graphics::Canvas;
use ggez::mint::Vector2;
use ggez::mint::Point2;

#[derive(Clone)]
pub struct Cell {
    pub rect: graphics::Rect,
    _position: Vector2<f32>,
}

impl Cell {
    pub fn new(
        _ctx: &mut ggez::Context, 
        _position: Vector2<f32>,
        // velocity: Duration,
        // acceleration: Duration,
    ) -> Self {
        Cell {
            rect: graphics::Rect::new(_position.x, _position.y, 20.0, 20.0),
            _position,
        }
    }

    // move the block down an extra space when the user presses down if possible
    pub fn speed_up(&mut self) {
        self.rect.y += 20.0;
    }
    
    // automatically move the block down after a certain amount of time has passed
    pub fn move_down(&mut self) {
        self.rect.y += 20.0;
    }

    // move the block left if possible after the user presses the left key
    pub fn move_left(&mut self)  {
        self.rect.x -= 20.0;
    }

    // move the block right if possible after the user presses the right key
    pub fn move_right(&mut self)  {
            self.rect.x += 20.0;
    }

    // check if the block can move down
    pub fn can_move_down(&mut self, grid: &mut Grid) -> bool {
        if self.rect.y == 585.0 {
            return false;
        }
        if grid.is_occupied((self.rect.x / 20.0).floor() as usize, ((self.rect.y / 20.0) + 1.0).floor() as usize) {
            return false;
        }
        true
    }

    // check if the block can move right
    pub fn can_move_right(&mut self, grid: &mut Grid) -> bool {
        if self.rect.x == 585.0 {
            return false;
        }
        if grid.is_occupied(((self.rect.x / 20.0) + 1.0).floor() as usize, (self.rect.y / 20.0).floor() as usize) {
            return false;
        }
        true
    }

    // check if the block can move left
    pub fn can_move_left(&mut self, grid: &mut Grid) -> bool {
        if self.rect.x == 5.0 {
            return false;
        }
        if grid.is_occupied(((self.rect.x / 20.0) - 1.0).floor() as usize, (self.rect.y / 20.0).floor() as usize) {
            return false;
        }
        true
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
    }

    // Draw the block onto the canvas
    pub fn draw(&self, canvas: &mut Canvas) -> ggez::GameResult<()> {
        let cell_position = Point2 {
            x: self.rect.x,
            y: self.rect.y,
        };
        
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(cell_position)
                .scale(self.rect.size())
                .color(graphics::Color::WHITE)
        );
        Ok(())
    }

    pub fn draw_previous(&self, canvas: &mut Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(self.rect.point())
                .scale(self.rect.size())
                .color(graphics::Color::WHITE)
        );
    }
}
