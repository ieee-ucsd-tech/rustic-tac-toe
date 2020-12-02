use crate::lib::io;

use crate::lib::board::Board;
use crate::lib::player::Player;
use crate::lib::point::Point;

// the game
pub struct Game {
    board: Board,
    active: Player,
    turn: usize,
    winner: Option<Player>,
}

// implementation stuff for the game
impl Game {
    /// Constructs a new game
    pub fn new() -> Self {
        todo!();
    }

    /// Runs the actual game
    /// Specifically, it
    ///     * prints the program intro
    ///     * keeps running turns until a winner is found or a draw has been reached
    ///     * prints the final board
    ///     * prints the winner of the game
    /// 
    pub fn run(&mut self) {
        // print intro
        todo!();

        // keep going until a winner is determined or the board has been fully populated
        let max_turns = self.board.len() * self.board[0].len();
        while todo!() {
            todo!();
        }
        
        // print the final board
        todo!();

        // print the result
        todo!();
    }

    /// Performs everything involved in a single turn of the tic-tac-toe game
    /// Specifically, it
    ///     * prints the state of the board at the start of the current turn
    ///     * takes in input until a valid move is passed in
    ///     * makes the actual move
    /// 
    fn do_turn(&mut self) {
        // if the board is already full there is no valid move
        // and the game should have exited the turn loop already
        let max_turns = self.board.len() * self.board[0].len();
        if self.turn == max_turns {
            unreachable!("Error: trying to do turn when board is full");
        }

        // print the board state
        todo!();

        // get a valid move
        todo!();

        // make the move
        todo!();
    }

    /// Makes active player make move `p`
    /// Then checks if said move `p` was a winning move
    /// or otherwise updates to the next turn
    /// 
    /// # Assumptions:
    ///     # the active player is the player making move `p`
    /// 
    /// # Arguments:
    ///     * `p` : the valid move to make
    /// 
    /// # Postconditions:
    ///     # the board is updated
    ///     # if the move was a winning move, the winner is updated
    ///     # otherwise, the turn and active player is updated
    /// 
    fn make_move(&mut self, p: &Point) {
        todo!();
    }

    /// Checks if point `p` is a winning move
    /// 
    /// # Assumptions:
    ///     * `p` is the move just made
    ///     * `self.active` is the player that just made said last move `p`
    ///     * `check_win` has been called on every previous move as well
    /// 
    /// # Arguments:
    ///     * `p` : the move to check as a possible winning move
    /// 
    /// # Returns:
    ///     * whether `p` is a winning move
    /// 
    fn check_win(&self, p: &Point) -> bool {
        // the function assumes `p` has just been played
        if self.board[p.r][p.c].is_none() {
            unreachable!("Error: tried checking for winning move at {} but cell was empty", p);
        }
        // the function assumes the active player is the one who just played the latest move `p`
        if Some(self.active) != self.board[p.r][p.c] {
            unreachable!("Error: tried checking for winning move at {} but current player differs", p);
        }

        todo!();
    }

    /// Updates the game turn
    /// 
    /// # Postconditions:
    ///     * the turn is updated
    ///     * the active player is updated
    /// 
    fn next_turn(&mut self) {
        todo!();
    }
}

