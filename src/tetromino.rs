use crate::cell::*;
use crate::grid::*;
use rand::Rng;
use std::time::Duration;
use ggez::mint::Vector2;
use ggez::mint::Point2;
use ggez::Context;
use ggez::graphics::Canvas;

pub struct Tetromino {
    shape: [[u8; 4]; 4],
    cells: [Cell; 4],
    position: Point2<f32>,
    velocity: Duration,
    acceleration: Duration,
}

pub const I_SHAPE: [[u8; 4]; 4] = [
    [0, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 1, 0, 0],
];

pub const O_SHAPE: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 1, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];

pub const T_SHAPE: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 1, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0],
];

pub const J_SHAPE: [[u8; 4]; 4] = [
    [0, 0, 1, 0],
    [0, 0, 1, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];

pub const L_SHAPE: [[u8; 4]; 4] = [
    [0, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];

pub const S_SHAPE: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 1, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0],
];

pub const Z_SHAPE: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 0, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];


impl Tetromino {

    pub fn new(
        _ctx: &mut ggez::Context,
        shape: [[u8; 4]; 4],
        position: Point2<f32>,
        velocity: Duration,
        acceleration: Duration,
    ) -> Self {
        let mut cells: [Cell; 4] = [
            Cell::new(_ctx, Vector2 { x: -20.0, y: -20.0 }),
            Cell::new(_ctx, Vector2 { x: -20.0, y: -20.0 }),
            Cell::new(_ctx, Vector2 { x: -20.0, y: -20.0 }),
            Cell::new(_ctx, Vector2 { x: -20.0, y: -20.0 }),
        ];
        let mut cell_index = 0;

        for row in 0..4 {
            for col in 0..4 {
                if shape[row][col] == 1 && cell_index < 4 {
                    let x = (col as f32 * 20.0) + position.x;
                    let y = (row as f32 * 20.0) + position.y;

                    cells[cell_index] = Cell::new (
                        _ctx,
                        Vector2 { x, y },
                    );

                    cell_index += 1;
                }
            }
        }
        Tetromino {
            shape,
            cells,
            position,
            velocity,
            acceleration,
        }
    }

    pub fn get_random_shape() -> [[u8; 4]; 4] {
        let tetromino_shapes = [I_SHAPE, O_SHAPE, T_SHAPE, J_SHAPE, L_SHAPE, S_SHAPE, Z_SHAPE];
        let random_index = rand::thread_rng().gen_range(0..tetromino_shapes.len());
        let selected_shape = tetromino_shapes[random_index];

        selected_shape
    }

    pub fn move_left(&mut self, grid: &mut Grid) {
        let mut can_move = true;
        for cell in &mut self.cells {
            if cell.can_move_left(grid) == false {
                can_move = false;
            }
        }
        if can_move {
            for cell in &mut self.cells {
                cell.move_left();
            }
            self.position.x -= 20.0;
        }
    }
    pub fn move_right(&mut self, grid: &mut Grid) {
        let mut can_move = true;
        for cell in &mut self.cells {
            if cell.can_move_right(grid) == false {
                can_move = false;
            }
        }
        if can_move {
            for cell in &mut self.cells {
                cell.move_right();
            }
            self.position.x += 20.0;
        }
    }
    pub fn move_down(&mut self, grid: &mut Grid, timer: &mut std::time::Instant, ctx: &mut Context, previous_cells: &mut Vec<Cell>) {
        let mut can_move = true; 
        for cell in &mut self.cells {
            if cell.can_move_down(grid) == false {
                can_move = false;
            }
        }
        if can_move && timer.elapsed() >= self.velocity {
            for cell in &mut self.cells {
                cell.move_down();
            }
            self.position.y += 20.0;
            *timer = std::time::Instant::now();
        }
        if can_move == false && timer.elapsed() >= self.velocity {
            self.reset(ctx, previous_cells, grid);
        }
    }
    pub fn speed_up(&mut self, grid: &mut Grid) {
        let mut can_move = true;
        for cell in &mut self.cells {
            if cell.can_move_down(grid) == false {
                can_move = false;
            }
        }
        if can_move {
            for cell in &mut self.cells {
                cell.speed_up();
            }
            self.position.y += 20.0;
        }
    }

    pub fn draw_tetromino(&mut self, canvas: &mut Canvas) {
        for cell in &mut self.cells {
            cell.draw(canvas).expect("Couldn't draw tetromino");
        }

        let mut cell_index = 0;
        for row in 0..4 {
            for col in 0..4 {
                if self.shape[row][col] == 1 && cell_index < 4 {
                    let x = (col as f32 * 20.0) + self.position.x;
                    let y = (row as f32 * 20.0) + self.position.y;

                    self.cells[cell_index].update_position(x, y);

                    cell_index += 1;
                }
            }
        }
    }

    pub fn reset(&mut self, ctx: &mut Context, previous_cells: &mut Vec<Cell>, grid: &mut Grid) {
        for cell in &self.cells {
            grid.occupy_cell((cell.rect.x / 20.0).floor() as usize, (cell.rect.y / 20.0).floor() as usize);
            previous_cells.push(cell.clone());
        }
        if self.velocity > self.acceleration {
            self.velocity -= self.acceleration;
        }
        *self = Tetromino::new(
            ctx, 
            Tetromino::get_random_shape(), 
            Point2 { x: 265.0, y: 5.0 },
            self.velocity, 
            self.acceleration)
    }

    pub fn rotate_tetromino(&mut self) {
        let mut rotated = [[0; 4]; 4];

        for row in 0..4 {
            for col in 0..4 {
                let new_row = col;
                let new_col = 3 - row;

                rotated[new_row][new_col] = self.shape[row][col];
            }
        }

        self.shape = rotated;

        let current_positions: Vec<(usize, usize)> = self
            .cells
            .iter()
            .map(|cell| {
                let row = (cell.rect.y / 20.0).floor() as usize;
                let col = ((cell.rect.x - 265.0) / 20.0).floor() as usize;
                (row, col)
            })
            .collect();

        for (row, _col) in current_positions.iter().zip(rotated.iter()) {
            for cell_row in 0..4 {
                for cell_col in 0..4 {
                    if rotated[cell_row][cell_col] == 1 {
                        let new_row = row.0 + cell_row;
                        let new_col = row.1 + cell_col;
                        let new_x = (new_col as f32 * 20.0) + 265.0;
                        let new_y = (new_row as f32 * 20.0) + 5.0;

                        let cell_index = new_row * 4 + new_col;
                        if let Some(cell) = self.cells.get_mut(cell_index) {
                            cell.rect.x = new_x;
                            cell.rect.y = new_y;
                        }
                    }
                }
            }
        }
    }
}