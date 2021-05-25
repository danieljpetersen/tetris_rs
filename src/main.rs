use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::thread_rng;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

// ----

const WINDOW_WIDTH: i32 = 500;
const WINDOW_HEIGHT: i32 = 800;
const GRID_WIDTH: u8 = 10;
const GRID_HEIGHT: u8 = 20;
const BLOCK_COUNT: u8 = GRID_WIDTH * GRID_HEIGHT;
const BLOCK_SIZE: u8 = 32;
const TICKS_PER_SECOND: f64 = 0.3;

lazy_static! {
    static ref TETROMINO_TYPES: [Tetromino; 7] = [
        Tetromino::new(0),
        Tetromino::new(1),
        Tetromino::new(2),
        Tetromino::new(3),
        Tetromino::new(4),
        Tetromino::new(5),
        Tetromino::new(6),
    ];
}

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

    // todo - i don't like how updating on the board happens. we have to manually copy three variables each time atm (positions, x_offset, y_offset) error prone
    fn transfer_shape_pattern_to_positions(pattern: &[[u8; 4]; 4], absolute_row_offset: i32, absolute_col_offset: i32) -> Option<[u8; 4]> {
        let mut positions = [0,0,0,0];
        let mut cur_index = 0;
        for (row_index, row) in pattern.iter().enumerate() {
            for (col_index, is_occupied) in row.iter().enumerate() {
                if *is_occupied == 1 {
                    let desired_index =  Board::get_index(row_index as i32 + absolute_row_offset as i32, col_index as i32 + absolute_col_offset as i32);
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
    next_shape: Tetromino,
    current_shape: Tetromino,
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
        let next_shape = Tetromino::new(thread_rng().gen_range(0..7));
        let current_shape = Tetromino::new(thread_rng().gen_range(0..7));

        Self {
            grid,
            current_shape,
            next_shape,
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

    fn get_valid_moves_for_current_shape(&self) -> Vec<[u8; 4]> {
        let mut result = Vec::with_capacity((GRID_WIDTH*3) as usize);// better to reallocate a bit more than we'll probably need once up front

        for rotation_index in 0..4 {
            let mut positions = Tetromino::transfer_shape_pattern_to_positions(&self.current_shape.rotation_patterns[rotation_index], self.current_shape.pattern_top_left_row, self.current_shape.pattern_top_left_col).unwrap();
            let mut x_offset = self.current_shape.pattern_top_left_col;

            // ---- shift shape to first column
            loop {
                let mut should_break = false;

                x_offset -= 1;

                for i in 0..4 {
                    positions[i] -= 1;

                    if self.grid[positions[i] as usize].col == 0 {
                        should_break = true;
                    }
                }

                if should_break {
                    break;
                }
            }

            'outer: loop {
                // ---- evaluate each position in this row, addding to valid_positions the position before we hit something
                {
                    let mut viable_position_found = false;

                    'row_loop: loop {
                        if self.is_occupied(&positions) {
                            break 'row_loop;
                        }

                        viable_position_found = true;

                        let mut should_break = false;
                        for i in 0..4 {
                            positions[i] += GRID_WIDTH;

                            if positions[i] >= BLOCK_COUNT {
                                should_break = true;
                            }
                        }

                        if should_break {
                            break 'row_loop;
                        }
                    }

                    // ----

                    if viable_position_found {
                        for i in 0..4 {
                            positions[i] -= GRID_WIDTH;
                        }

                        result.push(positions);
                    }
                }

                // ---- shift shape over once, breaking from outer loop if we are at the edge
                {
                    x_offset += 1;
                    positions = Tetromino::transfer_shape_pattern_to_positions(&self.current_shape.rotation_patterns[rotation_index], self.current_shape.pattern_top_left_row, x_offset).unwrap_or([255,255,255,255]);
                    if positions[0] == 255 {
                        break 'outer;
                    }
                }
            }
        }

        return result;
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

    fn is_occupied(&self, positions: &[u8; 4]) -> bool {
        for i in 0..4 {
            if self.grid[positions[i] as usize].occupied {
                return true;
            }
        }
        return false;
    }

    fn add_tetromino(&mut self) -> bool { // returns whether game over
        for i in 0..4 {
            let index = self.current_shape.positions[i];
            self.grid[index as usize].occupied = true;
            self.grid[index as usize].tetromino_type = self.current_shape.tetromino_type;
        }

        self.current_shape = self.next_shape;
        self.next_shape = Tetromino::new(thread_rng().gen_range(0..7));

        for i in 0..4 {
            if self.grid[self.current_shape.positions[i] as usize].occupied {
                return true;
            }
        }

        return false;
    }

    fn would_clear_line(&self, positions: &[u8; 4]) -> bool {
        let mut rows_to_check = HashSet::new();
        for i in 0..4 {
            let grid_index = positions[i];
            let row = self.grid[grid_index as usize].row;
            rows_to_check.insert(row);
        }

        for row in rows_to_check.iter() {
            for col in 0..GRID_WIDTH {
                let grid_index = Board::get_index(*row as i32, col as i32).unwrap();
                let occupied = self.grid[grid_index as usize].occupied || positions.contains(&grid_index);

                if !occupied {
                    return false;
                }
            }
        }

        return true;
    }

    fn should_clear_line(&mut self) -> i32 {
        for y in (0..GRID_HEIGHT).rev() {
            let mut answer = true;

            for x in 0..GRID_WIDTH {
                let index = Board::get_index(y as i32, x as i32).unwrap();

                if self.grid[index as usize].occupied != true {
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
            self.grid[index as usize].occupied = false; // clear

            while y >= 1 { // move column down
                y -= 1;

                let previous_index = index;
                index = Board::get_index(y as i32, x as i32).unwrap();

                if self.grid[index as usize].occupied {
                    self.grid[index as usize].occupied = false;
                    self.grid[previous_index as usize].occupied = true;
                    self.grid[previous_index as usize].tetromino_type = self.grid[index as usize].tetromino_type;
                }
            }
        }
    }

    fn clear_lines_if_applicable(&mut self) {
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
}

// ----

trait AppState {
    fn get_board(&self) -> &Board;
    fn update(&mut self, is_step_this_frame: bool, cur_time: &f64);
    fn draw(&self) {
        let mut block_index: u8 = 0;
        let board = self.get_board();
        for block in board.grid.iter() {
            let mut color = Color::new(0.2,0.2,0.2,0.2);
            if block.occupied {
                color =  TETROMINO_TYPES[block.tetromino_type as usize].color;
            }
            if board.is_point_inside_block(mouse_position(), block_index) {
                color = GREEN;
            }

            let (x, y) = board.get_block_position_from_row_col(block.row, block.col);
            draw_rectangle(x, y, BLOCK_SIZE as f32, BLOCK_SIZE as f32, color);

            block_index += 1;
        }

        for &index in board.current_shape.positions.iter() {
            let (x, y) = board.get_block_position_from_index(index);
            let color = TETROMINO_TYPES[board.current_shape.tetromino_type as usize].color;
            draw_rectangle(x, y, BLOCK_SIZE as f32, BLOCK_SIZE as f32, color);
        }
    }
}

// ----

struct AppStatePlayer {
    board: Board,
    input_debounce_timer: f64,
}

impl AppState for AppStatePlayer {
    fn get_board(&self) -> &Board {
        &self.board
    }

    fn update(&mut self, is_step_this_frame: bool, cur_time: &f64) {
        let mut x_offset: i32 = 0;
        let mut y_offset: i32 = 0;

        if is_step_this_frame {
            y_offset += 1;
        }

        let debounce_time = 0.1;
        if cur_time - self.input_debounce_timer > debounce_time {
            if is_key_down(KeyCode::Right) {
                self.input_debounce_timer = get_time();
                x_offset += 1;
            }

            if is_key_down(KeyCode::Left) {
                self.input_debounce_timer = get_time();
                x_offset -= 1;
            }
            if is_key_down(KeyCode::Down) {
                self.input_debounce_timer = get_time();
                y_offset += 1;
            }

            if is_key_pressed(KeyCode::Up) {
                let mut desired_positions = [0, 0, 0, 0];
                let mut collision = false;
                let mut rotation_index = self.board.current_shape.rotation_pattern_index + 1;
                if rotation_index >= 4 {
                    rotation_index = 0;
                }

                let result = Tetromino::transfer_shape_pattern_to_positions(&self.board.current_shape.rotation_patterns[rotation_index as usize], self.board.current_shape.pattern_top_left_row, self.board.current_shape.pattern_top_left_col);

                if result.is_some() {
                    desired_positions = result.unwrap();
                } else {
                    collision = true;
                }

                for i in 0..4 {
                    if self.board.grid[desired_positions[i] as usize].occupied {
                        collision = true;
                        break;
                    }
                }

                if collision != true {
                    self.board.current_shape.positions = desired_positions;
                    self.board.current_shape.rotation_pattern_index = rotation_index;
                }
            }
        }

        if y_offset == 2 { // seems reasonable to limit us to 1 vertical movement per tick. this also fixes bug with collision detection reverting back 2 squares instead of (the correct) 1
            y_offset = 1;
        }

        // segregating into two separate calls so that we can have different behavior for moving left / right and moving vertically (vertically we want to add shape to board on collision)
        self.move_current_shape(x_offset, 0, false);
        self.move_current_shape(0, y_offset, true);
        self.board.clear_lines_if_applicable();
    }
}

impl AppStatePlayer {
    fn new() -> Self {
        let board = Board::new();

        Self {
            board,
            input_debounce_timer: get_time()
        }
    }

    fn move_current_shape(&mut self, x_offset: i32, y_offset: i32, collision_adds_to_board: bool) {
        let mut collision = CollisionType::None;
        let mut desired_positions = [0,0,0,0];
        for i in 0..4 {
            let initial_index = self.board.current_shape.positions[i];
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
            self.board.current_shape.positions = desired_positions;
            self.board.current_shape.pattern_top_left_row += y_offset;
            self.board.current_shape.pattern_top_left_col += x_offset;
        }
        else if collision_adds_to_board {
            if collision == CollisionType::Ground || collision == CollisionType::Block { // wall is fine though i suppose that won't be hit regardless due to the way this is set up

                if self.board.add_tetromino() {
                    self.reset_due_to_game_over();
                }
            }
        }
    }

    fn reset_due_to_game_over(&mut self) {
        self.board = Board::new();
    }
}

// ----

const AI_WEIGHT_TYPE_LINES_CLEARED: usize = 0;
const AI_WEIGHT_TYPE_TOUCHING_WALL_EDGE: usize = 1;
const AI_WEIGHT_TYPE_TOUCHING_BLOCK_EDGE: usize = 2;
const AI_WEIGHT_TYPE_COVERING_HOLE: usize = 3;
const AI_WEIGHT_TYPE_HEIGHT: usize = 4;
const AI_WEIGHT_COUNT: usize = 5;

struct AiWeights {
    weights: [f32; AI_WEIGHT_COUNT]
}

impl AiWeights {
    fn new() -> Self {
        let weights = [0.0, 0.0, 0.0, 0.0, -1.0];

        Self {
            weights
        }
    }

    fn mutate(&mut self) {
        for i in 0..AI_WEIGHT_COUNT {
            self.weights[i] += thread_rng().gen_range(-0.4..0.4);
            println!("{}", self.weights[i]);
        }
    }
}

// #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct AiShapeEvaluation {
    positions: [u8; 4],
    score: f32,
}

struct AiAgent {
    weights: AiWeights,
    board: Board
}

impl AiAgent {
    fn new() -> Self {
        let weights = AiWeights::new();
        let board = Board::new();

        Self {
            weights,
            board
        }
    }

    fn determine_score_lines_cleared(&self, positions: &[u8; 4]) -> f32 {
        if self.board.would_clear_line(&positions) {
            return 1.0 * self.weights.weights[AI_WEIGHT_TYPE_LINES_CLEARED]
        }

        return 0.0
    }

    fn determine_score_touching_walls(&self, grid_index: u8) -> f32 {
        let col = self.board.grid[grid_index as usize].col;
        if col == 0 || col == GRID_WIDTH - 1 {
            return 1.0 * self.weights.weights[AI_WEIGHT_TYPE_TOUCHING_WALL_EDGE];
        }

        return 0.0;
    }

    fn determine_score_block_edge(&self, grid_index: u8) -> f32 {
        let mut edge_count = 0;
        let row = self.board.grid[grid_index as usize].row as i32;
        let col = self.board.grid[grid_index as usize].col as i32;

        let cell_to_left = Board::get_index(row, col + 1);
        let cell_to_right = Board::get_index(row, col - 1);
        let cell_to_bottom = Board::get_index(row + 1, col);

        if cell_to_left.is_some() {
            if self.board.grid[cell_to_left.unwrap() as usize].occupied {
                edge_count += 1;
            }
        }
        if cell_to_right.is_some() {
            if self.board.grid[cell_to_right.unwrap() as usize].occupied {
                edge_count += 1;
            }
        }
        if cell_to_bottom.is_some() {
            if self.board.grid[cell_to_bottom.unwrap() as usize].occupied {
                edge_count += 1;
            }
        }

        return edge_count as f32 * self.weights.weights[AI_WEIGHT_TYPE_TOUCHING_BLOCK_EDGE];
    }

    fn determine_score_covering_holes(&self, grid_index: u8) -> f32 {
        if self.board.grid[grid_index as usize].row != GRID_HEIGHT - 1 {
            if self.board.grid[(grid_index + GRID_WIDTH) as usize].occupied {
                return 1.0 * self.weights.weights[AI_WEIGHT_TYPE_COVERING_HOLE];
            }
        }

        return 0.0;
    }

    fn determine_score_height(&self, grid_index: u8) -> f32 {
        return (self.board.grid[grid_index as usize].row) as f32 * self.weights.weights[AI_WEIGHT_TYPE_HEIGHT];
    }
}

impl AppState for AiAgent {
    fn get_board(&self) -> &Board {
        &self.board
    }

    fn update(&mut self, is_step_this_frame: bool, cur_time: &f64) {
        if is_key_down(KeyCode::Space) {
            return;
        }

        let valid_moves = self.board.get_valid_moves_for_current_shape();
        if valid_moves.len() == 0 {
            return;
        }

        // todo can comment htis out only for debuggin
        let mut sorted_valid_moves: Vec<AiShapeEvaluation> = Vec::new();
        {
            for i in 0..valid_moves.len() {
                sorted_valid_moves.push(AiShapeEvaluation {
                    positions: valid_moves[i as usize],
                    score: 0.0
                })
            }
        }


        let mut cell_scores: HashMap<u8, f32> = HashMap::new();
        let mut best_score = 0.0;
        let mut best_valid_move_index = 0;
        for i in 0..valid_moves.len() {
            let mut score = 0.0;

            for j in 0..4 {
                let grid_index = valid_moves[i][j];

                let result = cell_scores.get(&grid_index);
                if result.is_some() != true {
                    score += self.determine_score_lines_cleared(&valid_moves[i]);
                    score += self.determine_score_touching_walls(grid_index);
                    score += self.determine_score_block_edge(grid_index);
                    score += self.determine_score_covering_holes(grid_index);
                    score += self.determine_score_height(grid_index);

                    cell_scores.insert(grid_index, score);
                }
                else {
                    score += result.unwrap();
                }
            }

            sorted_valid_moves[i as usize].score = score;

            if score > best_score {
                best_score = score;
                best_valid_move_index = i;
            }
        }

        // how the hell do i sort a vec of structs based on a float on the struct?

        // for move_index in 0..sorted_valid_moves.len() {
        //     for i in 0..4 {
        //         let (x, y) = self.board.get_block_position_from_index(sorted_valid_moves[move_index][i]);
        //
        //         let color = RED;
        //
        //         draw_rectangle(x, y, BLOCK_SIZE as f32, BLOCK_SIZE as f32, color);
        //     }
        // }
        //
        // if is_step_this_frame != true {
        //     return;
        // }

        self.board.current_shape.positions = valid_moves[best_valid_move_index as usize];
        self.board.add_tetromino();
    }
}

// ----

// struct AiTournament {
//
// }
//
// impl AiTournament {
//     fn run() {
//
//     }
// }

// ----

struct App {
    next_tick_time: f64,
    frame_count: u64,
    app_state_index: usize,
    app_states: Vec<Box<dyn AppState>>,
}

impl App {
    fn new() -> Self {
        let mut app_states: Vec<Box<dyn AppState>> = Vec::new();
        app_states.push(Box::new(AppStatePlayer::new()));
        app_states.push(Box::new(AiAgent::new()));

        Self {
            next_tick_time: get_time() + TICKS_PER_SECOND,
            frame_count: 1,
            app_state_index: 1,
            app_states: app_states
        }
    }

    // ----

    fn update(&mut self) {
        self.frame_count += 1;

        let mut is_step_this_frame = false;

        let cur_time = get_time();
        if cur_time > self.next_tick_time {
            self.next_tick_time += TICKS_PER_SECOND;
            is_step_this_frame = true;
        }

        self.app_states[self.app_state_index].update(is_step_this_frame, &cur_time);
    }

    // ----

    fn draw(&mut self) {
        self.app_states[self.app_state_index].draw();
    }
}
