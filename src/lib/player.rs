use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    // returns the initial player
    pub fn initial() -> Player {
        todo!();
    }

    // returns the next player
    pub fn next(&self) -> Player {
        todo!();
    }
}

// implement the Display trait for our Player enum variants
// doing so is the idiomatic way to implement `ToString`,
// since having `Display` implemented will automatically result in `ToString` implemented
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!();
    }
}

// unit tests
// #[cfg(test)] indicates to only compile and run the code when `cargo test` is run
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial() {
        assert_eq!(Player::initial(), Player::X);
    }

    #[test]
    fn test_next() {
        assert_eq!(Player::X.next(), Player::O);
        assert_eq!(Player::O.next(), Player::X);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Player::X.to_string(), "X");
        assert_eq!(Player::O.to_string(), "O");
    }
}