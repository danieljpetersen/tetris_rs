use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;

// ----

const WINDOW_WIDTH: i32 = 500;
const WINDOW_HEIGHT: i32 = 800;
const GRID_WIDTH: u8 = 10;
const GRID_HEIGHT: u8 = 20;
const BLOCK_SIZE: u8 = 32;
const TICKS_PER_SECOND: f64 = 1.0;

// ----

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris.rs".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new();

    loop {
        clear_background(Color::new(0.10, 0.10, 0.10, 1.0));

        app.update();
        app.draw();

        next_frame().await
    }
}

// ----

#[derive(Clone)]
struct Block {
    index: u8,
    tetromino_type: u8, // for determining draw color
    occupied: bool,
    col: u8,
    row: u8,
}

impl Block {
    fn new() -> Self {
        Self {
            index: 0,
            tetromino_type: 0,
            occupied: false,
            col: 0,
            row: 0
        }
    }
}

// ----

#[derive(PartialEq, Eq)]
enum CollisionType {
    None,
    Wall,
    Ground,
    Block
}

// ----

#[derive(Copy, Clone)]
struct Tetromino {
    tetromino_type: u8,
    positions: [u8; 4],
    rotation_patterns: [[[u8; 4]; 4]; 4],
    rotation_pattern_index: u8,
    pattern_top_left_row: i32,
    pattern_top_left_col: i32,
    color: Color,
}

impl Tetromino {
    fn new(tetromino_type: u8) -> Self {
        let color;
        let positions;
        let rotation_patterns;

        match tetromino_type {
            0 => { // I_SHAPE
                rotation_patterns = [
                    [
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                    ],

                    [
                        [0, 0, 0, 0],
                        [1, 1, 1, 1],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                    ],

                    [
                        [0, 0, 0, 0],
                        [1, 1, 1, 1],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],
                ];

                color = PINK;
            },

            1 => { // J_SHAPE
                rotation_patterns = [
                    [
                        [0, 0, 1, 0],
                        [0, 0, 1, 0],
                        [0, 1, 1, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 0, 0],
                        [0, 1, 1, 1],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 0, 1, 1],
                        [0, 0, 1, 0],
                        [0, 0, 1, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 0, 0, 0],
                        [0, 1, 1, 1],
                        [0, 0, 0, 1],
                        [0, 0, 0, 0],
                    ],
                ];

                color = YELLOW;
            },

            2 => { // L_SHAPE
                rotation_patterns = [
                    [
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 1, 1, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 0, 0, 0],
                        [1, 1, 1, 0],
                        [1, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [1, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 0, 1, 0],
                        [1, 1, 1, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],
                ];

                color = GREEN;
            },

            3 => { // O_SHAPE
                rotation_patterns = [
                    [
                        [0, 1, 1, 0],
                        [0, 1, 1, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 1, 0],
                        [0, 1, 1, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 1, 0],
                        [0, 1, 1, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 1, 0],
                        [0, 1, 1, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],
                ];

                color = BLUE;
            },

            4 => { // S_SHAPE
                rotation_patterns = [
                    [
                        [0, 1, 1, 0],
                        [1, 1, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 0, 0],
                        [0, 1, 1, 0],
                        [0, 0, 1, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 1, 0],
                        [1, 1, 0, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 0, 0],
                        [0, 1, 1, 0],
                        [0, 0, 1, 0],
                        [0, 0, 0, 0],
                    ],
                ];

                color = DARKPURPLE;
            },

            5 => { // Z_SHAPE
                rotation_patterns = [
                    [
                        [0, 1, 1, 0],
                        [0, 0, 1, 1],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 0, 0, 1],
                        [0, 0, 1, 1],
                        [0, 0, 1, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 1, 0],
                        [0, 0, 1, 1],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 0, 0, 1],
                        [0, 0, 1, 1],
                        [0, 0, 1, 0],
                        [0, 0, 0, 0],
                    ],
                ];

                color = ORANGE;
            },

            6 => { // T_SHAPE
                rotation_patterns = [
                    [
                        [0, 1, 0, 0],
                        [1, 1, 1, 0],
                        [0, 0, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 0, 0],
                        [0, 1, 1, 0],
                        [0, 1, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 0, 0, 0],
                        [1, 1, 1, 0],
                        [0, 1, 0, 0],
                        [0, 0, 0, 0],
                    ],

                    [
                        [0, 1, 0, 0],
                        [1, 1, 0, 0],
                        [0, 1, 0, 0],
                        [0, 0, 0, 0],
                    ],
                ];

                color = BEIGE;
            },

            _ => {
                panic!("invalid input to tetromino new, {}", tetromino_type);
            }
        }

