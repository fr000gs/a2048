//use a2048;
//mod a2048;
pub mod game;
use game::game as gamez;

fn main() {
    let mut board = gamez::Board::new();

    loop {
        println!("{:?}", board);

        println!("Enter direction (w, a, s, d):");
        let input = get_input();

        let direction = match input {
            'w' => gamez::Direction::Up,
            'a' => gamez::Direction::Left,
            's' => gamez::Direction::Down,
            'd' => gamez::Direction::Right,
            _ => continue,
        };

        board.move_tiles(direction);

        if board.is_over() {
            println!("Game over!");
            break;
        }
    }
}

fn get_input() -> char {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().chars().next().unwrap_or('q')
}
