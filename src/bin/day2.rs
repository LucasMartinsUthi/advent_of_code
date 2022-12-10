// A = Rock
// B = Paper
// C = Scissors

// X = Rock
// Y = Paper
// Z = Scissors

// Rock = 1
// Paper = 2
// Scissors = 3
struct HandPair {
    p1: String,
    p2: String,
}

impl HandPair {
    fn new(p1: &str, p2: &str) -> HandPair {
        HandPair { p1: p1.to_string(), p2: p2.to_string() }
    }

    pub fn strategy_one(&self) -> usize {
        let p1 = &self.p1[..];
        let p2 = &self.p2[..];

        let mut value = match (p1, p2) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,

            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,

            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            _ => 0,
        };

        let add_value = match p2 {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,    
        };

        value += add_value;

        return value
    }

    pub fn strategy_two(self) -> usize {
        let p1 = &self.p1[..];
        let p2 = &self.p2[..];

        let mut value = match (p1, p2) {
            ("A", "X") => 3,
            ("A", "Y") => 1,
            ("A", "Z") => 2,

            ("B", "X") => 1,
            ("B", "Y") => 2,
            ("B", "Z") => 3,

            ("C", "X") => 2,
            ("C", "Y") => 3,
            ("C", "Z") => 1,
            _ => 0,
        };

        let add_value = match p2 {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,    
        };

        value += add_value;

        return value
    }
}

fn main() {
    // Part 1
    let strategy_one_result = include_str!("../inputs/input2.txt")
        .lines()
        .map(|line| {
            let hand = line.split(" ").collect::<Vec<&str>>();

            return HandPair::new(hand[0], hand[1]).strategy_one();
        })
        .sum::<usize>();


    println!("Strategy result: {}", strategy_one_result);

    // Part 2
    let strategy_two_result = include_str!("../inputs/input2.txt")
        .lines()
        .map(|line| {
            let hand = line.split(" ").collect::<Vec<&str>>();

            return HandPair::new(hand[0], hand[1]).strategy_two();
        })
        .sum::<usize>();

    println!("Strategy 2 result: {}", strategy_two_result)
}


//TEST 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        // A Y => 8
        // B X => 1
        // C Z => 6

        let hand_pair = HandPair::new("A", "Y");
        assert_eq!(hand_pair.strategy_one(), 8);

        let hand_pair = HandPair::new("B", "X");
        assert_eq!(hand_pair.strategy_one(), 1);

        let hand_pair = HandPair::new("C", "Z");
        assert_eq!(hand_pair.strategy_one(), 6);
    }

    fn test_two() {
        // A Y => 4
        // B X => 1
        // C Z => 7

        let hand_pair = HandPair::new("A", "Y");
        assert_eq!(hand_pair.strategy_two(), 4);

        let hand_pair = HandPair::new("B", "X");
        assert_eq!(hand_pair.strategy_two(), 1);

        let hand_pair = HandPair::new("C", "Z");
        assert_eq!(hand_pair.strategy_two(), 7);
    }
}