        let pattern_top_left_row = 0 as i32;
        let pattern_top_left_col = (GRID_WIDTH/2 - 2) as i32;
        positions = Tetromino::transfer_shape_pattern_to_positions(&rotation_patterns[0], pattern_top_left_row, pattern_top_left_col).unwrap();

        return Self {
            tetromino_type,
            positions,
            rotation_patterns,
            pattern_top_left_row,
            pattern_top_left_col,
            rotation_pattern_index: 0,
            color,
        }
    }

    fn transfer_shape_pattern_to_positions(pattern: &[[u8; 4]; 4], row_offset: i32, col_offset: i32) -> Option<[u8; 4]> {
        let mut positions = [0,0,0,0];
        let mut cur_index = 0;
        for (row_index, row) in pattern.iter().enumerate() {
            for (col_index, is_occupied) in row.iter().enumerate() {
                if *is_occupied == 1 {
                    let desired_index =  Board::get_index(row_index as i32 + row_offset as i32, col_index as i32 + col_offset as i32);
                    if desired_index.is_some() {
                        positions[cur_index] = desired_index.unwrap() as u8;
                        cur_index += 1;
                    }
                    else {
                        return None;
                    }
                }
            }
        }

        return Some(positions);
    }
}

// ----

struct Board {
    grid: Vec<Block>,
    x_start: f32,
    y_start: f32,
}

