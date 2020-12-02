use std::io;

use crate::lib::errors::MoveError;
use crate::lib::board::Board;
use crate::lib::player::Player;
use crate::lib::point::Point;

// prints the introduction to the game
pub fn print_intro() {
    println!("Welcome to tic-tac-toe!");
    println!("Moves will be entered in the ROW[space]COLUMN format");
    println!("For example, the top center cell would be entered as");
    println!("0 1");
    println!();
    println!("Have fun!");
}

// prints the state of a board
pub fn print_board(board: Board) {
    // helper function for getting the string length of an object that implements the ToString trait
    fn strlen<T>(i: T) -> usize
            where T: std::string::ToString {
        i.to_string().len()
    }

    // get the width needed for each unit cell
    let label_width = strlen(board.len() - 1);

    // helper function to format unit cells by center padding their contents
    fn pad<T>(i: T, n: usize) -> String 
            where T: std::fmt::Display + Copy {
        format!("{:^width$}", i, width = n)
    }

    // print the column labels
    print!(" {}", pad("", label_width));
    for i in 0..board.len() {
        print!(" {}", pad(i, label_width));
    }
    println!();

    // print the rows of the board
    for (i, r) in board.iter().enumerate() {
        // print the row label, right aligned
        print!(" {}", format!("{:>w$}", i, w = label_width));

        // print the values of the row
        for cell in r {
            // either the player's ToString representation,
            // or an open white box to indicate emptiness
            let c = match *cell {
                Some(player) => player.to_string(),
                None => String::from("\u{25a1}"),
            };
            // &*String coerces the string into the &str type
            print!(" {}", pad(&*c, label_width));
        }
        // begin printing on next line
        println!();
    }
    // buffer newline
    println!();
}

/// Prompts `player` for a move until a valid move for the current board state is read
/// 
/// # Arguments:
///     * `player` : the player whose turn it is
///     * `board` : the game board
/// 
/// # Returns:
///     * `Point` : the valid move entered
/// 
pub fn read_valid_move(player: Player, board: Board) -> Point {
    loop {
        // ask for a move
        prompt_move(player);

        // read in the target cell for their move
        let line = read_line();

        // if there's an error reading the line, print the error and continue looping
        // otherwise, we have a valid move so we can exit the loop and return the point
        match parse_point(line, board) {
            Ok(p) => break p,
            Err(e) => println!("{}", e),
        };
    }
}

// prints the winner of the tic-tac-toe game
pub fn print_result(result: Option<Player>) {
    match result {
        Some(player) => println!("{} wins!", player),
        None => println!("Draw.")
    }
}

fn parse_point(string: String, board: Board) -> Result<Point, MoveError> {
    // try parsing the string into a Point
    let p = string.parse::<Point>()?;

    // check for OOB error
    if p.r >= board.len() || p.c >= board[0].len() {
        return Err(MoveError::OOB);
    }

    // check board is already occupied there
    if board[p.r][p.c].is_some() {
        return Err(MoveError::Occupied);
    }

    Ok(p)
}

// prompts `player` to move
fn prompt_move(player: Player) {
    println!("{} to move:", match player {
        Player::X => "X",
        Player::O => "O",
    });
}

// standard function for reading input from stdin in Rust
// in this case we're also stripping leading and trailing whitespace with `trim()`
fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Error reading input");

    line.trim().to_string()
}