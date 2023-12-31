use std::fs;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn from_str(val: &str) -> Move {
        match val {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            _ => Move::Scissor,
        }
    }

    fn score(&self, other: &Move) -> usize {
        match self {
            Move::Rock => match other {
                Move::Rock => 1 + 3,
                Move::Scissor => 1 + 6,
                _ => 1,
            },
            Move::Paper => match other {
                Move::Paper => 2 + 3,
                Move::Rock => 2 + 6,
                _ => 2,
            },
            Move::Scissor => match other {
                Move::Scissor => 3 + 3,
                Move::Paper => 3 + 6,
                _ => 3,
            },
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut points = 0;

    for row in input.split("\n") {
        if row.is_empty() {
            break;
        }

        let mut parts = row.split_whitespace();
        let opp = Move::from_str(parts.next().unwrap());
        let plan = parts.next().unwrap();

        let me: Move = match plan {
            "X" => match opp {
                Move::Scissor => Move::from_str("B"),
                Move::Rock => Move::from_str("C"),
                Move::Paper => Move::from_str("A"),
            },
            "Y" => match opp {
                Move::Scissor => Move::from_str("C"),
                Move::Rock => Move::from_str("A"),
                Move::Paper => Move::from_str("B"),
            },
            _ => match opp {
                Move::Scissor => Move::from_str("A"),
                Move::Rock => Move::from_str("B"),
                Move::Paper => Move::from_str("C"),
            },
        };

        points += me.score(&opp);
    }

    println!("{points}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_won() {
        let me = Move::from_str("X");
        let opp = Move::from_str("C");
        assert_eq!(me.score(&opp), 7);
    }

    #[test]
    fn test_loss() {
        let me = Move::from_str("Z");
        let opp = Move::from_str("A");
        assert_eq!(me.score(&opp), 3);
    }

    #[test]
    fn test_draw() {
        let me = Move::from_str("Z");
        let opp = Move::from_str("Z");
        assert_eq!(me.score(&opp), 6);
    }
}