impl Board {
    fn new() -> Self {
        let mut grid = vec![Block::new(); (GRID_WIDTH*GRID_HEIGHT) as usize];

        let mut i = 0;
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                grid[i].col = x as u8;
                grid[i].row = y as u8;
                grid[i].occupied = false;
                grid[i].index = i as u8;

                i += 1;
            }
        }

        Self {
            grid,
            x_start: (WINDOW_WIDTH as f32 / 2.0 - (BLOCK_SIZE as f32 *GRID_WIDTH as f32)/2.0) as f32,
            y_start: WINDOW_HEIGHT as f32 / 2.0 - (BLOCK_SIZE as f32 *GRID_HEIGHT as f32)/2.0,
        }
    }

    fn get_index(row: i32, col: i32) -> Option<u8> {
        if row >= 0 {
            if col >= 0 {
                if row < GRID_HEIGHT as i32 {
                    if col < GRID_WIDTH as i32 {
                        return Some((GRID_WIDTH * row as u8 + col as u8) as u8)
                    }
                }
            }
        }

        return None;
    }

    fn get_block_position_from_index(&self, index: u8) -> (f32, f32) {
        self.get_block_position_from_row_col(self.grid[index as usize].row, self.grid[index as usize].col)
    }

    fn get_block_position_from_row_col(&self, row: u8, col: u8) -> (f32, f32) {
        let x = self.x_start as f32 + col as f32 * BLOCK_SIZE as f32;
        let y = self.y_start as f32 + row as f32 * BLOCK_SIZE as f32;
        (x, y)
    }

    fn is_point_inside_block(&self, (x_point, y_point): (f32, f32), index: u8) -> bool {
        let (x, y) = self.get_block_position_from_row_col(self.grid[index as usize].row, self.grid[index as usize].col);

        if x_point > x {
            if y_point > y {
                if x_point < x + BLOCK_SIZE as f32 {
                    if y_point < y + BLOCK_SIZE as f32 {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn add_tetromino(&mut self, tetromino: &Tetromino) {
        for i in 0..4 {
            let index = tetromino.positions[i];
            self.grid[index as usize].occupied = true;
            self.grid[index as usize].tetromino_type = tetromino.tetromino_type;
        }
    }
}

// ----

struct App {
    board: Board,
    next_shape: Tetromino,
    current_shape: Tetromino,
    tetromino_types: [Tetromino; 7],
    next_tick_time: f64,
    input_debounce_timer: f64,
    frame_count: u64
}

impl App {
    fn new() -> Self {
        let board = Board::new();
        let tetromino_types = [
                Tetromino::new(0),
                Tetromino::new(1),
                Tetromino::new(2),
                Tetromino::new(3),
                Tetromino::new(4),
                Tetromino::new(5),
                Tetromino::new(6)
            ];
        let next_shape = Tetromino::new(thread_rng().gen_range(0..7));
        let current_shape = Tetromino::new(thread_rng().gen_range(0..7));

        Self {
            board,
            tetromino_types,
            next_shape,
            current_shape,
            next_tick_time: get_time() + TICKS_PER_SECOND,
            input_debounce_timer: get_time(),
            frame_count: 1
        }
    }

    // ----

    fn update(&mut self) {
        self.frame_count += 1;

        let mut x_offset: i32 = 0;
        let mut y_offset: i32 = 0;

        let cur_time = get_time();
        if cur_time > self.next_tick_time {
            self.next_tick_time += TICKS_PER_SECOND;
            y_offset += 1;
        }

        let mut reset_debounce = false;
        let debounce_time = 0.1;

        if is_key_down(KeyCode::Right) {
            if cur_time - self.input_debounce_timer > debounce_time {
                reset_debounce = true;
                x_offset += 1;
            }
        }
        if is_key_down(KeyCode::Left) {
            if cur_time - self.input_debounce_timer > debounce_time {
                reset_debounce = true;
                x_offset -= 1;
            }
        }
        if is_key_down(KeyCode::Down) {
            if cur_time - self.input_debounce_timer > debounce_time {
                reset_debounce = true;
                y_offset += 1;
            }
        }

        if is_key_pressed(KeyCode::Up) {
            let mut desired_positions = [0,0,0,0];
            let mut collision = false;
            let mut rotation_index = self.current_shape.rotation_pattern_index + 1;
            if rotation_index >= 4 {
                rotation_index = 0;
            }

            let result = Tetromino::transfer_shape_pattern_to_positions(&self.current_shape.rotation_patterns[rotation_index as usize], self.current_shape.pattern_top_left_row, self.current_shape.pattern_top_left_col);

            if result.is_some() {
                desired_positions = result.unwrap();
            }
            else {
                collision = true; // terrible
            }

            for i in 0..4 {
                if self.board.grid[desired_positions[i] as usize].occupied {
                    collision = true;
                    break;
                }
            }

            if collision != true {
                self.current_shape.positions = desired_positions;
                self.current_shape.rotation_pattern_index = rotation_index;
            }
        }

        if y_offset == 2 { // seems reasonable to limit us to 1 vertical movement per tick. this also fixes bug with collision detection reverting back 2 squares instead of (the correct) 1
            y_offset = 1;
        }

        if reset_debounce {
            self.input_debounce_timer = get_time();
        }

        // segregating into two separate calls so that we can have different behavior for moving left / right and moving vertically (vertically we want to add shape to board on collision)
        self.move_current_shape(x_offset, 0, false);
        self.move_current_shape(0, y_offset, true);

        loop {
            let row = self.should_clear_line();
            if row != -1 {
                self.clear_line(row as u8);
            }
            else {
                break;
            }
        }
    }

    // ----

    fn move_current_shape(&mut self, x_offset: i32, y_offset: i32, collision_adds_to_board: bool) {
        let mut collision = CollisionType::None;
        let mut desired_positions = [0,0,0,0];
        for i in 0..4 {
            let initial_index = self.current_shape.positions[i];
            let initial_row = self.board.grid[initial_index as usize].row;
            let initial_col = self.board.grid[initial_index as usize].col;
            let desired_index = Board::get_index((initial_row as i32 + y_offset) as i32, (initial_col as i32 + x_offset) as i32);

            if desired_index.is_some() {
                desired_positions[i] = desired_index.unwrap();
            }
            else {
                if initial_row as i32 + y_offset >= GRID_HEIGHT as i32 {
                    collision = CollisionType::Ground;
                }
                else {
                    collision = CollisionType::Wall;
                }

                break;
            }
        }

        if collision == CollisionType::None {
            for i in 0..4 {
                let desired_index = desired_positions[i];
                if self.board.grid[desired_index as usize].occupied {
                    collision = CollisionType::Block;
                    break;
                }
            }
        }

        if collision == CollisionType::None {
            self.current_shape.positions = desired_positions;
            self.current_shape.pattern_top_left_row += y_offset;
            self.current_shape.pattern_top_left_col += x_offset;
        }
        else if collision_adds_to_board {
            if collision == CollisionType::Ground || collision == CollisionType::Block { // wall is fine though i suppose that won't be hit regardless due to the way this is set up
                self.board.add_tetromino(&self.current_shape);
                self.current_shape = self.next_shape;
                self.next_shape = Tetromino::new(thread_rng().gen_range(0..7));

                for i in 0..4 {
                    if self.board.grid[self.current_shape.positions[i] as usize].occupied {
                        self.reset_due_to_game_over();
                    }
                }
            }
        }
    }

    fn should_clear_line(&mut self) -> i32 {
        for y in (0..GRID_HEIGHT).rev() {
            let mut answer = true;

            for x in 0..GRID_WIDTH {
                let index = Board::get_index(y as i32, x as i32).unwrap();

                if self.board.grid[index as usize].occupied != true {
                    answer = false;
                    break;
                }
            }

            if answer {
                return y as i32;
            }
        }

        return -1;
    }

    fn clear_line(&mut self, row: u8) {
        for x in 0..GRID_WIDTH {
            let mut y = row;
            let mut index = Board::get_index(y as i32, x as i32).unwrap();
            self.board.grid[index as usize].occupied = false; // clear

            while y >= 1 { // move column down
                y -= 1;

                let previous_index = index;
                index = Board::get_index(y as i32, x as i32).unwrap();

                if self.board.grid[index as usize].occupied {
                    self.board.grid[index as usize].occupied = false;
                    self.board.grid[previous_index as usize].occupied = true;
                    self.board.grid[previous_index as usize].tetromino_type = self.board.grid[index as usize].tetromino_type;
                }
            }

        }
    }

    fn reset_due_to_game_over(&mut self) {
        self.board = Board::new();
    }

    // ----

    fn draw(&mut self) {
        let mut block_index: u8 = 0;
        for block in self.board.grid.iter() {
            let mut color = WHITE;
            if block.occupied {
                color =  self.tetromino_types[block.tetromino_type as usize].color;
            }
            if self.board.is_point_inside_block(mouse_position(), block_index) {
                color = GREEN;
            }

            let (x, y) = self.board.get_block_position_from_row_col(block.row, block.col);
            draw_rectangle(x, y, BLOCK_SIZE as f32, BLOCK_SIZE as f32, color);

            block_index += 1;
        }

        for &index in self.current_shape.positions.iter() {
            // board.grid[index as usize].occupied = true;
            let (x, y) = self.board.get_block_position_from_index(index);
            let color =  self.tetromino_types[self.current_shape.tetromino_type as usize].color;
            draw_rectangle(x, y, BLOCK_SIZE as f32, BLOCK_SIZE as f32, color);
        }
    }
}