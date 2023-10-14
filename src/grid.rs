use ggez::graphics;
use ggez::graphics::Canvas;
use ggez::Context;
use ggez::mint::Point2;

pub struct Grid {
    grid: Vec<Vec<bool>>,
    grid_width: usize,
    grid_height: usize,
}

impl Grid {
    pub fn new(
        _ctx: &mut ggez::Context,
        width: usize,
        height: usize,
    ) -> Self {
        // a vector of vectors to represent every possible space a block could be in to handle collision detection
        let grid = vec![vec![false; width]; height];
        Grid {
            grid,
            grid_width: width,
            grid_height: height,
        }
    }

    // Draw the grid onto the canvas
    pub fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas) -> ggez::GameResult<()> {
        for (row_index, row) in self.grid.iter().enumerate() {
            for (cell_index, _cell) in row.iter().enumerate() {
                let rect = graphics::Rect::new((row_index as f32) * 20.0 + 5.0, (cell_index as f32) * 20.0 + 5.0, 0.7, 20.0);
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                    .dest(Point2 {
                        x: rect.x, 
                        y: rect.y 
                    })
                    .scale(rect.size())
                    .color(graphics::Color::BLUE)
                    );
                    let rect = graphics::Rect::new((row_index as f32) * 20.0 + 5.0, (cell_index as f32) * 20.0 + 5.0, 20.0, 0.7);
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                    .dest(Point2 {
                        x: rect.x, 
                        y: rect.y 
                    })
                    .scale(rect.size())
                    .color(graphics::Color::BLUE)
                    );
                }
            };
            
            let rect = graphics::Rect::new((self.grid_width * 20) as f32 + 5.0, (self.grid_height * 20) as f32 + 5.0, -600.0, 0.7);
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                .dest(Point2 {
                    x: rect.x, 
                    y: rect.y 
                })
                .scale(rect.size())
                .color(graphics::Color::BLUE)
            );
            let rect = graphics::Rect::new((self.grid_width * 20) as f32 + 5.0, (self.grid_height * 20) as f32 + 5.0, 0.7, -600.0);
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                .dest(Point2 {
                    x: rect.x, 
                    y: rect.y 
                })
                .scale(rect.size())
                .color(graphics::Color::BLUE)
            );

        Ok(())
    }

    // check if a given cell is already occupied
    pub fn is_occupied(&mut self, x: usize, y: usize) -> bool {
        self.grid[x][y]
    }

    // mark that a given cell is occupied
    pub fn occupy_cell(&mut self, x: usize, y: usize) {
        self.grid[x][y] = true;
    }
}