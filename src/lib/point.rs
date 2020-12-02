use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub r: usize,
    pub c: usize,
}

// implement the FromStr trait for our Point struct
impl FromStr for Point {
    type Err = ParseIntError;

    // define the `from_str` method for our Point struct;
    // `&str.parse::<T>()` invokes `T.from_str`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords: Vec<&str> = s.trim().split(" ").collect::<Vec<_>>();
        coords.push("dummy");   // probably not the best practice but i can't figure out
                                // how to properly return if the input parses to only one number oops

        let r = coords[0].parse::<usize>()?;
        let c = coords[1].parse::<usize>()?;

        Ok(Point{r: r, c: c})
    }
}

// implement the Display trait for our Point struct
// doing so is the idiomatic way to implement `ToString`,
// since having `Display` implemented will automatically result in `ToString` implemented
impl fmt::Display for Point {
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
    fn test_to_string() {
        let p = Point{r: 0, c: 0};
        assert_eq!(p.to_string(), "(0,0)");
    }

    #[test]
    fn test_parse_nan() {
        let input = "ieee";
        let result = input.parse::<Point>();
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_parse_one() {
        let input = "0";
        let result = input.parse::<Point>();
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_parse_neg() {
        let input = "-1 1";
        let result = input.parse::<Point>();
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_parse_good() -> Result<(), ParseIntError> {
        let input = "1333 030";
        let result = input.parse::<Point>();
        assert_eq!(result?, Point{r: 1333, c: 030});
        Ok(())
    }

    #[test]
    fn test_parse_trim() -> Result<(), ParseIntError> {
        let input = "      0 0                                ";
        let result = input.parse::<Point>();
        assert_eq!(result?, Point{r: 0, c: 0});
        Ok(())
    }
}
