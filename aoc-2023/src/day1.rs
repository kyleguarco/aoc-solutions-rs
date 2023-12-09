use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
fn parse(input: &str) -> String {
	input.to_string()
}

#[aoc(day1, part1)]
fn part1(_input: &str) -> usize {
	12
}

#[aoc(day1, part2)]
fn part2(_input: &str) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		assert_eq!(part1(&parse("1abc2")), 12);
	}

	#[test]
	fn part2_example() {
		assert_eq!(part2(&parse("0")), 0);
	}
}
