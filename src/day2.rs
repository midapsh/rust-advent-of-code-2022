const _DUMMY_INPUT: &str = include_str!("data/day2-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day2-real.txt");

#[derive(PartialEq, Clone, Copy, Debug)]
enum Jokenpo {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

// TODO(hspadim): Try to use C Char
impl From<char> for Jokenpo {
    fn from(jokenpo: char) -> Self {
        match jokenpo {
            // Opponent
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            // Player
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            // Invalid
            _ => Self::Invalid,
        }
    }
}

impl From<&str> for Jokenpo {
    fn from(jokenpo: &str) -> Self {
        match jokenpo {
            // Opponent
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            // Player
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            // Invalid
            _ => Self::Invalid,
        }
    }
}

impl From<i32> for Jokenpo {
    fn from(jokenpo: i32) -> Self {
        match jokenpo {
            // From Modular Arithmetic
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            // Invalid
            _ => Self::Invalid,
        }
    }
}

impl Jokenpo {
    const fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
            Self::Invalid => 0,
        }
    }
}

enum MatchResult {
    Win,  // 6
    Draw, // 3
    Lose, // 0
    Invalid,
}

impl From<&str> for MatchResult {
    fn from(match_result: &str) -> Self {
        match match_result {
            // Player
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            // Invalid
            _ => Self::Invalid,
        }
    }
}

impl MatchResult {
    const fn value(&self) -> i32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
            Self::Invalid => 0,
        }
    }
}

fn eval_match_result(jokenpo_player: Jokenpo, jokenpo_opponent: Jokenpo) -> i32 {
    if jokenpo_player == jokenpo_opponent {
        MatchResult::Draw.value() + jokenpo_player.value() // Draw
    } else if (jokenpo_player.value() % 3) == ((jokenpo_opponent.value() + 1) % 3) {
        MatchResult::Win.value() + jokenpo_player.value() // Win (player won against the opponent)
    } else {
        MatchResult::Lose.value() + jokenpo_player.value() // Lose (player lost against the opponent)
    }
}

fn eval_opponent_hand(jokenpo_opponent: Jokenpo, match_result: MatchResult) -> Jokenpo {
    match match_result {
        MatchResult::Draw => jokenpo_opponent,
        MatchResult::Win => Jokenpo::from(jokenpo_opponent.value() % 3),
        MatchResult::Lose => Jokenpo::from((jokenpo_opponent.value() + 1) % 3),
        MatchResult::Invalid => Jokenpo::Invalid,
    }
}

fn private_solve_part_1(values: &str) -> String {
    values
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let jokenpo_opponent = Jokenpo::from(iter.next().unwrap());
            let jokenpo_player = Jokenpo::from(iter.next().unwrap());
            eval_match_result(jokenpo_player, jokenpo_opponent)
        })
        .sum::<i32>()
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    values
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let jokenpo_opponent = Jokenpo::from(iter.next().unwrap());
            let match_result = MatchResult::from(iter.next().unwrap());
            let jokenpo_player = eval_opponent_hand(jokenpo_opponent, match_result);
            eval_match_result(jokenpo_player, jokenpo_opponent)
        })
        .sum::<i32>()
        .to_string()
}

fn _solve_part_1_dummy() -> String {
    private_solve_part_1(_DUMMY_INPUT)
}

pub fn solve_part_1_real() -> String {
    private_solve_part_1(REAL_INPUT)
}

fn _solve_part_2_dummy() -> String {
    private_solve_part_2(_DUMMY_INPUT)
}

pub fn solve_part_2_real() -> String {
    private_solve_part_2(REAL_INPUT)
}

#[cfg(test)]
mod tests {
    use super::{
        _solve_part_1_dummy, _solve_part_2_dummy, eval_match_result, eval_opponent_hand,
        solve_part_1_real, solve_part_2_real, Jokenpo, MatchResult,
    };

    ///////////////////////
    // Common Evaluation //
    ///////////////////////

    #[test]
    fn test_logic_when_draw() {
        assert_eq!(
            eval_match_result(Jokenpo::Paper, Jokenpo::Paper),
            3i32 + Jokenpo::Paper.value()
        );
        assert_eq!(
            eval_match_result(Jokenpo::Rock, Jokenpo::Rock),
            3i32 + Jokenpo::Rock.value()
        );
        assert_eq!(
            eval_match_result(Jokenpo::Scissors, Jokenpo::Scissors),
            3i32 + Jokenpo::Scissors.value()
        );
    }

    #[test]
    fn test_logic_when_lose() {
        assert_eq!(
            eval_match_result(Jokenpo::Rock, Jokenpo::Paper),
            0i32 + Jokenpo::Rock.value()
        );
        assert_eq!(
            eval_match_result(Jokenpo::Paper, Jokenpo::Scissors),
            0i32 + Jokenpo::Paper.value()
        );
        assert_eq!(
            eval_match_result(Jokenpo::Scissors, Jokenpo::Rock),
            0i32 + Jokenpo::Scissors.value()
        );
    }

    #[test]
    fn test_logic_when_win() {
        assert_eq!(
            eval_match_result(Jokenpo::Rock, Jokenpo::Scissors),
            6i32 + Jokenpo::Rock.value()
        );
        assert_eq!(
            eval_match_result(Jokenpo::Paper, Jokenpo::Rock),
            6i32 + Jokenpo::Paper.value()
        );
        assert_eq!(
            eval_match_result(Jokenpo::Scissors, Jokenpo::Paper),
            6i32 + Jokenpo::Scissors.value()
        );
    }

    //////////////////////
    // Match Evaluation //
    //////////////////////

    #[test]
    fn test_opponent_hand_logic_when_draw() {
        assert_eq!(
            eval_opponent_hand(Jokenpo::Paper, MatchResult::Draw),
            Jokenpo::Paper
        );
        assert_eq!(
            eval_opponent_hand(Jokenpo::Rock, MatchResult::Draw),
            Jokenpo::Rock
        );
        assert_eq!(
            eval_opponent_hand(Jokenpo::Scissors, MatchResult::Draw),
            Jokenpo::Scissors
        );
    }

    #[test]
    fn test_opponent_hand_logic_when_lose() {
        assert_eq!(
            eval_opponent_hand(Jokenpo::Paper, MatchResult::Lose),
            Jokenpo::Rock
        );
        assert_eq!(
            eval_opponent_hand(Jokenpo::Rock, MatchResult::Lose),
            Jokenpo::Scissors
        );
        assert_eq!(
            eval_opponent_hand(Jokenpo::Scissors, MatchResult::Lose),
            Jokenpo::Paper
        );
    }

    #[test]
    fn test_opponent_hand_logic_when_win() {
        assert_eq!(
            eval_opponent_hand(Jokenpo::Paper, MatchResult::Win),
            Jokenpo::Scissors
        );
        assert_eq!(
            eval_opponent_hand(Jokenpo::Rock, MatchResult::Win),
            Jokenpo::Paper
        );
        assert_eq!(
            eval_opponent_hand(Jokenpo::Scissors, MatchResult::Win),
            Jokenpo::Rock
        );
    }

    // Day 2 results

    #[test]
    fn test_part_1_dummy() {
        assert_eq!("15", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("12", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real()); // 12855
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
