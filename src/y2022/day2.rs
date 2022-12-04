use std::str::FromStr;

use crate::get_input;

type Score = u32;
type RoundResult = (Score, Score);

#[derive(Debug)]
enum Outcome {
	Win,
	Tie,
	Loss,
}

impl Outcome {
	fn score(&self) -> Score {
		match self {
			Self::Win => 6,
			Self::Tie => 3,
			Self::Loss => 0,
		}
	}
}

#[derive(Debug)]
enum Play {
	Rock,
	Paper,
	Scissors,
}

impl Play {
	fn score(&self) -> Score {
		match self {
			Self::Rock => 1,
			Self::Paper => 2,
			Self::Scissors => 3,
		}
	}

	fn play(&self, opp: &Self) -> Outcome {
		match (self, opp) {
			(Self::Rock, Self::Scissors)
			| (Self::Paper, Self::Rock)
			| (Self::Scissors, Self::Paper) => Outcome::Win,
			(Self::Rock, Self::Rock)
			| (Self::Paper, Self::Paper)
			| (Self::Scissors, Self::Scissors) => Outcome::Tie,
			(Self::Rock, Self::Paper)
			| (Self::Paper, Self::Scissors)
			| (Self::Scissors, Self::Rock) => Outcome::Loss,
		}
	}
}

impl FromStr for Play {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			// The second matches are from part 1
			"A" | "X" => Ok(Self::Rock),
			"B" | "Y" => Ok(Self::Paper),
			"C" | "Z" => Ok(Self::Scissors),
			_ => Err(()),
		}
	}
}

#[derive(Debug)]
struct Round {
	opponent: Play,
	you: Play,
}

impl Round {
	fn new(opponent: Play, you: Play) -> Self {
		Self { opponent, you }
	}

	fn tally(self) -> RoundResult {
		(
			self.opponent.score() + self.opponent.play(&self.you).score(),
			self.you.score() + self.you.play(&self.opponent).score(),
		)
	}
}

impl FromStr for Round {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// For part 1
		let mut plays = s.split(" ").map(|s| s.parse::<Play>());
		let opponent = plays.next().transpose()?.unwrap();
		let you = plays.next().transpose()?.unwrap();

		if plays.next().is_some() {
			return Err(());
		}

		Ok(Self::new(opponent, you))
	}
}

#[test]
fn part1() {
	let res = get_input("day_2.txt")
		.lines()
		.map(|s| s.parse::<Round>().unwrap().tally())
		.fold((0, 0), |(o, y), (a, b)| (o + a, y + b));
	println!("Opponent: {}, You: {}", res.0, res.1);
}

impl FromStr for Outcome {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// For part 2
		match s {
			"X" => Ok(Self::Loss),
			"Y" => Ok(Self::Tie),
			"Z" => Ok(Self::Win),
			_ => Err(()),
		}
	}
}

// For part 2
#[derive(Debug)]
struct Strategy {
	opponent: Play,
	outcome: Outcome,
}

impl Strategy {
	fn new(opponent: Play, outcome: Outcome) -> Self {
		Self { opponent, outcome }
	}
}

impl FromStr for Strategy {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut plays = s.split(" ");
		let opponent = plays.next().ok_or(())?.parse::<Play>()?;
		let outcome = plays.next().ok_or(())?.parse::<Outcome>()?;

		if plays.next().is_some() {
			return Err(());
		}

		Ok(Self::new(opponent, outcome))
	}
}

impl From<Strategy> for Round {
	fn from(strat: Strategy) -> Self {
		let you = match (&strat.opponent, &strat.outcome) {
			(Play::Rock, Outcome::Tie)
			| (Play::Paper, Outcome::Loss)
			| (Play::Scissors, Outcome::Win) => Play::Rock,
			(Play::Rock, Outcome::Win)
			| (Play::Paper, Outcome::Tie)
			| (Play::Scissors, Outcome::Loss) => Play::Paper,
			(Play::Rock, Outcome::Loss)
			| (Play::Paper, Outcome::Win)
			| (Play::Scissors, Outcome::Tie) => Play::Scissors,
		};

		Self::new(strat.opponent, you)
	}
}

#[test]
fn part2() {
	let res = get_input("day_2.txt")
		.lines()
		.map(|s| Round::from(s.parse::<Strategy>().unwrap()).tally())
		.fold((0, 0), |(o, y), (a, b)| (o + a, y + b));
	println!("Opponent: {}, You: {}", res.0, res.1);
}
