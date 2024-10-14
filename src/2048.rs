use rand::Rng;

const BOARD_SIZE: usize = 4;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Board {
    grid: [[u32; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        let mut board = Board { grid: [[0; BOARD_SIZE]; BOARD_SIZE] };
        board.generate_new_tile();
        board.generate_new_tile();
        board
    }

    fn generate_new_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..BOARD_SIZE);
        let col = rng.gen_range(0..BOARD_SIZE);

        if self.grid[row][col] == 0 {
            self.grid[row][col] = if rng.gen() { 2 } else { 4 };
        } else {
            self.generate_new_tile();
        }
    }

    fn move_direction(&mut self, start: usize, end: usize, row_or_col: bool) {
        let mut moved = false;
        let mut stack = Vec::new();

        for i in start..end {
            let mut current = i;
            while current != end && self.grid[row_or_col as usize][current] == 0 {
                self.grid[row_or_col as usize][current] = self.grid[row_or_col as usize][current + (end - start) as isize];
                self.grid[row_or_col as usize][current + (end - start) as isize] = 0;
                current += (end - start) as isize;
                moved = true;
            }

            if current != end && self.grid[row_or_col as usize][current] == self.grid[row_or_col as usize][current + (end - start) as isize] {
                self.grid[row_or_col as usize][current] *= 2;
                self.grid[row_or_col as usize][current + (end - start) as isize] = 0;
                stack.push(current);
                moved = true;
            }
        }

        // Check if any tiles were moved or combined
        if !moved {
            return;
        }

        // Update the stack based on combined tiles
        for i in stack.iter().rev() {
            if i > 0 && self.grid[row_or_col as usize][i - (end - start) as usize] == 0 {
                self.grid[row_or_col as usize][i - (end - start) as usize] = self.grid[row_or_col as usize][i];
                self.grid[row_or_col as usize][i] = 0;
            }
        }
    }

    fn move_tiles(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.move_direction(BOARD_SIZE - 1, 0, false),
            Direction::Down => self.move_direction(0, BOARD_SIZE - 1, false),
            Direction::Left => self.move_direction(BOARD_SIZE - 1, 0, true),
            Direction::Right => self.move_direction(0, BOARD_SIZE - 1, true),
        }
    }

    fn is_over(&self) -> bool {
        // Check if there are any empty tiles
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.grid[row][col] == 0 {
                    return false;
                }
            }
        }

        // Check if there are any adjacent tiles that can be combined
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE - 1 {
                if self.grid[row][col] == self.grid[row][col + 1] {
                    return false;
                }
            }
        }

        for col in 0..BOARD_SIZE {
            for row in 0..BOARD_SIZE - 1 {
                if self.grid[row][col] == self.grid[row + 1][col] {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let mut board = Board::new();

    // Game loop
    while !board.is_over() {
        println!("{}", board);

        // Get user input for movement (implement this)

        // Perform the move
        // ...
        // board.move_tiles(Direction::Up);

        // Check for end
        if is_over() {
            break;
        }
    }

    println!("Game over!");
}
