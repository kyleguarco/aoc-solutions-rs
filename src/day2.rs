use aoc_runner_derive::{aoc, aoc_generator};

type GameIDType = usize;
type CubeUType = u8;

struct Set {
	red: CubeUType,
	green: CubeUType,
	blue: CubeUType,
}

struct Game {
	id: GameIDType,
	perms: Vec<Set>,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
	todo!()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> usize {
	todo!()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> usize {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
		assert_eq!(part1(&parse(input)), 8);
	}

	#[test]
	fn part2_example() {
		assert_eq!(part2(&parse("<EXAMPLE>")), 0);
	}
}
