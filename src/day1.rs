use aoc_runner_derive::aoc;

fn calibrate<F>(mut find_digit: F) -> impl FnMut(&str) -> Option<u32>
where
	F: FnMut(&mut std::str::Chars) -> Option<u32>,
{
	move |line: &str| {
		let mut line_iter = line.chars();

		// We know from the problem statement that the number is only two digits.
		let tens = find_digit(&mut line_iter);
		let mut ones = None;
		// Keep going until there aren't any more digits on the line.
		while let Some(digit) = find_digit(&mut line_iter) {
			ones = Some(digit);
		}

		// If there was only one digit in the string, replace `tens` with `ones``.
		ones = ones.or(tens);
		tens.zip(ones).map(|(tens, ones)| tens * 10 + ones)
	}
}

fn sum_lines(input: &str, f: impl FnMut(&mut std::str::Chars) -> Option<u32>) -> u32 {
	input.lines().flat_map(calibrate(f)).sum()
}

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
	sum_lines(input, |line| {
		line.find(|chr| chr.is_digit(10))
			// Safe to unwrap since we assured above that these are digits.
			.map(|chr| unsafe { chr.to_digit(10).unwrap_unchecked() })
	})
}

struct RotatingBuffer<const SIZE: usize> {
	filled: usize,
	inner: [u8; SIZE],
}

impl<const SIZE: usize> RotatingBuffer<SIZE> {
	fn new() -> Self {
		Self { filled: 0, inner: [0; SIZE] }
	}

	fn clear(&mut self) {
		self.inner.fill(0);
		self.filled = 0;
	}

	/// * Sets `0..mid` to zero
	fn stamp(&mut self, mid: usize) {
		assert!(mid <= SIZE);
		// SAFETY: The above assert! guarantees that the slice has a valid range.
		let sli = unsafe { self.inner.get_mut(0..mid).unwrap_unchecked() };
		sli.fill(0);
		self.filled = SIZE - sli.len();
	}

	fn rotate_left_and_set(&mut self, val: u8) {
		self.inner.rotate_left(1);

		// SAFETY: The generic size constraint ensures that this is the end of the buffer.
		let end = unsafe { self.inner.get_mut(SIZE - 1).unwrap_unchecked() };
		*end = val;

		self.filled += 1;
		if self.filled >= SIZE {
			self.filled = SIZE;
		}
	}

	fn get_slice(&self) -> &[u8] {
		// SAFETY: `self.filled` is guaranteed to never be larger than SIZE by `rotate_left_and_set`
		unsafe {
			&self
				.inner
				.get((SIZE - self.filled)..SIZE)
				.unwrap_unchecked()
		}
	}
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
	const NUMSTR: [&'static str; 9] = [
		"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	];

	// Between "one" thru "nine", five is the max length of any string in-between.
	let mut buf = RotatingBuffer::<5>::new();
	sum_lines(input, move |line| loop {
		let Some(next) = line.next() else {
			buf.clear();
			return None;
		};

		if next.is_digit(10) {
			buf.rotate_left_and_set(0);
			return Some(next.to_digit(10).unwrap());
		}

		let next = next as u8;

		buf.rotate_left_and_set(next);
		let bsli = std::str::from_utf8(buf.get_slice()).expect("Bad UTF8");

		let mut to_digit = None;
		for (i, s) in NUMSTR.into_iter().enumerate() {
			if bsli.contains(s) {
				// The minimum length of one of the words is three. Remove the first three
				// elements of the buffer to prevent `bsli.contains` from returning true
				// on the previously seen word.
				buf.stamp(3);
				to_digit = Some((i + 1) as u32);
				break;
			}
		}

		if to_digit.is_none() {
			continue;
		} else {
			return to_digit;
		}
	})
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