// unit tests
// #[cfg(test)] indicates to only compile and run the code when `cargo test` is run
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_3x3() {
        let game = Game::new();
        assert_eq!(game.board.len(), 3);

        let b: Board = Default::default();
        assert_eq!(game.board, b);
        assert_eq!(game.active, Player::X);
        assert_eq!(game.turn, 0);
        assert_eq!(game.winner, None);
    }

    #[test]
    fn test_make_move_3x3() {
        let mut game = Game::new();
        assert_eq!(game.board.len(), 3);

        game.make_move(&Point{r: 0, c: 0});
        assert_eq!(game.board[0][0], Some(Player::X));
        assert_eq!(game.turn, 1);
        assert_eq!(game.active, Player::O);
    }

    #[test]
    fn test_next_turn_3x3() {
        let mut game = Game::new();
        assert_eq!(game.board.len(), 3);

        game.next_turn();
        assert_eq!(game.turn, 1);
        assert_eq!(game.active, Player::O);
        game.next_turn();
        assert_eq!(game.turn, 2);
        assert_eq!(game.active, Player::X);
        game.next_turn();
        assert_eq!(game.turn, 3);
        assert_eq!(game.active, Player::O);
    }

    // X O O
    // □ X □
    // □ □ X
    #[test]
    fn test_x_wins_3x3() {
        let mut game = Game::new();
        assert_eq!(game.board.len(), 3);
        
        game.make_move(&Point{r: 0, c: 0});
        game.make_move(&Point{r: 0, c: 1});
        game.make_move(&Point{r: 1, c: 1});
        game.make_move(&Point{r: 0, c: 2});
        game.make_move(&Point{r: 2, c: 2});

        assert_eq!(game.winner, Some(Player::X));
    }

    // O X X
    // □ O □
    // X □ O
    #[test]
    fn test_o_wins_3x3() {
        let mut game = Game::new();
        assert_eq!(game.board.len(), 3);
        
        game.make_move(&Point{r: 2, c: 0});
        game.make_move(&Point{r: 0, c: 0});
        game.make_move(&Point{r: 0, c: 1});
        game.make_move(&Point{r: 1, c: 1});
        game.make_move(&Point{r: 0, c: 2});
        game.make_move(&Point{r: 2, c: 2});

        assert_eq!(game.winner, Some(Player::O));
    }

    // X X O
    // O O X
    // X X O
    #[test]
    fn test_draw_3x3() {
        let mut game = Game::new();
        assert_eq!(game.board.len(), 3);
        
        game.make_move(&Point{r: 0, c: 0});
        game.make_move(&Point{r: 1, c: 1});
        game.make_move(&Point{r: 0, c: 1});
        game.make_move(&Point{r: 0, c: 2});
        game.make_move(&Point{r: 2, c: 0});
        game.make_move(&Point{r: 1, c: 0});
        game.make_move(&Point{r: 1, c: 2});
        game.make_move(&Point{r: 2, c: 2});
        game.make_move(&Point{r: 2, c: 1});

        assert_eq!(game.winner, None);
    }


    // X O □
    // X O □
    // X □ □
    #[test]
    fn test_col_0_3x3() {
        let mut game = Game::new();
        assert_eq!(game.board.len(), 3);

        game.make_move(&Point{r: 0, c: 0});
        game.make_move(&Point{r: 0, c: 1});
        game.make_move(&Point{r: 0, c: 1});
        game.make_move(&Point{r: 1, c: 1});
        game.make_move(&Point{r: 0, c: 2});

        assert_eq!(game.winner, Some(Player::X));
    }

    // □ □ □
    // □ □ □
    // □ □ □
    #[test]
    fn test_diag_down_3x3() {
        let mut game = Game::new();
        assert_eq!(game.board.len(), 3);

        game.make_move(&Point{r: 0, c: 0});
        game.make_move(&Point{r: 1, c: 0});
        game.make_move(&Point{r: 2, c: 2});
        game.make_move(&Point{r: 1, c: 2});
        game.make_move(&Point{r: 1, c: 1});

        assert_eq!(game.winner, Some(Player::X));
    }

    // □ □ □
    // □ □ □
    // □ □ □
    #[test]
    fn test_diag_up_3x3() {
        let mut game = Game::new();
        assert_eq!(game.board.len(), 3);

        game.make_move(&Point{r: 0, c: 2});
        game.make_move(&Point{r: 0, c: 0});
        game.make_move(&Point{r: 1, c: 1});
        game.make_move(&Point{r: 0, c: 1});
        game.make_move(&Point{r: 2, c: 0});

        assert_eq!(game.winner, Some(Player::X));
    }
}