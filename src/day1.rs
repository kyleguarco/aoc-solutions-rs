use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
	fn calibrate(line: &str) -> Option<u32> {
		let mut line = line.chars();
		let mut find_digit = || line.find(|c| c.is_digit(10));

		// We know from the problem statement that the number is only two digits.
		let first_digit = find_digit();
		let mut last_digit = None;

		// Keep going until there aren't any more digits on the line.
		while let Some(d) = find_digit() {
			last_digit = Some(d);
		}

		// If there was only one digit in the string, replace `last_digit` with the first one.
		last_digit = last_digit.or(first_digit);

		first_digit.zip(last_digit).map(|(f, l)| {
			unsafe {
				// lmao imagine shifting characters together as a byte slice?
				// Safe to unwrap since we assured above that these are digits.
				std::str::from_utf8(&[f as u8, l as u8])
					.unwrap_unchecked()
					.parse()
					.unwrap_unchecked()
			}
		})
	}

	input.lines().flat_map(calibrate).sum()
}

#[aoc(day1, part2)]
fn part2(_input: &str) -> u32 {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
		assert_eq!(part1(input), 142);
	}

	#[test]
	fn part2_example() {
		let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
		assert_eq!(part2(input), 281);
	}
}
