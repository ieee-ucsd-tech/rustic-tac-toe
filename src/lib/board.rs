use crate::lib::player::Player;

const SIZE: usize = 3; // tic tac toe is 3x3, but we can change this!

// a SIZE x SIZE board
// Option is used to be able to represent None (unoccupied cell) as well
// do note that `type` only creates an alias for an existing type
pub type Board = [[Option<Player>; SIZE]; SIZE];